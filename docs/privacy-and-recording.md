# Recording and privacy

Daywright captures the world around the person wearing it, which means it also
captures other people. That is the part of this project with real legal and
ethical weight, so the defaults are set here, before any capture code ships, and
the code must follow them rather than the other way round.

## Open question to settle before defaults are final

The law on recording other people differs by country. For every country where
Daywright devices are used or sold, confirm whether recording a conversation the
wearer is part of is allowed, and whether recording others who are not party to
it is allowed, before the defaults below are treated as settled there. Until
then, assume the stricter reading.

## Defaults

- **Nothing raw is kept by default.** Audio is processed in a rolling short
  buffer (on the order of a couple of minutes) and continuously overwritten,
  like a dashcam. Video frames are captured only on an event and are dropped
  after they are used, unless the wearer saves them.
- **Saving is a deliberate act.** A gesture saves the current buffer, which is
  what makes it useful as evidence in a dispute. Saved items are listed in the
  app and can be deleted.
- **A visible indicator.** An LED shows when the pendant is capturing. It is not
  a hidden recorder.
- **Data minimization.** Only what a feature needs is stored. The calorie
  feature keeps a food log, not the photos it was derived from, unless saved.
- **Memory is text, not surveillance.** Inferred preferences and facts are short
  text items the wearer can read and remove, not a transcript of everyone met.
- **Local-first is a first-class option.** Because speech, model, and voice all
  have local implementations (see `docs/architecture.md`), the wearer can run
  the whole pipeline without anything leaving their own machines.

## Home devices

- **Nothing leaves before the wake word.** The kitchen device listens for its
  wake word on the device itself; audio is only sent after it hears it.
- **A visible indicator and a physical mute.** An LED shows when it is listening
  past the wake word, and a switch turns the microphones off.
- **The e-ink board only shows.** It pulls today's plan; it has no microphone or
  camera.

## Health data

Health data is the most sensitive data Daywright holds, so it follows the strictest
rules we face, in every market: the FTC Health Breach Notification Rule for US
health apps, Washington's My Health My Data Act, and GDPR Article 9 in the EU.

- **Opt-in per source.** Each source (the scale, Health Connect, the calendar) is
  switched on by the person, one at a time, and can be switched off.
- **Body photos stay on the device.** Measurements are taken from them on the
  phone; only the measurements are kept unless the person saves the photos.
- **Encrypted, and never sold.** Not to advertisers, not to data brokers, not to
  anyone.
- **The person can see and delete everything,** including every rule and
  preference in memory.

## When models run in the cloud

When a cloud provider is selected, audio goes to the speech provider and frames
and text go to the model provider. The app must make the active providers
visible to the wearer, and switching every capability to local must be possible
without a rebuild.

## Not negotiable

- No silent capture: the indicator is always honest.
- No selling or sharing of captured data.
- Other people's captured data is minimized and never retained beyond what a
  saved item needs.
