# Daywright pendant firmware

Target: Seeed XIAO ESP32-S3 Sense (ESP32-S3, 8 MB PSRAM, digital PDM mic,
5 MP autofocus camera on the sense board). Built with PlatformIO, Arduino-ESP32
core.

## What it will do (M5)

- Capture microphone audio continuously, run voice-activity detection on device
  so silence is never streamed (battery, bandwidth, speech cost).
- Capture a camera frame only on an event: speech, a tap, or food in view.
- Read the tap sensor: single tap addresses Daywright, double tap mutes, hold
  saves the rolling buffer.
- Drive the status LED as an honest capture indicator.
- Stream to the courier over Wi-Fi when a known network is present, otherwise
  over the phone relay.

## Build

```sh
cp wifi_config.h.example wifi_config.h   # fill in, stays local
pio run                                   # build
pio run -t upload                         # flash over USB-C
pio device monitor                        # serial log
```

The `espressif32` platform and toolchain are a large first download. Not built
yet.

## Notes

- The ESP32-S3 is BLE only. The earpiece is driven by the phone, not the
  pendant (see ADR-0006 and ADR-0007 in `docs/decisions.md`).
- Always-on camera is a battery and heat trap; the camera sleeps and wakes on
  events. This constraint is a first-class part of the design, not an
  afterthought.
