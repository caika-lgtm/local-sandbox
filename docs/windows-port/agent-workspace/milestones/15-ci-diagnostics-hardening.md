# M15: CI and Diagnostics Hardening

Status: In progress
Depends on: See `00-index.md`
RFC sections: See `traceability.md`

## Objective

Harden the Windows port with hosted compile/golden CI, self-hosted WHPX smoke tests, and useful artifacts.

## Scope

- Add hosted Windows jobs for compile/unit/golden tests.
- Add self-hosted Windows 11 WHPX jobs for ignored/integration tests.
- Upload redacted debug artifacts on failure.
- Document runner setup and labels.
- Add diagnostic commands to docs.

## Out of scope

- Do not require WHPX on hosted runners.
- Do not upload secrets or unredacted env.
- Do not make flaky boot smoke block unrelated hosted unit tests.

## Likely files / crates

- `.github/workflows/*`
- `test helpers`
- `diagnostics docs`
- `runner setup docs`

## Design notes

- Preserve existing macOS behavior unless the milestone explicitly states otherwise.
- Keep Windows-specific implementation behind platform/backend boundaries.
- Prefer precise capability errors over silent degradation.
- Update `state.md` when implementation reveals a better file layout or dependency.

## Tests to add or update

The specific tests should match the implementation, but this milestone must include enough validation to satisfy the acceptance criteria below. Prefer unit/golden/fake tests before requiring self-hosted integration tests.

## Acceptance criteria

- [ ] Hosted Windows compile/golden job passes.
- [ ] Self-hosted WHPX boot smoke job passes or is correctly gated.
- [ ] Failure artifacts include redacted argv and serial logs.
- [ ] Runner setup documented.

## Coding-agent prompt

```text
You are implementing M15: CI and Diagnostics Hardening for the LocalSandbox Windows QEMU + WHPX port.

Read first:
- docs/windows-port/rfc-qemu-whpx.md
- docs/windows-port/AGENTS.md
- docs/windows-port/agent-workspace/state.md
- docs/windows-port/agent-workspace/decisions.md
- docs/windows-port/agent-workspace/milestones/15-ci-diagnostics-hardening.md

Implement only this milestone. Preserve public CLI/SDK/Node APIs and existing macOS behavior. Add tests required by the milestone. Do not implement later milestones opportunistically. Update state.md and this milestone handoff before finishing.
```

## Security checklist

Complete the checklist in `../security-checklist.md`. Record any new risk in `../risk-register.md`.

## Handoff

- Branch/PR: TBD
- Summary: TBD
- Tests run: TBD
- Debug artifacts: TBD
- New decisions: TBD
- New risks: TBD
- Next milestone: TBD
