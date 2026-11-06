//! The swap point. Each capability Daywright needs from the outside world is a
//! trait with more than one implementation, chosen by config, never by a code
//! change. That is how "self-host or cloud" is true from day one
//! (see docs/architecture.md).
//!
//! The cloud and local implementations are stubbed here as `TODO(M1)`. The
//! traits and a [`mock`] implementation are real so the pipeline in the `media`
//! crate can be built and tested without any network.

use async_trait::async_trait;
use daywright_domain::{Language, Urgency};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("provider request failed: {0}")]
    Request(String),
    #[error("provider is not configured: {0}")]
    NotConfigured(String),
}

pub type Result<T> = std::result::Result<T, ProviderError>;

/// Raw audio samples. The codec and sample rate are agreed by the transport;
/// providers that need something else convert internally.
pub type AudioBytes = Vec<u8>;

/// The text of something that was said, with the language it was said in.
#[derive(Debug, Clone)]
pub struct Transcript {
    pub text: String,
    pub language: Language,
}

/// What the model decided to do with a moment.
#[derive(Debug, Clone)]
pub struct ModelReply {
    /// The words to say, if any.
    pub text: String,
    /// Whether this is worth saying at all right now.
    pub should_speak: bool,
    /// If it is, how much attention it demands.
    pub urgency: Urgency,
}

/// A chosen voice for synthesis.
#[derive(Debug, Clone)]
pub struct Voice {
    pub id: String,
    pub language: Language,
}

/// Turn speech into text.
#[async_trait]
pub trait SpeechToText: Send + Sync {
    /// Transcribe a run of audio. `hint` is the expected language when known,
    /// `None` to auto-detect.
    async fn transcribe(&self, audio: AudioBytes, hint: Option<Language>) -> Result<Transcript>;
}

/// Decide whether and what to say, and carry out spoken instructions.
#[async_trait]
pub trait LanguageModel: Send + Sync {
    /// `context` is the recent transcript and relevant memory; `instruction` is
    /// the standing brief plus any direct request from the wearer.
    async fn respond(&self, context: &str, instruction: &str) -> Result<ModelReply>;
}

/// Turn text into speech.
#[async_trait]
pub trait TextToSpeech: Send + Sync {
    async fn synthesize(&self, text: &str, voice: &Voice) -> Result<AudioBytes>;
}

/// Which implementation of each capability to use. Read from config.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Backend {
    Cloud,
    Local,
}

pub mod cloud {
    //! TODO(M1): ElevenLabs Scribe v2 Realtime or Soniox for speech-to-text,
    //! Claude over HTTPS (server-sent events) for the model, ElevenLabs for
    //! voice. There is no official Anthropic Rust SDK, so the model client is
    //! plain HTTPS.
}

pub mod local {
    //! TODO(M1): whisper.cpp through `whisper-rs` for speech-to-text, Ollama
    //! over HTTP for the model, Piper for voice.
}

pub mod mock {
    //! A no-network implementation so the pipeline can be built and tested now.

    use super::*;

    pub struct MockStt;

    #[async_trait]
    impl SpeechToText for MockStt {
        async fn transcribe(
            &self,
            _audio: AudioBytes,
            hint: Option<Language>,
        ) -> Result<Transcript> {
            Ok(Transcript {
                text: "(mock transcript)".to_string(),
                language: hint.unwrap_or(Language("en".to_string())),
            })
        }
    }

    pub struct MockLlm;

    #[async_trait]
    impl LanguageModel for MockLlm {
        async fn respond(&self, _context: &str, _instruction: &str) -> Result<ModelReply> {
            Ok(ModelReply {
                text: "(mock reply)".to_string(),
                should_speak: false,
                urgency: Urgency::Normal,
            })
        }
    }

    pub struct MockTts;

    #[async_trait]
    impl TextToSpeech for MockTts {
        async fn synthesize(&self, _text: &str, _voice: &Voice) -> Result<AudioBytes> {
            Ok(Vec::new())
        }
    }
}
