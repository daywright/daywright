# Licensing

Daywright is open source, split by component (ADR-0011 in `docs/decisions.md`). The
exact mapping, file by file, is in `REUSE.toml`, and the full license texts are in
`LICENSES/`.

| What | Where | License |
|------|-------|---------|
| The brain | `backend/` | GNU Affero General Public License v3.0 only (`AGPL-3.0-only`) |
| The device protocol | `backend/crates/proto/` | Apache License 2.0 (`Apache-2.0`) |
| Device firmware | `devices/*/firmware/` | Apache License 2.0 (`Apache-2.0`) |
| Phone apps | `apps/` | Apache License 2.0 (`Apache-2.0`) |
| Hardware designs and their docs | `devices/*/hardware/` | CERN Open Hardware Licence v2, Strongly Reciprocal (`CERN-OHL-S-2.0`) |
| Documentation | `docs/` and other Markdown files | Creative Commons Attribution 4.0 (`CC-BY-4.0`) |
| Small configuration files | `.gitignore`, `*.example`, `REUSE.toml` | CC0 1.0 (`CC0-1.0`) |

In plain words (the license texts are what count):

- You may run, study, change and share all of it, including commercially.
- If you run a changed version of the brain as a service for other people, you
  must offer them the source of your changes.
- If you distribute changed hardware, you share the changed design.
- The protocol, firmware and apps are permissive: build compatible devices and
  apps, open or closed.
- The Daywright name and logos are not licensed; see `TRADEMARKS.md`.

Copyright 2026 The Daywright contributors.
