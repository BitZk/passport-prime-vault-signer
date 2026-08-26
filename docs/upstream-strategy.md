# Upstream strategy

## Current strategy — coordinate, do not duplicate

Foundation has confirmed the planned native feature and advised against hardware integration without a supported interface. Functional implementation is paused. Retain this repository as possible UX, protocol-design, documentation, and test input; do not start a separate vault or signer. See the [technical response summary and resume criteria](foundation-status.md).

## Why native coordination is required

The desired built-in Vault-to-Bitcoin flow crosses private storage and sensitive seed boundaries of two Foundation-signed applications. An ordinary SDK app has no supported selection or activation request for that flow. App-specific seeds and generic navigation do not substitute for such an interface. Any native implementation contribution would need Foundation coordination and review; a future supported interface must be evaluated on its actual contract.

This repository remains useful as the independent proposal, UX prototype, threat model, protocol model, validation record, and community discussion space.

## Proposed KeyOS change surface

This is exploratory material from the reviewed v1.3.1 snapshot, not an implementation assignment or Foundation-approved design. If a native contribution is agreed, the relevant areas might include:

- `api/gui-server/src/navigation/vault.rs` — typed seed-selection request and secret-free result.
- `api/gui-server/src/navigation.rs` — Vault app ID and navigation helper.
- `apps/gui-app-seed-vault` — selection-only route, explicit confirmation, activation call, cancellation, and response.
- `api/security` and `os/security` — Foundation-only temporary signing-seed session, opaque handles, caller binding, state query, clear, and zeroization.
- `apps/gui-app-bitcoin` — Apply/Clear Temporary Seed callbacks, active-source state, account reload, fingerprint verification, and existing transport reuse.
- `os/gui-app-launcher` — accurate temporary-seed status and clear/recovery behavior.
- manifests and permission templates — narrowly scoped Foundation-app permissions only; no broad third-party template change.
- integration tests — lifecycle, navigation, signing parity, replay rejection, failure recovery, and power/lock semantics.

## Conditional implementation sequence

Do not start this sequence during the pause. Foundation's final design may replace it, and its own implementation may make a contribution unnecessary.

1. Meet the documented resume criteria, agree on the contribution and integration boundary, and obtain the applicable security review checklist.
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
