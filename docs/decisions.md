# Decisions

A log of the choices that shape Daywright and why, so the reasoning is not lost and
later work does not quietly undo it. Newest context at the top of each entry.

## ADR-0001: The name is Daemon

Status: superseded on 2026-11-07 by ADR-0018.

A background process that is always running, and the Greek daimon, the inner
voice that warned a person before a mistake. The double meaning fits an
always-on open project and the moment that started it (being overcharged and
freezing for a second). Checked against obvious clashes; the ones to avoid were
Echo, Halo, Oracle, Copilot, Jarvis, and Friday.

## ADR-0002: Rust for the backend, framework axum

Chosen by the owner. One memory-safe language for the whole backend, efficient
and cheap to self-host, which fits the self-hosting preference.

Honest note on a reversal: an earlier recommendation leaned away from Rust. The
latency argument for or against any backend language is weak here, because the
delay lives in the network and the models, not the glue code. The real argument
against Rust was ecosystem: a Python framework already had every speech provider
wired up and the fiddly realtime parts solved. That argument was removed by
ADR-0009 (build it ourselves). With the pipeline built in house regardless, Rust
is coherent. The accepted cost is that we hand-build the realtime media
orchestration (gating, turn-taking, barge-in) that a framework would have given.

axum keeps the service structured in layers with tower middleware, rather than
ad hoc handlers.

## ADR-0003: Fully native apps, Android first

Status: release order superseded on 2026-11-06 by ADR-0012. The native
requirement stands.

Chosen by the owner. An always-on wearable leans hard on background execution,
BLE, and audio session control, which native platforms do best. Two codebases
(Kotlin and Swift) is the accepted cost. Android is first, matching the owner's
stock-Android preference; iOS follows.

## ADR-0004: Self-host or cloud from day one

Chosen by the owner. Every capability (speech to text, language model, text to
speech) is a trait with both a cloud and a local implementation, selected by
config. No build targets one path only.

## ADR-0005: Phone-tethered v1, cellular add-on later

The pendant talks to a gateway. The phone is the gateway now; a cellular module
can replace it later without changing the pendant or the backend.

Why not a SIM in the pendant now: a standalone modem plus always-on capture on a
neck pendant is the Humane AI Pin design, which failed largely on battery life
and overheating, sold about ten thousand units, and was shut down (HP bought the
assets for 116 million dollars and the devices stopped working on 28 February
2025). A pendant has even less room for battery and heat than a lapel pin. The
phone is already a charged battery, a modem, a screen, and a processor.

## ADR-0006: Seeed XIAO ESP32-S3 Sense

Small (about 21 by 18 by 15 mm with the sense board), has the camera and digital
mic, and the owner can solder and 3D print. It is BLE only, with no Classic
Bluetooth, which is why the earpiece is driven by the phone (ADR-0007). Current
stock ships with a 5 MP autofocus camera.

## ADR-0007: Voice-first, proactive within bounds, any earpiece

Status: amended on 2026-11-06 by ADR-0014 (stationary home devices use an
on-device wake word) and ADR-0015 (anything said once is kept as a visible rule).

- Tap to address it; no spoken wake word.
- It interjects on its own only for money, commitments, health, and safety;
  waits for a pause; stays under about ten words; double tap mutes.
- No manual watcher list. Standing preferences are inferred and stored as
  memory the owner can review or correct.
- Any earpiece works because the phone drives the Bluetooth audio. A hidden
  induction earpiece is supported through a loop in the necklace.

## ADR-0008: PolyForm Noncommercial License 1.0.0

Status: superseded on 2026-11-06 by ADR-0011.

The owner wants personal use free and commercial use to go through him. That is
source-available, not open source: the Open Source Definition forbids
restricting use by field of endeavor, and barring commercial use is exactly
that. PolyForm Noncommercial 1.0.0 is built for this case. We call it
source-available, never open source, to keep the claim honest. If the intent
later changes to "anyone may use it, improvements stay open, and I sell
exceptions", the route is AGPL-3.0 plus dual licensing instead.

