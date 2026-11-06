//! Core domain types for Daywright. No I/O, no framework, no provider code.
//! These are the words the rest of the system is built from.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// How much a given moment demands the wearer's attention.
///
/// Daywright interjects on its own only for [`Urgency::Critical`]. Everything else
/// waits for the wearer to ask (a tap) or for a natural pause.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Urgency {
    /// Money, commitments, health, safety. Worth a brief, unprompted word.
    Critical,
    /// Useful but not worth interrupting for. Surfaced when asked.
    Normal,
}

/// What the assistant is doing right now. Entered by voice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    /// The default: quiet guidance in the wearer's ear.
    Advice,
    /// Continuous live translation of the conversation.
    Translate,
    /// Teaching the wearer a language.
    Tutor,
    /// Speaking aloud on the wearer's behalf, for the other person to hear.
    SpeakFor,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Advice
    }
}

/// Where spoken output goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputRoute {
    /// The wearer's hidden earpiece. Private. The default.
    Earpiece,
    /// A loudspeaker (the phone in v1), so others hear it. Only for `SpeakFor`.
    Loudspeaker,
}

impl Default for OutputRoute {
    fn default() -> Self {
        OutputRoute::Earpiece
    }
}

/// A BCP-47 language tag, for example `en`, `es`, `de`, `fr`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Language(pub String);

/// The person wearing a Daywright pendant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub display_name: String,
    /// Languages the wearer understands, most fluent first. Used to pick the
    /// language of guidance and to drive translation.
    pub languages: Vec<Language>,
}

/// A pendant registered to a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: Uuid,
    pub user_id: Uuid,
    pub label: String,
}

/// Something Daywright has learned about the wearer and should keep in mind.
///
/// This is how standing preferences exist without a rules screen: they are
/// inferred from how the wearer talks to it and stored as plain text the wearer
/// can read and remove.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryItem {
    pub id: Uuid,
    pub user_id: Uuid,
    /// Plain-language fact or preference, for example
    /// "warn me if a price is above the going rate".
    pub text: String,
    /// Where it came from, for the wearer's trust and for review.
    pub source: MemorySource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemorySource {
    /// The wearer said it directly.
    Stated,
    /// Daywright inferred it from repeated behaviour.
    Inferred,
}

/// One logged meal or item, for the calorie feature. The derived numbers are
/// kept, not the photo, unless the wearer saves the photo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub description: String,
    /// Estimates are ranges, not false precision.
    pub kcal_low: u32,
    pub kcal_high: u32,
}
