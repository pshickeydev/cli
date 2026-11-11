---
"@googleworkspace/cli": patch
---

Ignore RUSTSEC-2026-0370/0371/0372 (zvariant, zbus_names, zvariant_derive unmaintained) in cargo-deny. These maintenance notices reach the graph only transitively via keyring v4's Linux secret-service backend (keyring -> zbus-secret-service-keyring-store -> secret-service v5 -> zbus v5); no published fix exists upstream yet, so the fix is to pin the ignore with a revisit condition rather than fail CI.
