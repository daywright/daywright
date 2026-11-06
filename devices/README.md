# Devices

Daywright is one brain with many devices (ADR-0013). A device senses, tells, or
both, and every device speaks the open protocol in `backend/crates/proto`.

| Device | Folder | Senses | Tells | Milestone |
|--------|--------|--------|-------|-----------|
| Daywright Kitchen, a voice scale | `kitchen/` | weight, voice | speaker, small screen | M2 |
| Daywright Board, e-ink | `board/` | | today's plan | TRMNL-compatible in M1, own board in M6 |
| Daywright Pendant | `pendant/` | voice, camera, tap | earpiece (through the phone) | M5 |
| Body measurement | not started | weight, body composition, waist, grip | | M6 |

Each device folder holds:

- `firmware/`: code that runs on the device. Apache-2.0.
- `hardware/`: KiCad boards and printable enclosures (FreeCAD as the master,
  STL or 3MF exported). CERN-OHL-S-2.0.

## Building a compatible device

Anyone may build a device that speaks the protocol, including for sale: the
protocol and firmware are Apache-2.0. Calling it Daywright, or using the logo, needs
permission (see `TRADEMARKS.md`).
