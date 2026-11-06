# Daywright iOS app

Native Swift. Same job as the Android app: the courier in your pocket that reads
Apple Health, relays Bluetooth devices, and shows a screen only when asked. Apple
Health can only be read from an iPhone app, so this app is required for iPhone
users.

Ships to users together with the Android app in M4 (ADR-0012). The Xcode project
is generated when M4 begins, rather than hand-written here, so it is correct from
the start. Whether the shared logic is written in Kotlin Multiplatform or in
Swift is decided at the start of M4. iOS background execution and Core Bluetooth
are stricter than Android, so they get tested first.
