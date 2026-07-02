# Review Checklist for Windows Port PRs

Use this checklist for every milestone PR.

## Scope

- [ ] PR implements the assigned milestone only.
- [ ] PR does not silently implement later milestones.
- [ ] PR references the milestone document.
- [ ] `state.md` is updated.
- [ ] Milestone handoff is filled in.

## Architecture

- [ ] Windows/QEMU details stay below platform/backend boundaries.
- [ ] Public CLI/SDK/Node APIs are unchanged, or an accepted decision allows the change.
- [ ] Existing macOS behavior is preserved.
- [ ] New abstractions describe LocalSandbox product needs, not incidental QEMU details.

## Security

- [ ] No-network default preserved.
- [ ] No secrets in argv, logs, tests, snapshots, or debug artifacts.
- [ ] Host listeners are loopback/private only.
- [ ] QMP/control endpoints are private.
- [ ] Host filesystem exposure is minimized.
- [ ] Direct `:rw` host mounts remain unsupported on Windows MVP.
- [ ] Failure modes fail closed.

## Testing

- [ ] Required milestone tests were added.
- [ ] Tests are deterministic or explicitly marked ignored/integration.
- [ ] Golden outputs are redacted and stable.
- [ ] Windows-only tests are cfg-gated correctly.
- [ ] Hosted CI does not require WHPX.
- [ ] Self-hosted WHPX tests are isolated from normal hosted jobs.

## Diagnostics

- [ ] Errors name likely cause and remediation.
- [ ] QEMU failures include redacted argv where relevant.
- [ ] Boot/control failures capture serial or protocol logs where relevant.
- [ ] Debug artifacts are documented and redacted.

## Documentation

- [ ] `state.md` updated.
- [ ] `decisions.md` updated only if a new accepted decision exists.
- [ ] `risk-register.md` updated for new/retired risks.
- [ ] `validation.md` updated for new test commands or CI behavior.
- [ ] Milestone handoff includes next recommended milestone.
