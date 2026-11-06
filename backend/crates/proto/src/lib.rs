//! The wire protocol between the pendant, the gateway, and the backend.
//!
//! Kept deliberately small. The pendant streams captured media and events up;
//! the backend sends playback and indicator commands down. Audio payloads are
//! carried as raw bytes by the transport (binary WebSocket frames); these
//! structs describe the control messages around them.

use serde::{Deserialize, Serialize};

/// A message from the pendant (via the gateway) to the backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Upstream {
    /// A run of captured audio. `seq` orders the stream; the samples travel as
    /// the accompanying binary frame, not inline here.
    AudioChunk { seq: u64, samples_len: usize },
    /// A single JPEG frame captured because something happened (speech, a tap,
    /// food in view). The bytes travel as the accompanying binary frame.
    VideoFrame { seq: u64, jpeg_len: usize },
    /// A physical event on the pendant.
    Event { event: DeviceEvent },
    /// Periodic liveness and battery report.
    Heartbeat { battery_pct: u8 },
}

/// A physical interaction or state change on the pendant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceEvent {
    /// Address Daywright directly. Replaces a wake word.
    TapSingle,
    /// Mute or unmute proactive interjections.
    TapDouble,
    /// Save the current rolling buffer (see docs/privacy-and-recording.md).
    TapHold,
    LowBattery,
}

/// A message from the backend down to the pendant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Downstream {
    /// Play audio to the given route. The samples travel as the accompanying
    /// binary frame.
    PlayAudio { route: AudioRoute, samples_len: usize },
    /// Set the status LED, including the honest capture indicator.
    SetLed { pattern: LedPattern },
    /// Stop any current playback.
    StopAudio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioRoute {
    /// The wearer's earpiece, via the phone.
    Earpiece,
    /// A loudspeaker, for speak-for-me.
    Loudspeaker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LedPattern {
    Off,
    /// Steady: capturing. The indicator is always honest.
    Capturing,
    /// Pulse: thinking or speaking.
    Active,
    /// Muted proactive interjections.
    Muted,
}
