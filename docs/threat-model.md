# Threat model

## Assets

- Imported and BIP85-derived Bitcoin seed entropy stored by Vault.
- Private keys and signatures derived from the selected seed.
- The user's intent to select one specific Vault entry.
- The integrity of transaction, message, address, account, and multisig data shown for approval.
- The primary seed, which must remain unaffected by temporary-seed operations.

## In-scope attackers

- A malicious or compromised companion phone, wallet, or QuantumLink peer.
- A malicious third-party KeyOS app.
- A malformed QR, UR, file, NFC payload, PSBT, multisig configuration, or navigation message.
- A local attacker replaying a stale navigation response or session handle.
- An interrupted activation, clear, lock, shutdown, or app crash.
- A contributor accidentally committing a seed, signing key, wallet artifact, or machine-specific signing configuration.

## Security invariants

1. Selecting a Vault seed requires an explicit action and confirmation on Prime's trusted display.
2. The confirmation identifies the seed by user label and independently computed master fingerprint.
3. Cross-app navigation never contains seed entropy or private derivation material.
4. Each response is bound to one request and incompatible versions fail closed.
5. A temporary session is scoped to the Bitcoin Wallet, the current boot, and one active seed.
6. The primary seed is never overwritten or re-encrypted to implement a temporary session.
7. Clearing blocks new signing before secret state is zeroized.
8. Shutdown always destroys temporary seed state. Lock should clear it unless Foundation approves and tests a different rule.
9. Passphrases apply only to the currently active seed and are never persisted.
10. The Bitcoin Wallet re-derives and displays its active fingerprint; it never trusts the Vault label or response alone.
11. Existing transaction and message parsing remains the final authority for signing through every transport.
12. Seeds, private keys, passphrases, xprvs, PSBTs, and session secrets never enter logs, panics, crash reports, telemetry, screenshots, fixtures, or Git history.

## Failure rules

- Unknown versions, malformed buffers, nonce mismatches, unknown session IDs, and stale responses are rejected.
- If activation status is ambiguous, signing is disabled until the security service reports one authoritative state.
- If clear status is ambiguous, signing remains disabled and the device requires a restart or an authenticated recovery path.
- A Vault error returns to the primary-seed UI without changing the active source.
- A Bitcoin account-load error must not fall back to another seed and continue signing.

## Out of scope for the initial design

- Resistance to invasive physical extraction beyond existing Passport Prime guarantees.
- Changes to Prime's secure-boot, firmware-signing, PIN, Keycard, or backup design.
- Adding third-party access to Foundation-reserved seed or NFC APIs.
- Supporting non-BIP39 seeds or non-Bitcoin signing.
- Production claims without independent review and disposable-regtest hardware validation.

## Required negative tests

- replayed response, wrong request ID, wrong protocol version;
- expired, guessed, wrong-caller, and already-cleared session IDs;
- cancellation at every screen and interruption at every IPC boundary;
- lock, shutdown, power loss, app crash, and out-of-memory behavior;
- malformed/oversized labels and serialization buffers;
- selected fingerprint differs from Bitcoin-derived fingerprint;
- primary and temporary seeds have overlapping account indices;
- temporary seed is not a member of an imported multisig;
- passphrase application/clear across seed changes;
- every QR, file, QuantumLink, and supported NFC route uses the selected seed or fails closed.
