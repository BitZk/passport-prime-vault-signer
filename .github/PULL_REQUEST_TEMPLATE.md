## Summary

Describe the smallest user-visible or architectural outcome.

## Security impact

- Threat-model assumptions changed:
- New permissions or secret-bearing types:
- Failure and cancellation behavior:
- Why this fails closed:

## Validation

- [ ] `./scripts/check-public-tree.sh`
- [ ] `./scripts/check-spdx.sh`
- [ ] `git diff --check`
- [ ] Rust tests, or explanation if the Foundation SDK was unavailable
- [ ] `foundation sim`, or explanation if not applicable
- [ ] No real wallet, device, signing, or personal data is included
- [ ] Documentation and ADRs are updated
- [ ] DCO sign-off is present

## Boundaries

State whether evidence comes from static review, unit tests, simulator, or physical Passport Prime hardware. Do not imply Foundation approval.
