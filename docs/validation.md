# Validation record

Date: 2026-08-25

## Completed

| Check | Result |
| --- | --- |
| Foundation SDK 1.0.0 CLI identity | `foundation 1.0.0` |
| Official SDK common archive SHA-256 | Matches the published checksum |
| Official SDK common archive GPG signature | Good signature from fingerprint `5DBE7F185293935315E56E31CFE1890AB7FC8B64` |
| Durable SDK 1.0.0 user installation | Common archive, macOS ARM64 archive, and checksum file signatures verified; both archive checksums matched |
| Nix installation | Nix 2.35.2 on macOS 26.5.2 / ARM64 |
| `foundation doctor` inside `foundation develop` | All 12 checks passed, including the KeyOS target |
| SDK-pinned Rust / Cargo | Rust 1.96.0-nightly (`02c7f9bec`, 2026-04-10); Cargo 1.96.0-nightly (`eb94155a9`, 2026-04-09) |
| `foundation doctor`: app config names | Pass: Vault Signer |
| `foundation doctor`: app icon size | Pass: 110 × 110 px |
| Public-tree safety script | Pass |
| SPDX script | Pass |
| Bash syntax checks | Pass |
| Theme JSON parse | Pass |
| Hosted dependency-free protocol model | 6 passed, 0 failed; Rust 1.97.1 on Ubuntu 24.04 |
| Initial GitHub repository-safety job | Pass |
| `foundation preview ui/app.slint` | Preflight compiled and viewer launched |
| SDK viewer `--check ui/app.slint` with generated UI/theme imports | Exit 0, no diagnostics |
| SDK viewer screenshots | Main and intended-flow pages visually checked at 480 × 800; light/default preview theme |
| `cargo test --locked` after theme-path correction | 6 passed, 0 failed in the full SDK app test target |
| `cargo check --locked` after theme-path correction | Passed; warnings noted below |
| `foundation sim` | Hosted app built, staged, and launched; app emitted its prototype protocol v1 startup message |

The initial [GitHub Actions run](https://github.com/BitZk/passport-prime-vault-signer/actions/runs/32912290236) validated commit `8300e86ed447bdde6c2594b252f97565a8b60a27`. It compiled only `src/protocol.rs`, not the SDK app or Slint UI.

Platform research used KeyOS **v1.3.1**, commit `de966a11e88d28f116b52509679c19eb33591711`. Development checks used Foundation SDK **1.0.0** and its bundled hosted runtime. The SDK manifest records workspace commit `a9ce6713949a5d6bb8f5edfca19bcaabc946f09d` with a clean release build; this is distinct from the earlier KeyOS research snapshot. These are validation snapshots, not a claim about later firmware.

## Development issues found and corrected

- Fresh Nix installations require `nix-command` and `flakes` for `foundation develop`. Enabled them for the development session only; no global configuration change was needed.
- The SDK template's direct-Cargo fallback referenced the global theme cache, while SDK 1.0.0 preflight generated this app's theme under `target/foundation/themes/rust`. Corrected the fallback to the project-local output and added an explicit missing-theme diagnostic. The initial `cargo test --locked` failed on this mismatch; the corrected run passed.
- Cargo refreshed the template lockfile to include the theme crate's registry dependency on `i-slint-common` 1.17.1 alongside the SDK's bundled 1.17.0. Subsequent tests used `--locked`.
- The generated `ui/ui` mapping is a symlink, so the directory-only ignore pattern did not cover it. Corrected the ignore pattern and explicitly reject this mapping in staged-tree checks.
- Excluded SDK-generated `ui/gen` files from the authored-source SPDX check; generated files were not edited.

Compiler output still includes unused-import/dead-code warnings from bundled Slint code and the deliberately unconnected protocol model. Passing these checks is not a warning-free build or a security audit.

## Publication safeguards

The repository is public at [BitZk/passport-prime-vault-signer](https://github.com/BitZk/passport-prime-vault-signer). GitHub API readback confirmed secret scanning, secret-scanning push protection, and private vulnerability reporting enabled on 2026-08-25. Non-provider pattern scanning remained disabled and is not claimed as protection.

Commits use the maintainer's GitHub noreply address. Local hooks and CI run the repository safety checks; matched content is never printed by the checker. These checks are a limited backstop, not proof that a tree contains no secrets. Neither the checker nor GitHub protections should be relied upon to recognize every mnemonic or wallet-data format.

## Remaining validation limits

The simulator boot and app startup were observed, but interactive navigation and dark-mode behavior were not verified. The standalone native viewer/simulator windows were not addressable through the available Mac UI automation interface. The SDK viewer's screenshot facility supplied static page evidence; a simulator screenshot request through the launcher terminal produced no confirmed capture.

No seed activation, signing, QR/NFC/file transport parity, hardware build, or physical-device test has been performed. The app still has no seed, PIN, backup, Keycard, NFC read/write, or cross-app private-storage permission. Its compiled manifest was inspected for that boundary.

No publisher signing identity was generated and no device was contacted. No real wallet data was provided or read. The bundled simulator initialized its own default security test state outside the checkout; that state and raw runtime logs must never be published. The simulator was stopped after the startup check.
