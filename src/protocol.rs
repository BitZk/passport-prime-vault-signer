// SPDX-FileCopyrightText: 2026 BitZk contributors
// SPDX-License-Identifier: MIT

//! Non-secret model for a proposed Vault-to-Bitcoin activation flow.
//!
//! Seed entropy is deliberately absent. The proposed Foundation-controlled
//! security service activates and owns the temporary seed; cross-app
//! navigation returns only display metadata and opaque identifiers.

pub const PROTOCOL_VERSION: u16 = 1;

pub type RequestId = [u8; 16];
pub type SessionId = [u8; 16];
pub type MasterFingerprint = [u8; 4];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeedWordCount {
    Twelve,
    TwentyFour,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectVaultSeedRequest {
    pub protocol_version: u16,
    pub request_id: RequestId,
}

impl SelectVaultSeedRequest {
    pub const fn new(request_id: RequestId) -> Self {
        Self { protocol_version: PROTOCOL_VERSION, request_id }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeedDescriptor {
    pub label: String,
    pub fingerprint: MasterFingerprint,
    pub word_count: SeedWordCount,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectVaultSeedResult {
    Activated {
        protocol_version: u16,
        request_id: RequestId,
        session_id: SessionId,
        descriptor: SeedDescriptor,
    },
    Canceled {
        protocol_version: u16,
        request_id: RequestId,
    },
    Rejected {
        protocol_version: u16,
        request_id: RequestId,
        reason: RejectionReason,
    },
}

impl SelectVaultSeedResult {
    pub fn matches(&self, request: &SelectVaultSeedRequest) -> bool {
        let (version, request_id) = match self {
            Self::Activated { protocol_version, request_id, .. }
            | Self::Canceled { protocol_version, request_id }
            | Self::Rejected { protocol_version, request_id, .. } => (*protocol_version, request_id),
        };

        version == PROTOCOL_VERSION
            && request.protocol_version == PROTOCOL_VERSION
            && request_id == &request.request_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RejectionReason {
    IncompatibleVersion,
    NoBitcoinSeeds,
    ActivationFailed,
    StaleRequest,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SessionState {
    #[default]
    Primary,
    Activating { request_id: RequestId },
    Temporary { session_id: SessionId, fingerprint: MasterFingerprint },
    Clearing { session_id: SessionId },
}

impl SessionState {
    /// A new seed may be selected only after the previous session is cleared.
    pub fn begin_activation(&mut self, request_id: RequestId) -> Option<SelectVaultSeedRequest> {
        if *self != Self::Primary {
            return None;
        }
        *self = Self::Activating { request_id };
        Some(SelectVaultSeedRequest::new(request_id))
    }

    pub fn apply_result(
        &mut self,
        request: &SelectVaultSeedRequest,
        result: &SelectVaultSeedResult,
    ) -> bool {
        if *self != (Self::Activating { request_id: request.request_id }) || !result.matches(request) {
            return false;
        }

        *self = match result {
            SelectVaultSeedResult::Activated { session_id, descriptor, .. } => Self::Temporary {
                session_id: *session_id,
                fingerprint: descriptor.fingerprint,
            },
            SelectVaultSeedResult::Canceled { .. } | SelectVaultSeedResult::Rejected { .. } => Self::Primary,
        };
        true
    }

    /// Disable signing first; do not claim Primary until KeyOS confirms clear.
    pub fn begin_clear(&mut self) -> Option<SessionId> {
        let Self::Temporary { session_id, .. } = *self else {
            return None;
        };
        *self = Self::Clearing { session_id };
        Some(session_id)
    }

    pub fn confirm_cleared(&mut self, session_id: SessionId) -> bool {
        if *self != (Self::Clearing { session_id }) {
            return false;
        }
        *self = Self::Primary;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor() -> SeedDescriptor {
        SeedDescriptor {
            label: "Synthetic vault entry".to_string(),
            fingerprint: [0, 0, 0, 0],
            word_count: SeedWordCount::Twelve,
        }
    }

    #[test]
    fn rejects_a_response_bound_to_another_request() {
        let mut state = SessionState::Primary;
        let request = state.begin_activation([1; 16]).unwrap();
        let result = SelectVaultSeedResult::Activated {
            protocol_version: PROTOCOL_VERSION,
            request_id: [2; 16],
            session_id: [3; 16],
            descriptor: descriptor(),
        };

        assert!(!state.apply_result(&request, &result));
        assert_eq!(state, SessionState::Activating { request_id: [1; 16] });
    }

    #[test]
    fn activation_records_only_opaque_and_display_metadata() {
        let mut state = SessionState::Primary;
        let request = state.begin_activation([1; 16]).unwrap();
        let result = SelectVaultSeedResult::Activated {
            protocol_version: PROTOCOL_VERSION,
            request_id: request.request_id,
            session_id: [2; 16],
            descriptor: descriptor(),
        };

        assert!(state.apply_result(&request, &result));
        assert_eq!(
            state,
            SessionState::Temporary { session_id: [2; 16], fingerprint: [0, 0, 0, 0] }
        );
    }

    #[test]
    fn clear_requires_authoritative_confirmation() {
        let mut state = SessionState::Temporary { session_id: [2; 16], fingerprint: [0; 4] };
        assert_eq!(state.begin_clear(), Some([2; 16]));
        assert_eq!(state, SessionState::Clearing { session_id: [2; 16] });
        assert!(!state.confirm_cleared([3; 16]));
        assert!(state.confirm_cleared([2; 16]));
        assert_eq!(state, SessionState::Primary);
    }

    #[test]
    fn cannot_replace_an_active_session_without_clearing() {
        let mut state = SessionState::Temporary { session_id: [2; 16], fingerprint: [0; 4] };
        assert!(state.begin_activation([1; 16]).is_none());
    }

    #[test]
    fn rejects_an_incompatible_protocol_version() {
        let mut state = SessionState::Primary;
        let request = state.begin_activation([1; 16]).unwrap();
        let result = SelectVaultSeedResult::Canceled {
            protocol_version: PROTOCOL_VERSION + 1,
            request_id: request.request_id,
        };
        assert!(!state.apply_result(&request, &result));
    }

    #[test]
    fn rejects_replay_after_success() {
        let mut state = SessionState::Primary;
        let request = state.begin_activation([1; 16]).unwrap();
        let result = SelectVaultSeedResult::Activated {
            protocol_version: PROTOCOL_VERSION,
            request_id: request.request_id,
            session_id: [2; 16],
            descriptor: descriptor(),
        };
        assert!(state.apply_result(&request, &result));
        assert!(!state.apply_result(&request, &result));
    }
}
