# Roadmap

Order of work. Each milestone is something the owner can actually use or test,
not a layer with no payoff. The owner is user number one at every step.

## M0: Skeleton (done)

Monorepo, crate layout, app targets, firmware target, docs, licenses.

## M1: The brain, for the owner

- Build the Cargo workspace (`cargo check`, then `cargo run -p daywright-api`).
- The timeline of observations (ADR-0016) in Postgres, and the memory list of
  rules and preferences (ADR-0015).
- The planner: today's meals, training and steps from the timeline and the rules;
  a weekly review that explains what changed.
- One cloud provider per capability behind the traits: speech to text, Claude over
  HTTPS, text to speech. Pick the speech provider on English first, then check
  the next major languages.
- The owner's Android courier: reads Health Connect (the smart scale, steps, sleep)
  from the phone in the pocket and relays it to the timeline.
- An e-ink board showing today's plan, served to a TRMNL display in
  bring-your-own-server mode.
- A test harness that feeds a recorded clip in and gets spoken guidance out, so
  the voice pipeline is provable without any device.

## M2: The kitchen voice scale

- Prototype on modules: ESP32-S3, load cell and ADC, microphones with an audio
  front end, speaker, small display (see `devices/kitchen/README.md`).
- Wake word on the device; "one big tomato" end to end: heard, weighed, matched in
  USDA FoodData Central, logged, confirmed out loud in under ten words.
- A 3D-printed enclosure with a wipeable top.

## M3: Open release

- Publish the repository: `CONTRIBUTING.md`, DCO checks, issue templates, a
  Discord.
- A Home Assistant integration and a TRMNL plugin, so both communities can try
  Daywright with hardware they already own.
- Self-hosting guide: one command, local providers documented.

## M4: Daywright Cloud and the first paying users

- The hosted brain with accounts and billing, in the private Daywright Cloud
  repository, talking to this one through the public API.
- iPhone and Android apps reach users together (ADR-0012). The shared-logic
  decision (Kotlin Multiplatform or two codebases) is made at the start of this
  milestone.
- Payments set up the way the private payments research recommends.

## M5: The pendant

- Firmware on the XIAO ESP32-S3 Sense: mic capture, on-device voice activity
  detection, tap sensor, status LED, stream to the courier.
- Camera frames on event: food in view becomes an observation without a photo
  taken by hand.
- A 3D-printed first enclosure.
- The pendant's own jobs: catch an overcharge in the moment, live translation.

## M6: Measurement devices and polish

- A Bluetooth waist tape and grip strength as observations.
- Our own body scale on a weight and body-composition front end such as TI's
  AFE4300, validated against DEXA before any accuracy claim.
- Our own e-ink board.
- Local provider implementations proven (whisper.cpp, Ollama, Piper) for the fully
  self-hosted path.
- Custom PCBs in KiCad.
- Cellular add-on board evaluated against the battery and heat budget.

## Parking lot

- Pendant loudspeaker for speak-for-me without the phone.
- Hidden induction earpiece loop in the necklace.
- Calibration against occasional DEXA scans so cheap daily measurements learn
  each person's offset.
