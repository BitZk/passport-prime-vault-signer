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
| Hosted dependency-free protocol model | 6 passed, 0 failed; Rust 1.97.1 on Ubuntu 24.04 |
| Initial GitHub repository-safety job | Pass |

The initial [GitHub Actions run](https://github.com/BitZk/passport-prime-vault-signer/actions/runs/32912290236) validated commit `8300e86ed447bdde6c2594b252f97565a8b60a27`. It compiled only `src/protocol.rs`, not the SDK app or Slint UI.

Platform research and SDK checks used Foundation SDK **1.0.0** and KeyOS **v1.3.1**, commit `de966a11e88d28f116b52509679c19eb33591711`. These are validation snapshots, not a claim about later firmware.

## Publication safeguards

The repository is public at [BitZk/passport-prime-vault-signer](https://github.com/BitZk/passport-prime-vault-signer). GitHub API readback confirmed secret scanning, secret-scanning push protection, and private vulnerability reporting enabled on 2026-08-25. Non-provider pattern scanning remained disabled and is not claimed as protection.

Commits use the maintainer's GitHub noreply address. Local hooks and CI run the repository safety checks; matched content is never printed by the checker. These checks are a limited backstop, not proof that a tree contains no secrets. Neither the checker nor GitHub protections should be relied upon to recognize every mnemonic or wallet-data format.

## Not yet validated

The local host does not have Nix, Cargo, the KeyOS Rust target, or `arm-none-eabi-strip`. `foundation doctor` correctly reports these missing prerequisites. The hosted protocol-model test does not validate the full app. No Foundation SDK app compile, Slint generated-router compile, simulator run, hardware build, or physical-device test has been claimed.

Installing Nix is a system-level setup step and is intentionally separate from the repository scaffold. Once approved and installed, run the commands in [Development](development.md), fix any compiler/UI issues, and replace this section with exact evidence.

No publisher signing identity was generated, no device was contacted, and no wallet seed was read or created.
