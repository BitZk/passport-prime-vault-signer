# Vault Signer for Passport Prime

Vault Signer is a design-first Passport Prime project for temporarily applying a BIP39 seed that is already stored in Prime's built-in Vault to the built-in Bitcoin Wallet.

> [!WARNING]
> This repository is an early feasibility and user-interface prototype. It does not read, store, activate, or sign with seed material. Do not enter real seed words into development builds, issues, pull requests, screenshots, or fixtures.

> [!NOTE]
> Functional implementation and hardware integration are paused. Foundation confirmed that temporary-seed use is planned natively, but no supported third-party Vault-to-Bitcoin handoff exists. Final scope and release timing were not confirmed. The simulator prototype is retained for possible UX, protocol-design, documentation, and test contributions. See [Foundation coordination and project status](docs/foundation-status.md).

## Why this project exists

Foundation documents the intended flow as **Bitcoin Wallet → Apply Temporary Seed → Load from Vault**, and Prime's Vault can import external Bitcoin seeds. The reviewed public KeyOS v1.3.1 snapshot contains the Vault import model and Bitcoin Wallet labels; no implemented handoff was found in that snapshot. Foundation subsequently confirmed the planned native feature and the unsupported third-party integration boundary, as summarized in the [status record](docs/foundation-status.md).

That missing connection cannot be implemented honestly as an ordinary third-party SDK app:

- KeyOS scopes each app's private storage, so another app cannot read the built-in Vault database.
- The SDK's app-specific seed belongs to the calling app; it is not the primary Bitcoin seed or a stored Vault seed.
- Generic app navigation does not expose a supported Vault selection or Bitcoin temporary-activation request to third-party apps.
- Master-seed, PIN, backup, and NFC read/write operations are reserved for Foundation-signed apps.
- Reimplementing the Bitcoin Wallet would duplicate its PSBT, account, multisig, passphrase, QR, file, and QuantumLink security surface.

The project therefore preserves design input for a possible Foundation-coordinated contribution rather than pursuing a separate vault or signer. The SDK app is a simulator-safe UX shell with no secret permissions. Its interaction design, proposed protocol, threat model, and tests are not an approved native implementation plan.

Relevant official sources:

- [Foundation developer home](https://docs.foundation.xyz/developers/home/)
- [KeyOS security model](https://docs.foundation.xyz/developers/keyos/)
- [capability matrix](https://docs.foundation.xyz/developers/capabilities/)
- [building Prime apps](https://docs.foundation.xyz/developers/building-apps/)
- [Prime Vault](https://docs.foundation.xyz/prime/prime-apps/vault/)
- [Prime Bitcoin Wallet](https://docs.foundation.xyz/prime/prime-apps/wallet/)
- [Foundation KeyOS source](https://github.com/Foundation-Devices/KeyOS)

## Intended experience

The following is the project's design goal, not confirmed Foundation release scope. Session behavior and feature/transport parity remain subject to native design review.

1. The user opens the built-in Bitcoin Wallet and chooses **Apply Temporary Seed**.
2. Prime opens the built-in Vault in a seed-selection mode.
3. The user selects a Bitcoin seed and confirms its label and master fingerprint on the trusted display.
4. KeyOS activates the seed only for the current powered session.
5. The existing Bitcoin Wallet handles accounts, multisig, passphrases, address verification, message and PSBT signing, QR, files, and QuantumLink.
6. The user clears the temporary seed manually, or KeyOS clears it at shutdown.

The proposed cross-app response contains only an opaque session identifier and non-secret display metadata. Seed entropy is not part of the public navigation protocol. See [Architecture](docs/architecture.md), [Data model](docs/data-model.md), and [Threat model](docs/threat-model.md).

## Repository layout

```text
.
├── app-config.toml          # Foundation SDK identity and minimal permissions
├── src/                     # Rust UX shell and non-secret protocol model
├── ui/                      # Slint multi-page prototype
├── resources/               # Required app icons and theme
├── docs/                    # Architecture, decisions, research, and roadmap
├── scripts/                 # Public-tree and SPDX checks
└── .github/                 # CI and contribution templates
```

The app layout follows Foundation's `multi-page-app` SDK template. The generated `.foundation-sdk/current` mapping and all signing material remain local and are ignored.

## Development

Foundation's toolchain is Nix-first. After installing Nix and the [Foundation SDK](https://docs.foundation.xyz/developers/get-started/):

```bash
foundation develop
foundation doctor
foundation preview ui/app.slint
foundation sim
```

Run the repository safety checks before every commit:

```bash
git config core.hooksPath .githooks
./scripts/check-public-tree.sh
./scripts/check-spdx.sh
```

Do not run `foundation cert gen`, commit `cosign2.toml`, or place signing keys inside the repository. Hardware building and sideloading are deliberately outside the current milestone. See [Development](docs/development.md).

## Project status

- [x] Confirm platform constraints against the current docs and KeyOS source.
- [x] Establish an SDK-conformant, simulator-safe UX project.
- [x] Define a secret-free navigation contract and session state model.
- [x] Document the security invariants and upstream change surface.
- [x] Ask Foundation about native plans and record the platform boundary.
- [x] Pause functional implementation and hardware integration; retain research materials.
- [ ] Reassess a supported interface or specific Foundation-coordinated contribution when available.
- [ ] Obtain maintainer approval and the required design/security review before any implementation milestone resumes.

See the [Roadmap](docs/roadmap.md) for conditional future milestones and the [status record](docs/foundation-status.md) for resume criteria.

## Contributing

Community design, documentation, and test-proposal review is welcome, especially around seed identification, confirmation, temporary-seed lifecycle, and failure handling. Functional seed-handling and hardware contributions are paused. Read [CONTRIBUTING.md](CONTRIBUTING.md) and [SECURITY.md](SECURITY.md) first.

Never submit real wallet data. Use only structural placeholders such as `fingerprint: 00000000`; do not use published mnemonic test vectors because they are routinely mistaken for usable wallets.

## Name and trademarks

**Vault Signer** describes the project's focus on deliberate seed selection and signing on Passport Prime.

Passport, Passport Prime, KeyOS, Envoy, and Foundation are trademarks or product names of Foundation Devices. This independent project is not endorsed by Foundation Devices.

## License

The standalone prototype is MIT licensed. Any patch copied into or derived from Foundation's GPL-3.0-or-later KeyOS repository must retain the applicable KeyOS copyright and GPL notices; see [Upstream strategy](docs/upstream-strategy.md).
