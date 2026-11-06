// Daywright pendant firmware (skeleton).
//
// Role: capture microphone audio and, on an event, a camera frame; run voice
// activity detection on device so silence never leaves the pendant; read the
// tap sensor; drive the status LED as an honest capture indicator; stream up to
// the gateway. See docs/architecture.md and docs/privacy-and-recording.md.
//
// This is a skeleton. It has not been compiled. Real capture, VAD, and the
// WebSocket link land in milestone M3 (see docs/roadmap.md). Wi-Fi credentials
// live in wifi_config.h, which is gitignored; copy wifi_config.h.example.

#include <Arduino.h>

// #include "wifi_config.h"  // defines WIFI_SSID / WIFI_PASS (not committed)

static const int PIN_STATUS_LED = LED_BUILTIN;

enum class LedPattern { Off, Capturing, Active, Muted };

void setLed(LedPattern pattern) {
  // TODO(M3): drive an RGB or single LED to show, honestly, when capturing.
  switch (pattern) {
    case LedPattern::Capturing:
      digitalWrite(PIN_STATUS_LED, HIGH);
      break;
    default:
      digitalWrite(PIN_STATUS_LED, LOW);
      break;
  }
}

void setup() {
  Serial.begin(115200);
  pinMode(PIN_STATUS_LED, OUTPUT);
  setLed(LedPattern::Off);

  // TODO(M3): init PSRAM camera (OV5640), I2S PDM microphone, tap sensor.
  // TODO(M3): connect Wi-Fi; otherwise rely on the phone gateway relay.
  // TODO(M3): open the WebSocket session to the gateway.
  Serial.println("daywright pendant: skeleton boot");
}

void loop() {
  // TODO(M3): the capture and stream loop.
  //  1. read a short audio frame from I2S
  //  2. run on-device VAD; if silence, sleep and continue
  //  3. on speech or a tap, stream audio up; capture a camera frame on event
  //  4. play any audio the gateway sends to the chosen route
  //  5. manage battery: sleep the camera aggressively
  delay(1000);
}
