# Architecture

## The shape

One brain, many devices. Each part can change without forcing the others to
change.

```
  SENSE                                  THE BRAIN (backend, Rust)              TELL
  kitchen voice scale --- Wi-Fi --->  +---------------------------------+  ---> kitchen speaker
  pendant --- BLE ---> phone ------>  | timeline of observations        |  ---> earpiece (via phone)
  watch, scale --> Apple Health /     | memory: rules and preferences   |  ---> e-ink board (it pulls)
       Health Connect --> phone --->  | planner: today, the week        |  ---> watch, phone (when asked)
  calendar, photos ---------------->  | tell router                     |
                                      | providers: speech, model, voice |
                                      +---------------------------------+
                                         runs self-hosted or in Daywright Cloud
```

- **Devices sense, tell, or both** (ADR-0013). They speak one open protocol,
  `backend/crates/proto` (Apache-2.0), so the community can build compatible
  devices.
- **The phone is a courier, not a screen you hold.** It relays Bluetooth devices
  (the pendant, the earpiece) and reads health data that only exists on the phone
  (Apple Health, Health Connect). Nothing in the devices or the backend assumes the
  courier is a phone: a cellular add-on board can replace it later (ADR-0005).
- **The backend is the brain.** It keeps the timeline, the memory and the plan,
  decides what to say and where, and turns speech to text and text back to speech.

## The data: one timeline of observations

Every input is stored as an observation (ADR-0016):

| Field | Example |
|-------|---------|
| kind | `body.weight`, `food.item`, `sleep.session`, `activity.steps`, `training.set`, `calendar.busy` |
| value and unit | `82.4 kg`, `182 g`, `33 kcal` |
| time | when it happened, not when it arrived |
| source and device | `kitchen-scale-1`, `health-connect`, `voice` |
| confidence | weighed beats guessed, measured beats estimated |

Plans, rules, devices and people are ordinary records next to the timeline. This
is a plain table that is almost only appended to, not event sourcing. A new part
of life adds kinds and sources, never new storage.

## The brain

- **Memory** (ADR-0015): one list of rules (hard constraints) and preferences
  (soft ones), each with its origin and date. Said once by voice, or inferred and
  marked as inferred until confirmed. Readable on any screen, editable by voice.
- **Planner:** builds today's plan (meals, training, sleep, steps) from the
  timeline, the memory and the goal, and a weekly review that explains what
  changed and why. Rules always win; when a rule slows the goal, it says so once.
- **Tell router:** picks the device for each message: the kitchen speaker while
  cooking, the e-ink board in the morning, the earpiece outside, the watch at the
  gym, the phone screen only when asked.
- **Guardrails** (ADR-0017): warning signs stop the plan and point to a doctor; no
  diagnosis, no drug coaching.

## Backend (Rust, Cargo workspace)

Framework is **axum** on tokio, structured in layers, not ad hoc. Crates:

| Crate | Responsibility | License |
|-------|----------------|---------|
| `domain` | Core types with no I/O: person, device, observation, rule, plan, mode, urgency. | AGPL-3.0-only |
| `proto` | The open device protocol between devices, the courier and the backend. | Apache-2.0 |
| `providers` | Traits for speech to text, language model, text to speech and device-data connectors, with cloud and local implementations. The swap point. | AGPL-3.0-only |
| `media` | The realtime voice pipeline: gate, transcribe, decide, speak. Turn-taking and barge-in live here. | AGPL-3.0-only |
| `api` | The axum server: auth, device registry, timeline, memory, planner, the streaming endpoint, and the e-ink screen endpoint. The one binary. | AGPL-3.0-only |

The timeline, memory and planner start inside `domain` and `api`, and get their
own crates only when they earn them.

### Self-host or cloud, from the start

Each capability is a trait with at least two implementations, chosen by config,
never by a code change (ADR-0004):

- **Speech to text:** cloud (ElevenLabs Scribe v2 Realtime, or Soniox) or local
  (whisper.cpp through `whisper-rs`).
- **Language model:** cloud (Claude over HTTPS) or local (Ollama).
- **Text to speech:** cloud (ElevenLabs) or local (Piper).
- **Device data:** direct connectors by default; a cloud aggregator such as Terra
  as an option.

There is no official Anthropic Rust SDK, so model calls are plain HTTPS with
streaming over server-sent events. That is the sanctioned path when a language
has no SDK, and it is a small, stable surface.

### Keeping an always-on assistant affordable

Proactivity is tiered so the expensive model is not reasoning every second:

1. Devices drop silence before anything leaves them: voice-activity detection on
   the pendant, a wake word on the kitchen device.
2. A cheap gate model (`claude-haiku-5-5`) answers one question continuously: is
   anything here worth saying or doing?
3. Only when the gate says yes does the strong model (`claude-opus-5-5`) compose
   the guidance, write the plan or carry out the instruction.

## Interaction model

- **Addressing it:** a wake word on stationary home devices (ADR-0014); a single
  tap on the pendant (ADR-0007), so there is no talking to a necklace in public.
- **Proactive interjection:** bounded to health, money, commitments and safety.
  It waits for a pause, stays under about ten words, and can be muted.
- **Modes**, all entered by voice: coach (default), advice, live translation,
  language tutor, and speak-for-me.

## Devices

See `devices/README.md`. Kitchen voice scale and e-ink board first, the pendant
after, measurement devices later.

## Open core and the paid layer

The brain in this repository is complete and self-hostable (ADR-0011). Daywright
Cloud adds hosting, accounts, billing and premium programs. It lives in a private
repository and talks to the brain only through the public API, so nothing in this
repository depends on it.

## What the backend does not do

It does not store raw audio or video by default, it does not keep body photos
unless the person chooses to, and it does not assume a network it does not need.
See `docs/privacy-and-recording.md`.
