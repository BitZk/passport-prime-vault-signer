# ADR 0002: Keep entropy out of cross-app navigation

- Status: Proposed, pending Foundation review
- Date: 2026-08-25

## Context

KeyOS GUI navigation can carry serialized request and response buffers between apps. Returning mnemonic entropy from Vault to Bitcoin through that generic route would create extra copies in Vault, GUI-server, serialization, caller, error, and tracing buffers.

## Decision

Vault will ask a Foundation-controlled security service to activate the chosen seed and receive an opaque, boot-bound session ID. Cross-app navigation returns that ID plus non-secret display metadata. Bitcoin requests the active signing seed through a dedicated Foundation-only security API.

The primary device seed is never overwritten. Temporary state is in memory, caller-bound, and cleared on manual clear and shutdown. Clearing on lock is the default proposal.

## Consequences

- The security service gains a sensitive lifecycle and requires extensive review.
- The navigation contract is safer to log structurally because it contains no entropy, though identifiers still should not be logged unnecessarily.
- Bitcoin must handle ambiguous activation/clear status by disabling signing.
- A smaller raw-entropy navigation patch is explicitly rejected unless Foundation demonstrates an equivalent or stronger containment design.
