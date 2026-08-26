# Roadmap

## Current position — implementation paused

Foundation confirmed that native temporary-seed support is planned and that the required third-party interface is unavailable. Final scope and timing remain unconfirmed in the response. See [Foundation coordination and project status](foundation-status.md).

Phase 0 is established. Phase 1 has answered the product-status and platform-boundary questions but has not approved an integration design. Non-secret UX/model review, documentation, and test proposals remain possible; phases 2–5 are conditional reference material, not active work or a promise to implement Foundation's feature.

Do not resume functional or hardware work until the [resume criteria](foundation-status.md#conditions-to-resume) are met. Foundation's own implementation may remove the need for some or all of these phases.

## Phase 0 — public project foundation

- SDK-conformant simulator UX shell.
- Stable app ID and minimal permissions.
- Architecture, data model, threat model, and upstream decision.
- Secret-safe Git ignores, CI checks, contribution templates, and disclosure policy.
- Exact Foundation source and SDK research record.

Exit: repository checks pass and no production capability is claimed.

## Phase 1 — Foundation design alignment

- Completed: Foundation confirmed the planned native feature and unsupported third-party handoff; no public release or beta was available at the time of its response.
- Pending: identify a supported interface or agree on a specific useful contribution without duplicating native work.
- Before any resumed implementation: obtain the applicable wallet/security review checklist and agree on ownership, lock behavior, account metadata, backups, Envoy state, and transport scope.
- Record design decisions separately from Foundation's approval; existing ADRs are project proposals or internal direction only.

Exit: the documented resume criteria are met and the maintainer approves a bounded milestone, or the project remains design/test input to Foundation's native feature.

## Conditional phase 2 — protocol and lifecycle

The session-service approach below is one unapproved proposal. Replace or retire it if Foundation's supported design differs.

- Add typed Vault navigation request/result types to a pinned KeyOS branch.
- Add opaque, caller-bound, boot-bound temporary seed sessions to the security service.
- Test version mismatch, spoofing, replay, cancellation, interruption, clear, restart, and lock semantics.

Exit: no seed entropy crosses the navigation protocol; lifecycle tests pass.

## Conditional phase 3 — Vault and Bitcoin integration

- Add Bitcoin-only Vault selection UI with label/fingerprint confirmation.
- Add active seed-source handling to Bitcoin account derivation and signing.
- Reuse existing Bitcoin QR, file, QuantumLink, message, PSBT, multisig, passphrase, account, and address-verification paths.
- Display unmistakable temporary-seed state and provide manual clear.

Exit: full simulator suite passes for primary and temporary seeds, including negative paths.

## Conditional phase 4 — device validation

Requires separate explicit hardware/signing authorization; the pause is not lifted by a successful simulator run.

- Build reproducibly from the reviewed full SHA.
- Use only disposable regtest seeds and transactions.
- Validate camera/QR, files, QuantumLink, every officially supported NFC path, lock, shutdown, power loss, crash, and recovery.
- Inspect logs, crash reports, backups, storage, and artifacts for secret leakage.

Exit: documented hardware evidence and no unresolved high-severity issue.

## Conditional phase 5 — upstream and release readiness

- Independent security and cryptographic review.
- Upstream KeyOS pull requests with small commits and complete test evidence.
- Foundation product, UX, localization, accessibility, documentation, and trademark review.
- Reproducible artifact provenance and release checklist.

Exit: if an implementation contribution is agreed, Foundation reviews, merges, and releases it through its own process. No release or endorsement is promised. This repository remains a non-signing research prototype unless its status is explicitly revised after review.
