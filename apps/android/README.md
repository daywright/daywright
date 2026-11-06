# Daywright Android app

Native Kotlin. The courier in your pocket: it reads Health Connect, relays
Bluetooth devices (the pendant, the earpiece) to the backend, and shows a screen
only when you ask for one. It ships to users together with the iPhone app in M4
(ADR-0012); before that, M1 builds just enough of it for the owner's own phone.

## Open and build

This tree has the project files but not the Gradle wrapper (the `gradlew` script
and its jar). Generate it once with a local Gradle, or just open the folder in
Android Studio, which does it for you:

```sh
cd apps/android
gradle wrapper --gradle-version 8.11   # if you have Gradle; otherwise use Android Studio
./gradlew assembleDebug
```

Requires the Android SDK (compileSdk 35). Not built yet, so treat it as an
unbuilt skeleton.

## What lands in M1 (the owner's courier)

- Read Health Connect (weight from the smart scale, steps, sleep, workouts) and
  send it to the timeline as observations.
- A foreground service for a reliable session with the backend.
- Notifications for orders, only where no better device is near.

## What lands in M4

- Pair any Bluetooth earpiece and play guidance to it.
- The memory list, the plan and progress, for when you want a screen.
- Everything the iPhone app does, released together.
