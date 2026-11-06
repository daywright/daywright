//! The realtime pipeline: take a run of captured audio, decide whether and what
//! to say, and produce speech to play back. Turn-taking and barge-in will live
//! here. This is the orchestration we own rather than get from a framework
//! (see ADR-0009 in docs/decisions.md).

use daywright_domain::{Language, Mode, OutputRoute, Urgency};
use daywright_providers::{
    AudioBytes, LanguageModel, ProviderError, SpeechToText, TextToSpeech, Voice,
};

/// The providers the pipeline drives, already chosen from config.
pub struct Pipeline {
    stt: Box<dyn SpeechToText>,
    llm: Box<dyn LanguageModel>,
    tts: Box<dyn TextToSpeech>,
    voice: Voice,
}

/// What came out of handling one run of audio.
pub struct Outcome {
    /// The words Daywright decided on, for logging and the app transcript.
    pub text: String,
    /// Speech to play, present only when Daywright judged it worth speaking.
    pub audio: Option<AudioBytes>,
    pub urgency: Urgency,
    pub route: OutputRoute,
}

impl Pipeline {
    pub fn new(
        stt: Box<dyn SpeechToText>,
        llm: Box<dyn LanguageModel>,
        tts: Box<dyn TextToSpeech>,
        voice: Voice,
    ) -> Self {
        Self {
            stt,
            llm,
            tts,
            voice,
        }
    }

    /// Handle one run of captured audio.
    ///
    /// `instruction` is the standing brief plus any direct request. `mode`
    /// decides routing: everything is private to the earpiece except
    /// speak-for-me, which goes to a loudspeaker.
    ///
    /// TODO(M1): the cheap gate model should run before the main model so the
    /// expensive one only wakes when there is something worth saying
    /// (see docs/architecture.md). For now the single `llm` stands in for both.
    pub async fn handle(
        &self,
        audio: AudioBytes,
        language_hint: Option<Language>,
        instruction: &str,
        mode: Mode,
    ) -> Result<Outcome, ProviderError> {
        let transcript = self.stt.transcribe(audio, language_hint).await?;
        // Never log transcript content: it is the speech of whoever is around
        // the wearer (see docs/privacy-and-recording.md). Metadata only.
        tracing::debug!(
            chars = transcript.text.len(),
            language = %transcript.language.0,
            "transcribed"
        );

        let reply = self.llm.respond(&transcript.text, instruction).await?;

        let route = match mode {
            Mode::SpeakFor => OutputRoute::Loudspeaker,
            _ => OutputRoute::Earpiece,
        };

        let audio = if reply.should_speak {
            Some(self.tts.synthesize(&reply.text, &self.voice).await?)
        } else {
            None
        };

        Ok(Outcome {
            text: reply.text,
            audio,
            urgency: reply.urgency,
            route,
        })
    }
}
