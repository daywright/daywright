# Daywright

One brain you talk to, and a family of devices that see, count and tell you what
to do next, so you never have to hold a phone or think about it. Its first job is
health: getting busy people who were once in shape back in shape.

The name: a wright is a maker, as in shipwright or playwright, and Daywright
reads as "day right". It helps make each day right, and it is built in the open
by its community of makers, its wrights: people who write the software and design
the hardware together.

## What it does

- Reads the data you already produce (phone, watch, scale, kitchen) and plans your
  day: what to eat, when and how to train, when to sleep.
- Gives short, plain orders on whatever device is near you: "Three sets of five
  RDLs at 18:30."
- Remembers what you tell it once: "I only have Fridays to work out." "Keep the
  food simple and the same each day."
- Logs food by voice and weight in the kitchen: say "one big tomato" and it is done.
- Shows progress with photos and numbers, then adjusts the plan.
- Later, beyond health: quiet advice when a price is wrong or a commitment is
  risky, live translation, and the rest of daily life.

## The suite

| Device | Senses | Tells | When |
|--------|--------|-------|------|
| Daywright Kitchen, a voice scale | weight, voice | speaker, small screen | first own device (M2) |
| Daywright Board, e-ink | | today's plan, on the fridge or desk | TRMNL-compatible in M1, own board later |
| Daywright Pendant | voice, camera | the earpiece | after the kitchen device (M5) |
| Phone, in your pocket | Apple Health, Health Connect, Bluetooth courier | notifications, only when asked | owner's Android in M1; iPhone and Android in M4 |
| Watches and smart scales | through Apple Health and Health Connect | watch alerts | integrations |

## How you interact with it

- **Talk to it.** In the kitchen a wake word, detected on the device itself, means
  you never touch a button. On the pendant a tap means "this is for you", so you
  are never talking to your necklace in public.
- **It tells you on the device that fits the moment**, and keeps it short.
- **Everything it knows about you is one list** you can read and change by voice.

## Repository map

| Path | What | License |
|------|------|---------|
| `backend/` | Rust (Cargo workspace, axum). The brain: timeline, memory, planner, speech, model, voice, API. | AGPL-3.0-only |
| `backend/crates/proto/` | The open device protocol. | Apache-2.0 |
| `apps/android/`, `apps/ios/` | The phone apps: courier, health data, an optional screen. | Apache-2.0 |
| `devices/` | Firmware and hardware for each device. | Apache-2.0 firmware, CERN-OHL-S-2.0 hardware |
| `docs/` | Product, architecture, decisions, privacy, roadmap. | CC-BY-4.0 |

## Status

Skeleton. Nothing is built or verified on a machine yet. See `docs/roadmap.md`
for the order of work.

## Start here

- `docs/product.md` for what Daywright does day to day.
- `docs/architecture.md` for the whole picture and the data flow.
- `docs/decisions.md` for every choice and the reasoning behind it.
- `docs/privacy-and-recording.md` before any capture code ships.

## License

Open source, split by component: see `LICENSE.md`, with the exact mapping in
`REUSE.toml`. The Daywright name and logos are not covered by these licenses (see
`TRADEMARKS.md`). Contributions are welcome under the Developer Certificate of
Origin (see `CONTRIBUTING.md`).
