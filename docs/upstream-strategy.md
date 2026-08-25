# Upstream strategy

## Why upstream is required

The feature crosses the private storage and sensitive seed boundaries of two Foundation-signed applications. A sandboxed SDK app cannot safely or completely implement it. The production implementation should be reviewed and merged in Foundation's KeyOS repository.

This repository remains useful as the independent proposal, UX prototype, threat model, protocol model, validation record, and community discussion space.

## Proposed KeyOS change surface

The exact paths may change after Foundation review, but the current v1.3.1 source suggests these areas:

- `api/gui-server/src/navigation/vault.rs` — typed seed-selection request and secret-free result.
- `api/gui-server/src/navigation.rs` — Vault app ID and navigation helper.
- `apps/gui-app-seed-vault` — selection-only route, explicit confirmation, activation call, cancellation, and response.
- `api/security` and `os/security` — Foundation-only temporary signing-seed session, opaque handles, caller binding, state query, clear, and zeroization.
- `apps/gui-app-bitcoin` — Apply/Clear Temporary Seed callbacks, active-source state, account reload, fingerprint verification, and existing transport reuse.
- `os/gui-app-launcher` — accurate temporary-seed status and clear/recovery behavior.
- manifests and permission templates — narrowly scoped Foundation-app permissions only; no broad third-party template change.
- integration tests — lifecycle, navigation, signing parity, replay rejection, failure recovery, and power/lock semantics.

## Contribution sequence

1. Ask Foundation to confirm the product behavior and preferred security-service boundary.
2. Submit the protocol and lifecycle types without signing behavior.
3. Add the security-service temporary session with exhaustive unit and IPC tests.
4. Add the Vault selection/activation flow and trusted-display confirmation.
5. Refactor Bitcoin's seed source and prove primary-seed behavior unchanged.
6. Add QR, file, QuantumLink, account, multisig, passphrase, message, and PSBT parity tests.
7. Test disposable regtest flows in the simulator and on a developer device.
8. Obtain independent review before requesting release inclusion.

Each step should be a small, reviewable KeyOS commit. Do not ship a monolithic patch that mixes GUI, permissions, security storage, and signing.

## Licensing

KeyOS is GPL-3.0-or-later and requires SPDX headers on new files. Any copied or derived KeyOS implementation belongs in a GPL-licensed upstream branch or patch directory with original Foundation notices preserved. The MIT license in this repository applies only to the independent prototype and documentation; it does not relicense KeyOS code.

## No implied approval

A successful simulator build, a developer-signed app, a public repository, or community review is not Foundation approval and is not authorization to represent the project as an official Prime feature.
