# Foundation inquiry: Vault-backed temporary signing

Status: Historical draft. The maintainer reported sending an inquiry and supplied Foundation's response. The exact sent message is not independently verified here; see the [technical response summary and project decision](foundation-status.md).

Prepared: 2026-08-25

## Original email draft

To: hello@foundation.xyz

Subject: Passport Prime SDK feedback — Vault-backed temporary signing

Hello Foundation team,

I'm BitZk, and I'm exploring a small open-source project called [Vault Signer](https://github.com/BitZk/passport-prime-vault-signer) for Passport Prime.

The use case is to select an additional Bitcoin seed already stored in Prime's built-in Vault and temporarily use it with the built-in Bitcoin Wallet, without replacing the primary seed. I'd like to reuse the existing Vault, wallet, transaction-review screens, and supported signing transports rather than build a separate vault or Bitcoin signer.

Your [Bitcoin Wallet guide](https://docs.foundation.xyz/prime/prime-apps/wallet/) labels the temporary-seed feature as coming soon and describes loading from Vault. That appears to match this goal closely, so I'd like to check with you before duplicating work.

Could you help clarify:

1. Is this already available in a release or beta, actively being developed, or planned? Is there a public issue, milestone, or firmware version I should follow?
2. Is the intended scope to support both imported and Vault-generated Bitcoin seeds through the existing wallet, including QR, file import/export, and QuantumLink? Are there limitations around multisig, passphrases, message signing, or planned NFC support?
3. If work is underway, would documentation, simulator tests, or UX feedback be useful? If there is a gap for contributors, what is the smallest useful contribution and preferred integration point? I would prefer your existing APIs and data structures over introducing parallel ones.
4. What contribution process and security review checklist should I follow before proposing anything that handles seed material?

The public repository currently contains an SDK-based, simulator-safe UI prototype, a non-secret protocol model, and design notes. It cannot access or sign with seeds. Its architecture is exploratory, not a request for you to adopt a particular KeyOS redesign. I have not tested this feature on current device firmware, and my review of an older public source snapshot is not evidence that your team lacks an implementation.

My priorities are clear seed selection, explicit confirmation, preserving the primary seed, a well-defined temporary-session lifecycle, and avoiding secret material in app-to-app messages or public project artifacts. If your native feature already covers the use case, I'd be happy to align this project with it and help where useful.

Thank you,

BitZk

## Maintainer notes — not part of the email

- **Why ask first:** the live [wallet guide](https://docs.foundation.xyz/prime/prime-apps/wallet/) explicitly marks the feature as forthcoming. This supports a planned-product interpretation, not a claim that it is an overlooked feature. It does not establish implementation status, release timing, or availability on any particular device.
- **Transport scope:** that guide currently documents transaction signing through QuantumLink, QR, and files. It describes NFC message signing as future functionality; the draft asks about NFC without claiming it is available today.
- **Contact:** the [Developer FAQ](https://docs.foundation.xyz/developers/faq/) gives the recipient and SDK-feedback subject above and asks wallet developers to request a security review checklist before shipping. Both pages were checked on the preparation date.
- **Validation snapshots:** SDK 1.0.0, bundled workspace commit `a9ce6713949a5d6bb8f5edfca19bcaabc946f09d`; separate public KeyOS research snapshot v1.3.1, commit `de966a11e88d28f116b52509679c19eb33591711`. See [Validation](validation.md). These are not claims about the latest device firmware.
- **Privacy:** send only the email section and public repository link. Do not attach runtime logs, local SDK mappings, device screenshots, wallet data, or signing material. This inquiry contains no such data.
- **Outcome:** Foundation confirmed the planned native feature and unsupported third-party handoff. Functional implementation and hardware work are paused; the [status record](foundation-status.md) defines possible design/test contributions and resume criteria.

This file is retained as an inquiry record, not an instruction to send or resend the message. Further outreach, upstream submissions, and any later hardware or signing work require separate authorization. Foundation's private reply is not reproduced here.
