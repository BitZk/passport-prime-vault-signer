# Data model

## Existing Vault storage

KeyOS v1.3.1 stores Vault entries in the Vault app's private filesystem scope. Its public source models generated BIP85 Bitcoin seeds, imported BIP39 mnemonics, passwords, and Nostr keys in `apps/gui-app-seed-vault/src/seed.rs`; persistence is coordinated by `apps/gui-app-seed-vault/src/state.rs`.

Vault Signer must not create a second copy of that database, reach into the Vault filesystem, or depend directly on its JSON representation. The Vault remains the source of truth and resolves an entry internally after user selection.

## Cross-app request

The dependency-free host model in `src/protocol.rs` proposes the following contract. It is not yet a KeyOS wire-format implementation; serialization and IPC integration require Foundation review.

```text
SelectVaultSeedRequest {
    protocol_version: u16,
    request_id: [u8; 16]
}
```

`request_id` is generated for each selection and binds the eventual result to the initiating Bitcoin Wallet instance. It is not a secret.

## Cross-app result

On success:

```text
Activated {
    protocol_version: u16,
    request_id: [u8; 16],
    session_id: [u8; 16],
    descriptor: {
        label: String,
        fingerprint: [u8; 4],
        word_count: Twelve | TwentyFour
    }
}
```

The result also supports explicit cancellation and typed rejection. The result contains no mnemonic, entropy, seed QR, private key, passphrase, xprv, or signing key.

`session_id` is an opaque, non-persistent handle issued by the security service. It must be unguessable, single-device, single-boot, bound to the Bitcoin Wallet caller, and invalid after clear, lock if adopted, shutdown, restart, or activation failure.

The label is display metadata and should be length-bounded. The fingerprint is the standard four-byte Bitcoin master fingerprint and must be recomputed from the activated seed by the trusted implementation rather than accepted from persistent metadata.

## Temporary secret state

Seed entropy is held only inside Foundation-controlled processes that already require it:

- Vault while resolving the selected entry;
- the KeyOS security service while the temporary session is active;
- the Bitcoin Wallet only while deriving/signing, subject to the existing `ngwallet` design.

Every owning type must zeroize on drop and every error/cancellation path. It must not implement a revealing `Debug`, `Display`, serialization, or logging representation.

## Bitcoin account metadata

Existing Bitcoin account configuration may be keyed by master fingerprint and remain persisted without private key material. Before adopting that behavior for temporary seeds, Foundation should decide:

- whether temporary-seed account metadata is backed up;
- whether it remains hidden when the seed is inactive;
- how name/index collisions are handled across fingerprints;
- whether Envoy should retain or forget the associated watch-only account;
- whether a passphrase view is scoped to the temporary seed session.

No choice should be inferred from the current primary-seed behavior without tests and product review.
