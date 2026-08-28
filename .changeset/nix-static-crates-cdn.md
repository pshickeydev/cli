---
"@googleworkspace/cli": patch
---

Fix the Nix flake build failing in CI with `curl: (22) ... error: 403` when vendoring crate sources. The crates.io API endpoint rate-limits downloads with intermittent 403 responses; the flake's nixpkgs pin is updated to a revision where `importCargoLock` fetches crate sources from the `static.crates.io` CDN instead.
