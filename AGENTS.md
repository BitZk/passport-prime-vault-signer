# Vault Signer agent guide

This repository is a simulator-safe Foundation SDK app plus exploratory design material for a possible native contribution.

Functional implementation and hardware integration are paused following Foundation's response. Read `docs/foundation-status.md` before planning new work. Preserve the prototype and allow scoped non-secret UX/model, documentation, and test-proposal work; do not add seed storage, seed handoff, signing, or sensitive permissions. Resume only after the documented Foundation-interface/contribution and maintainer-approval gates are met. General encouragement for design feedback is not authorization to implement a native API.

Before running `foundation` commands, read the packaged SDK guide at `<sdk-root>/docs/guide/src/foundation-cli.md`. Run `foundation doctor` inside `foundation develop`. Prefer `foundation preview` for UI checks and `foundation sim` for hosted runtime checks.

Do not run `foundation cert gen`, `foundation cert install`, `foundation build`, `foundation pack`, `foundation sideload`, `foundation logs`, or any hardware command unless the user explicitly authorizes signing or hardware work.

Never add seed words, private keys, xpubs, wallet addresses, PSBTs, device identifiers, certificates, signing identities, `cosign2.toml`, `.env` files, or local SDK paths. Test protocol behavior with zeroed identifiers and synthetic labels only.

The current app must remain simulator-safe and must not request seed, PIN, backup, Keycard, NFC read/write, or cross-app private-storage access. Those capabilities require an upstream Foundation-reviewed KeyOS design.

Use `apply_patch` for edits, preserve SPDX headers, run `./scripts/check-public-tree.sh` and `./scripts/check-spdx.sh`, and state which Foundation SDK/KeyOS snapshot was used for validation.
