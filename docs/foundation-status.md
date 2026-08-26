# Foundation coordination and project status

Recorded: 2026-08-25

Status: Functional seed handling and hardware integration are paused. The simulator-safe research prototype is retained.

## Source and limits

This is a technical summary of Foundation's response supplied by the maintainer, not a reproduction of private correspondence. The maintainer reported sending the inquiry and provided the reply. Sender details, mail headers, and the original reply are not included.

The statements below reflect that response, not an independent check of current device firmware. There is no confirmed release date, final feature scope, approved project design, or implementation assignment.

## What Foundation confirmed

- Temporary-seed use is planned for the native Bitcoin Wallet, with manual entry, SeedQR, and selection from Vault as intended entry paths. At the time of the response, it was not available in a public release or beta; final scope and timing remained unconfirmed.
- App-private storage isolation prevents an ordinary SDK app from reading the built-in Vault's records.
- The SDK's app-specific seed is a deterministic secret for the calling app. It is neither the device's Bitcoin seed nor a way to retrieve a stored Vault seed.
- Generic app launch and navigation facilities do not provide a supported third-party request for selecting a Vault seed or activating it in Bitcoin Wallet.
- A functional handoff needs coordinated native work, narrow authorization, and user confirmation, with seed material confined to trusted Foundation components. Foundation advised against pursuing hardware integration without that interface.
- UX and protocol-design input, documentation, and test proposals remain useful potential contributions. This is not approval of the repository's proposed session API or permission changes.

## Project decision

Preserve the existing SDK UI shell, non-secret protocol model, threat model, and validation evidence. Do not implement seed storage, seed handoff, signing, new sensitive permissions, or hardware integration while this work is paused.

An independent vault would address storage only; it would not create a supported connection to the native Bitcoin Wallet. The project will not duplicate a vault or wallet to work around the missing interface.

The existing architecture and ADRs are exploratory project material. They do not describe a supported SDK contract, Foundation's final implementation, or guaranteed transport parity.

## Useful work during the pause

- Review synthetic seed-identification and confirmation UX.
- Refine non-secret models and test proposals for activation, cancellation, session lifetime, ambiguous state, and clearing.
- Maintain documentation and repository publication safeguards.
- Discuss bounded contributions with Foundation when there is a specific question or proposal.

These options do not authorize additional work automatically. Keep real wallet data, runtime logs, device identifiers, and signing material out of public contributions. Do not publish private correspondence verbatim.

## Conditions to resume

1. Foundation publishes a supported interface that covers this use case, or agrees to a specific native contribution with an agreed integration boundary. A general welcome for design feedback is not an implementation assignment.
2. Recheck the applicable SDK/KeyOS snapshot and native feature scope. If the native feature already meets the need, prefer testing or documentation over a parallel implementation.
3. The maintainer explicitly approves a bounded next milestone after reviewing ownership, permissions, data structures, session behavior, and the required security review process.

Hardware, signing identities, and secret-handling work still need separate explicit authorization and an applicable review/test plan. A simulator build or a new SDK version alone does not lift the pause.

There is no scheduled monitoring or follow-up automation. Reassess when Foundation provides new guidance or the maintainer requests a review.

## Preserved validation baseline

Prior development validation used Foundation SDK **1.0.0**, bundled workspace commit `a9ce6713949a5d6bb8f5edfca19bcaabc946f09d`. Separate source research used KeyOS **v1.3.1**, commit `de966a11e88d28f116b52509679c19eb33591711`. See the [validation record](validation.md) for completed checks and their limits; this status update does not add hardware or signing validation.