## ADR-0009: Build it ourselves

The owner chose not to fork or depend on Omi or a media framework. Omi's pendant
is audio only (only its glasses have a camera) and its backend hard-requires
Firebase, Google Cloud, Firestore, Pinecone, Redis, and cloud model keys, which
is cloud-locked and leans on a stack the owner avoids. We study the prior art
but own our code.

## ADR-0010: Daywright absorbs the health coach; health is the first job

2026-11-06. Decided by the owner.

In November 2026 the owner explored a separate product: a coach that measures as
much as possible and tells a busy person what to eat and how to train, so they
never have to think about it. It is the same idea as Daywright, "some super
intelligence" that sees, counts and gives orders, and two projects would mean two
brains.

Decision: one project. Daywright is a brain you talk to plus a suite of devices that
sense and tell (ADR-0013). Its first job is health: getting busy people who were
once in shape back in shape, with the owner as user number one. The original jobs
in ADR-0007 (money, commitments, translation) stay and come after health.

Consequences: the data model is built for any part of life from day one
(ADR-0016) while the product stays focused on health until it earns its keep.
Business material (the concept deck, pricing, fundraising) lives in `private/`,
which is ignored by git and never published.

## ADR-0011: Open source, split by component

2026-11-06. Supersedes ADR-0008. Decided by the owner on Claude's recommendation.

