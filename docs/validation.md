# Validation record

Date: 2026-08-25

## Completed

| Check | Result |
| --- | --- |
| Foundation SDK 1.0.0 CLI identity | `foundation 1.0.0` |
| Official SDK common archive SHA-256 | Matches the published checksum |
| Official SDK common archive GPG signature | Good signature from fingerprint `5DBE7F185293935315E56E31CFE1890AB7FC8B64` |
| `foundation doctor`: app config names | Pass: Vault Signer |
| `foundation doctor`: app icon size | Pass: 110 × 110 px |
| Public-tree safety script | Pass |
| SPDX script | Pass |
| Bash syntax checks | Pass |
| Theme JSON parse | Pass |

## Not yet validated

The host does not have Nix, Cargo, the KeyOS Rust target, or `arm-none-eabi-strip`. `foundation doctor` correctly reports these missing prerequisites. No Rust compile, unit test, Slint generated-router compile, simulator run, hardware build, or physical-device test has been claimed.

Installing Nix is a system-level setup step and is intentionally separate from the repository scaffold. Once approved and installed, run the commands in [Development](development.md), fix any compiler/UI issues, and replace this section with exact evidence.

No publisher signing identity was generated, no device was contacted, and no wallet seed was read or created.
