# Daywright Board: the e-ink screen

A low-power, always-visible "tell" device for the fridge door, the desk or the
bedside table (ADR-0013). It shows today's plan at a glance and refreshes a few
times a day. It has no microphone and no camera.

## What it shows

- Today's meals, the training session if there is one, the step target.
- One line of coaching ("Screens off at 22:30").
- Progress: the weight trend and the waist.

## Two stages

1. **TRMNL-compatible (M1).** TRMNL sells a 7.5-inch e-ink display with
   open-source firmware and a bring-your-own-server mode, in which the display
   polls a server you run instead of TRMNL's
   ([spec sheet](https://trmnl.app/products/x/spec-sheet)). The Daywright backend
   serves the screen image, so any TRMNL display, or a DIY build of one, works as
   a Daywright board.
2. **Our own board (M6).** An ESP32 with an e-paper panel, a battery and a
   magnetic back, designed here under CERN-OHL-S-2.0.

## Folders

- `firmware/` (not started): needed only for our own board.
- `hardware/` (not started): board and enclosure for our own board.