ADR-0008 chose a noncommercial license because commercial use was to go through
the owner. The goal has changed: an open, community-driven suite of software and
devices in the spirit of the ArtCraft apps, and a business that earns from a
hosted service and hardware rather than from the code. Open projects with that
shape do earn: ArtCraft's apps are free and open source and it sells optional
cloud AI credits ([pricing](https://getartcraft.com/pricing)); Home Assistant is
open source and its company is funded by a cloud subscription and its own
hardware, with no outside investors
([Open Home Foundation](https://newsletter.openhomefoundation.org/support-our-work/)).
Nothing here is published yet, so the license is chosen once and kept. Projects
that changed license later were forked by their communities: Terraform became
OpenTofu and Redis became Valkey, and Redis lost most of its outside contributors
([DevClass](https://devclass.com/2025/04/01/one-year-ago-redis-changed-its-license-and-lost-most-of-its-external-contributors/)).

| Component | License | Why |
|-----------|---------|-----|
| `backend/`, the brain | AGPL-3.0-only | Anyone can run and change it. Anyone who offers a changed version as a network service must share the changes, so a larger company cannot take it closed. |
| `backend/crates/proto/`, the device protocol | Apache-2.0 | Device makers and app authors can implement it in any product, including commercial ones. |
| `devices/*/firmware/`, `apps/` | Apache-2.0 | Permissive, and compatible with app store terms. Copyleft code has been pulled from Apple's App Store before: VLC in 2011 ([Computerworld](https://www.computerworld.com/article/1347653/gplv2-blocks-vlc-from-apple-s-app-store.html)). |
| `devices/*/hardware/` | CERN-OHL-S-2.0 | Changed hardware designs must be shared when distributed. |
| Documentation | CC-BY-4.0 | Reuse with credit. |
| Small configuration files | CC0-1.0 | No conditions on boilerplate. |

The exact mapping is in `REUSE.toml` and the full texts are in `LICENSES/`.

- **No contributor license agreement.** Contributions come in under the license of
  the folder they touch (inbound equals outbound), signed off under the Developer
  Certificate of Origin. That is the promise that the license will not be pulled
  later.
- **The name and logos are not licensed** (`TRADEMARKS.md`). A license cannot stop
  a copy of the hardware, as Prusa found with 1:1 clones of its printers; a
  trademark stops the copy from using the name.
- **The paid layer is separate software.** Daywright Cloud (hosting, accounts,
  billing, premium programs) lives in a private repository and talks to the brain
  only through its public API.

Why not the route ADR-0008 named, AGPL plus dual licensing: selling license
exceptions needs the right to relicense every contribution, which means a
contributor license agreement, and that is the tool communities have learned to
distrust after relicensings like Terraform's and Redis's. The revenue here comes
from hosting and hardware, not from selling exceptions.

## ADR-0012: iPhone and Android together

2026-11-06. Supersedes the release order in ADR-0003. Decided by the owner on
Claude's recommendation.

ADR-0003 put Android first to match the owner's own phone. Daywright is now English
first and aimed at paying users, and in the US most phones sold are iPhones,
about 60% in Q1 2026 by Omdia's count
([MacTech](https://www.mactech.com/2026/05/27/apple-sold-19-9-million-iphones-in-the-first-quarter-for-60-of-the-us-smartphone-market/)).
Apple Health can only be read from an iPhone app.

Decision: both apps reach users together (M4). ADR-0003's native requirement
stands for what needs it: Bluetooth, background audio and health data. Before M4
only the owner's own Android courier is built (M1), because that is the phone the
owner carries. Whether the shared logic is written once (Kotlin Multiplatform) or
twice is decided at the start of M4 in its own ADR.

## ADR-0013: A suite of devices on one open protocol

2026-11-06.

The owner does not want to hold a phone, and different moments need different
devices: a kitchen with busy hands, a fridge door in the morning, the street, the
gym.

Decision: Daywright is one brain with many devices. A device senses, tells, or both.
Every device speaks the open protocol in `backend/crates/proto` (Apache-2.0) over
the local network, over Bluetooth through the phone, or through the cloud relay.
The phone stays in the pocket as the courier for Bluetooth devices and for health
data that only exists on the phone; nothing requires looking at it.

Build order: the kitchen voice scale (ADR-0014), the e-ink board (TRMNL-compatible
first, then our own board), the pendant (ADR-0006), then measurement devices.
Third-party devices join through connectors: watches and scales through Apple
Health and Health Connect, TRMNL displays through their bring-your-own-server
mode, and optionally a cloud aggregator such as Terra behind the same kind of
trait as every other provider (ADR-0004). ADR-0009 still holds: we integrate
other products, we do not fork them or depend on their backends.

Tell routing: the brain picks the device that fits the moment and the person's
rules. The kitchen speaker while cooking, the e-ink board in the morning, the
earpiece outside, the watch at the gym, and the phone screen only when asked.

## ADR-0014: The kitchen voice scale is the first own device

2026-11-06. Amends ADR-0007 for stationary home devices.

The owner's founding frustration with calorie apps was holding a phone with dirty
hands while cutting tomatoes. The wish: say "one big tomato" to a small device,
with no button, and be done. Food is the hardest data to capture, and most home
food passes through the kitchen.

Decision: build a counter device before the pendant: a scale, microphones with an
audio front end, a speaker and a small display, on the ESP32-S3 family. It listens
for a wake word on the device itself, so nothing leaves it until it is addressed.
The pendant keeps tap-to-talk, because the street is public and the kitchen is
not. Weighing makes home food entries measured instead of guessed. Food values
come from USDA FoodData Central: a large tomato is 182 g at 18 kcal per 100 g,
about 33 kcal ([FoodData Central 170457](https://api.nal.usda.gov/fdc/v1/food/170457?api_key=DEMO_KEY)).

Reference design: Home Assistant's Voice Preview Edition, open hardware on an
ESP32-S3 with an XMOS XU316 audio processor, sold for $59
([CNX Software](https://www.cnx-software.com/2024/12/20/59-voice-preview-edition-adds-an-offline-smart-speaker-to-your-home-assistant-server/)).

Why before the pendant: it stays plugged in, so there are no battery or heat
limits (the problem behind ADR-0005); no camera is worn in public; the parts are
cheap modules; and it can be built at home with a soldering iron and a 3D printer.

## ADR-0015: Rules you say once

2026-11-06. Refines ADR-0007.

The owner wants to give standing instructions by voice: "remember that I only have
Fridays to work out", "keep the food simple and consistent each day".

Decision: spoken instructions become entries in one memory list. Each entry is a
rule (a hard constraint) or a preference (a soft one), with where it came from and
when. Preferences inferred under ADR-0007 join the same list, marked as inferred
until confirmed. The planner must satisfy every rule; when a rule slows a goal
down, Daywright says so once and then obeys. The list can be read on any screen and
changed by voice ("forget the Friday rule").

Example: with Fridays only, Daywright plans one full-body Friday session plus daily
walking. A 2021 review found that weekly training volume matters more than how
many days you train, with at least four hard sets per muscle group a week
([Iversen et al.](https://pubmed.ncbi.nlm.nih.gov/34125411/)).

ADR-0007 said there is no manual watcher list. That still holds: nobody fills in
a settings screen. But anything said once is kept, and it is visible.

## ADR-0016: One timeline of observations

2026-11-06.

The owner wants all data in one place for the AI to work with, and room for the
rest of life later without a rewrite.

Decision: every input from every source is stored as an observation: what was
measured or what happened, its value and unit, when, which source and device, and
how confident we are. Weights, meals, sleep, steps, workouts, measurements taken
from photos and busy times from the calendar are all observations. Plans, rules,
devices and people are ordinary records next to the timeline. This is one plain
table that is almost only appended to; it is not event sourcing. A new part of
life (money, the home) adds new kinds of observation and new sources, not new
storage.

This is the decision that is expensive to change later; everything else grows on
top of it.

## ADR-0017: Health guardrails

2026-11-06.

- **Wellness, not medical.** No diagnosis or treatment claims, which keeps Daywright
  inside the FDA's general wellness policy for weight and fitness tools.
- **18 and over.**
- **Warning signs stop the plan.** Chest pain, fainting, very high blood pressure
  or signs of an eating disorder pause the plan and point to a doctor.
- **No steroid or other drug coaching.** Apple's App Review Guideline 1.4.3 bars
  apps that encourage illegal drug use.
- **Mainstream evidence, not gym folklore.**
- **Body photos are processed on the device.** By default only the measurements
  are kept. Health data is opt-in per source, encrypted and never sold, built to
  the strictest rules we face: the FTC Health Breach Notification Rule for US
  health apps, Washington's My Health My Data Act, and GDPR Article 9.
- **Supplements, if sold, are evidence-backed basics only**, and Daywright may always
  say "you don't need any".

These are design rules, not legal advice. Each market is checked before launch
there.

## ADR-0018: The name is Daywright

2026-11-07. Supersedes ADR-0001. Chosen by the owner from a shortlist that Claude
checked.

"Daemon" suited a single wearable, but the project is now a health brain and a
suite of devices for a mainstream audience, built by a community. To most people
the word reads as "demon", and as a common computing term it can neither be found
in search nor owned as a brand.

Decision: Daywright. A wright is a maker, as in shipwright or playwright, and the
name reads as "day right": the brain helps make each day right, and the people who
build it are its wrights. The devices are Daywright Kitchen, Daywright Board and
Daywright Pendant; the hosted service is Daywright Cloud.

Checked on 2026-11-07: the GitHub organization name was free; daywright.dev,
daywright.org and daywright.co were unregistered; daywright.com (registered in
September 2025) and daywright.app belong to others; no business using the name
was found. Rejected in the same check: Fettle and Hale, each already used by
several health businesses, and Homeostat, whose GitHub name and main domains were
all taken.

The other decisions now use the new name; ADR-0001 keeps the old one as history.
The Android application ID is `dev.daywright.app`, which assumes the project owns
daywright.dev.
