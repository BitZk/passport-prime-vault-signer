# Roadmap

## Phase 0 — public project foundation

- SDK-conformant simulator UX shell.
- Stable app ID and minimal permissions.
- Architecture, data model, threat model, and upstream decision.
- Secret-safe Git ignores, CI checks, contribution templates, and disclosure policy.
- Exact Foundation source and SDK research record.

Exit: repository checks pass and no production capability is claimed.

## Phase 1 — Foundation design alignment

- Confirm whether the built-in documented flow is already implemented privately or planned.
- Obtain Foundation's wallet/security review checklist.
- Agree on security-service ownership, lock behavior, account metadata, backups, Envoy state, and NFC scope.
- Convert open questions into accepted ADRs.

Exit: Foundation accepts or redirects the proposed integration boundary.

## Phase 2 — protocol and lifecycle

- Add typed Vault navigation request/result types to a pinned KeyOS branch.
- Add opaque, caller-bound, boot-bound temporary seed sessions to the security service.
- Test version mismatch, spoofing, replay, cancellation, interruption, clear, restart, and lock semantics.

Exit: no seed entropy crosses the navigation protocol; lifecycle tests pass.

## Phase 3 — Vault and Bitcoin integration

- Add Bitcoin-only Vault selection UI with label/fingerprint confirmation.
- Add active seed-source handling to Bitcoin account derivation and signing.
- Reuse existing Bitcoin QR, file, QuantumLink, message, PSBT, multisig, passphrase, account, and address-verification paths.
- Display unmistakable temporary-seed state and provide manual clear.

Exit: full simulator suite passes for primary and temporary seeds, including negative paths.

## Phase 4 — device validation

- Build reproducibly from the reviewed full SHA.
- Use only disposable regtest seeds and transactions.
- Validate camera/QR, files, QuantumLink, every officially supported NFC path, lock, shutdown, power loss, crash, and recovery.
- Inspect logs, crash reports, backups, storage, and artifacts for secret leakage.

Exit: documented hardware evidence and no unresolved high-severity issue.

## Phase 5 — upstream and release readiness

- Independent security and cryptographic review.
- Upstream KeyOS pull requests with small commits and complete test evidence.
- Foundation product, UX, localization, accessibility, documentation, and trademark review.
- Reproducible artifact provenance and release checklist.

Exit: Foundation merges and releases the feature. Until then, Vault Signer remains experimental.
