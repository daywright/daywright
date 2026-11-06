# Daywright backend

Rust, one Cargo workspace, axum on tokio. The brain: the timeline of
observations, memory, the planner, speech, the model and voice. Licensed
AGPL-3.0-only, except `crates/proto` (the open device protocol), which is
Apache-2.0 so any device or app can implement it.

## Crates

- `domain` core types, no I/O
- `proto` the open device protocol (Apache-2.0)
- `providers` speech to text, language model, text to speech and device-data
  connector traits, with cloud and local implementations (the swap point)
- `media` the realtime pipeline that ties the providers together
- `api` the one binary: the axum server

## Build and run

```sh
cp .env.example .env
cargo check
cargo run -p daywright-api
```

The first `cargo check` downloads the dependency tree, which is tens of
megabytes. Not built yet, so treat the code as an unbuilt skeleton until it has
been.

## State

Skeleton only. The traits and types are in place; the provider implementations,
the timeline and the database layer are marked with `TODO(M1)` and land in
milestone M1 (see `docs/roadmap.md`).
