//! The Daywright backend binary: the axum server.
//!
//! Skeleton. It starts, logs its config, and answers health and ping. The
//! streaming session endpoint, auth, memory, and food log land in M1 and M2
//! (see docs/roadmap.md). The provider wiring below proves the pieces compose.

mod config;

use axum::{routing::get, Json, Router};
use config::Config;
use daywright_domain::Language;
use daywright_media::Pipeline;
use daywright_providers::{mock, Voice};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env();
    tracing::info!(
        port = config.port,
        gate_model = %config.gate_model,
        main_model = %config.main_model,
        "starting daywright-api"
    );

    // Prove the pipeline composes from chosen providers. M1 swaps the mocks for
    // the cloud or local implementations named by `config`.
    let _pipeline = Pipeline::new(
        Box::new(mock::MockStt),
        Box::new(mock::MockLlm),
        Box::new(mock::MockTts),
        Voice {
            id: "default".to_string(),
            language: Language("en".to_string()),
        },
    );

    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/ping", get(ping));

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "listening");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> &'static str {
    "ok"
}

async fn ping() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "service": "daywright-api", "status": "alive" }))
}
