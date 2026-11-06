# Daywright Kitchen: the voice scale

The first Daywright device (ADR-0014). A counter unit that hears and weighs food so
nobody holds a phone with dirty hands.

## How it is used

> "One big tomato."
>
> "Large tomato, 182 grams, 33 calories. Logged."

- **No button.** It listens for a wake word on the device itself; nothing leaves
  it until it hears the wake word.
- **Weighed, not guessed.** Food on the scale while you speak is logged at its
  measured weight. Without the scale it uses the standard portion: USDA FoodData
  Central lists a large tomato at 182 g and 18 kcal per 100 g.
- **Short answers** through its speaker, under about ten words; a small screen
  shows the last items and today's total.
- **Mute switch and LED**, as in `docs/privacy-and-recording.md`.

## Prototype (M2), candidate parts

| Part | Candidates | Notes |
|------|------------|-------|
| Main board | ESP32-S3 (XIAO ESP32-S3 or ESP32-S3-DevKitC) | Same family as the pendant (ADR-0006). |
| Scale | load cell plus a 24-bit ADC (HX711 or NAU7802) | Kitchen range, 1 g resolution. |
| Microphones | two I2S MEMS microphones, or a voice front end like the XMOS XU316 | Kitchens are loud: water, extractor fans, frying. |
| Speaker | I2S amplifier and a small speaker | For the spoken confirmation. |
| Display | small e-paper or OLED | Last items, today's total. |
| Enclosure | 3D-printed, wipeable top | Food-safe surface, easy to clean. |

Reference design: Home Assistant's Voice Preview Edition is open hardware on an
ESP32-S3 with an XMOS XU316 audio processor
([CNX Software](https://www.cnx-software.com/2024/12/20/59-voice-preview-edition-adds-an-offline-smart-speaker-to-your-home-assistant-server/)).

## Folders

- `firmware/` (not started): wake word, audio capture, scale readout, protocol.
- `hardware/` (not started): board and enclosure.
