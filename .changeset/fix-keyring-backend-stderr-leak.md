---
"@googleworkspace/cli": patch
---

Fix stray "Using keyring backend: keyring" message printed to stderr before every command that touches encrypted credentials. The diagnostic is now emitted via `tracing::debug!` (opt-in through `GOOGLE_WORKSPACE_CLI_LOG`) instead of an unconditional `eprintln!`.
