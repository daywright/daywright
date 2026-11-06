//! Configuration read from the environment. See `backend/.env.example`.

use daywright_providers::Backend;

pub struct Config {
    pub port: u16,
    pub stt: Backend,
    pub llm: Backend,
    pub tts: Backend,
    pub gate_model: String,
    pub main_model: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            port: env_u16("DAYWRIGHT_PORT", 3000),
            stt: env_backend("DAYWRIGHT_STT"),
            llm: env_backend("DAYWRIGHT_LLM"),
            tts: env_backend("DAYWRIGHT_TTS"),
            gate_model: env_string("DAYWRIGHT_LLM_GATE_MODEL", "claude-haiku-5-5"),
            main_model: env_string("DAYWRIGHT_LLM_MAIN_MODEL", "claude-opus-5-5"),
        }
    }
}

fn env_string(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_u16(key: &str, default: u16) -> u16 {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/// Defaults to cloud; `local` selects the self-hosted implementation.
fn env_backend(key: &str) -> Backend {
    match std::env::var(key).as_deref() {
        Ok("local") => Backend::Local,
        _ => Backend::Cloud,
    }
}
