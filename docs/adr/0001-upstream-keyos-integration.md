# ADR 0001: Implement production behavior upstream in KeyOS

- Status: Accepted as internal project direction; implementation deferred, not Foundation-approved
- Date: 2026-08-25

## Context

The requested behavior selects a seed from Foundation's built-in Vault and uses all functionality in Foundation's built-in Bitcoin Wallet. KeyOS prevents ordinary apps from reading another app's storage, and sensitive master-seed/NFC operations are Foundation-reserved.

## Decision

Any future implementation contribution must align with Foundation's native feature or a supported integration boundary, rather than bypassing app isolation. The standalone Vault Signer SDK app remains a permission-minimal UX and protocol prototype and will not implement a duplicate seed store or wallet.

Following Foundation's response, functional implementation and hardware work are paused. The response confirms the platform boundary, not this repository's proposed architecture. The [status record](../foundation-status.md) governs what may proceed and the conditions to resume.

## Consequences

- Foundation review and signing are required before release.
- Existing Bitcoin behavior and transports can be reused rather than reimplemented.
- The project cannot advertise a working signing app during the design milestone.
- Upstream implementation files must use KeyOS's GPL-3.0-or-later licensing and contribution rules.
