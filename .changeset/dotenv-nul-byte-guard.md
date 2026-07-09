---
"@googleworkspace/cli": patch
---

Guard `.env` file parsing against NUL bytes in keys/values. `std::env::set_var` panics on a NUL byte, so a malformed or adversarial `.env` file could previously crash the CLI on startup; such lines are now silently skipped instead.
