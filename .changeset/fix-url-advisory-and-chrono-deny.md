---
"@googleworkspace/cli": patch
---

Bump url to 2.5.9 (RUSTSEC-2026-0592 stack overflow in `Url::origin()` on nested blob: URLs) and ignore the chrono/chrono-tz unmaintained advisories in deny.toml — maintenance notices with no available upgrade, matching the existing cargo-audit behavior.
