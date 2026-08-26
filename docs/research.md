# Platform research record

Research date: 2026-08-25

## Sources reviewed

- Foundation developer pages: Developer Home, Get Started, KeyOS, Building Apps, CLI Reference, Capabilities, Developer FAQ, and API Reference landing page.
- Foundation consumer pages: Prime Vault, Prime Bitcoin Wallet, Files, Backups, and Passport Prime FAQs.
- Foundation KeyOS public repository at commit `de966a11e88d28f116b52509679c19eb33591711`, described locally as `v1.3.1`.
- GPG-signed Foundation SDK 1.0.0 common archive, SHA-256 `0426e7d42a7c7c140e99271a594a6f69fcb9695a777a728b6b4a35b0115e92ab`, signed by fingerprint `5DBE7F185293935315E56E31CFE1890AB7FC8B64`.
- Foundation's response supplied by the maintainer, summarized in [Foundation coordination and project status](foundation-status.md). Private correspondence is not reproduced; this is a reported clarification rather than a new firmware test.

## Confirmed findings

1. Prime apps are isolated Rust binaries with app-scoped storage and declarative permissions.
2. Third-party apps may request an app-specific seed and hardware randomness; master seed, PIN/login, backup, Keycard, and NFC read/write capabilities are Foundation-reserved.
3. The built-in Vault v1.3.1 source supports imported 12- and 24-word BIP39 mnemonics and BIP85-derived Bitcoin seeds.
4. The built-in Bitcoin Wallet manifest has Foundation-only `GetSeed` access and owns account/signing/transport behavior.
5. Bitcoin Wallet localization includes Apply/Clear Temporary Seed and Load from Vault labels, but no implemented Vault selection/activation path was found in the public v1.3.1 Bitcoin source.
6. The current GUI navigation service already supports typed app-to-app request/response patterns, but passing raw entropy through its response buffer would increase secret exposure.
7. The correct production boundary is a Foundation-reviewed integration, not a third-party app reading Vault storage.

## Foundation clarification

Foundation confirmed the planned native feature, including manual entry, SeedQR, and loading from Vault. At the time of the response there was no public release or beta, and final scope and timing were not confirmed. It also confirmed that the SDK's app-specific seed is separate from both the device Bitcoin seed and Vault entries, and that generic navigation exposes no supported third-party Vault-selection or temporary-activation request.

Functional implementation and hardware integration are therefore paused. UX/protocol-design input, documentation, and test proposals remain possible contributions, not approval of the proposed API. See the [status record](foundation-status.md) for the source limits and resume criteria.

## Documentation drift noted

- Developer pages displayed SDK `1.0.0-beta.1`; the signed latest installer selected `1.0.0`.
- The consumer Bitcoin Wallet page marks temporary-seed use, including loading from Vault, as forthcoming. Foundation's response clarifies that this describes planned behavior; it is not evidence of a working public interface or an overlooked implementation.
- Foundation's FAQ language around seed access is less precise than the capability matrix, which marks direct master-seed operations as Foundation-signed only.
- The developer overview simplifies private storage as `User`; the reviewed filesystem source distinguishes caller-scoped `AppData` from shared encrypted `User` files. Vault persistence uses `AppData`.

These differences are why this project records exact source snapshots and separates documented plans, Foundation's reported status, and behavior actually demonstrated on firmware.

## Pinned source entry points

- [Vault seed model](https://github.com/Foundation-Devices/KeyOS/blob/de966a11e88d28f116b52509679c19eb33591711/apps/gui-app-seed-vault/src/seed.rs)
- [Vault seed resolution](https://github.com/Foundation-Devices/KeyOS/blob/de966a11e88d28f116b52509679c19eb33591711/apps/gui-app-seed-vault/src/state.rs)
- [Bitcoin seed loading](https://github.com/Foundation-Devices/KeyOS/blob/de966a11e88d28f116b52509679c19eb33591711/apps/gui-app-bitcoin/src/store.rs)
- [Bitcoin navigation handler](https://github.com/Foundation-Devices/KeyOS/blob/de966a11e88d28f116b52509679c19eb33591711/apps/gui-app-bitcoin/src/main.rs)
- [GUI navigation API](https://github.com/Foundation-Devices/KeyOS/blob/de966a11e88d28f116b52509679c19eb33591711/api/gui-server/src/navigation.rs)
- [Filesystem location mapping](https://github.com/Foundation-Devices/KeyOS/blob/de966a11e88d28f116b52509679c19eb33591711/os/fs/src/main.rs)
