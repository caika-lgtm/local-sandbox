# M07: Guest Ready Handshake

Status: Not started
Depends on: See `00-index.md`
RFC sections: See `traceability.md`

## Objective

Make VM readiness deterministic and observable.

## Scope

- Define a ready message/version/capability handshake over `lsb-proto`.
- Host waits for ready before mount/exec operations.
- Expose guest version/capabilities for diagnostics.
- Implement timeout and failure reporting with serial log hints.

## Out of scope

- Do not implement full exec yet unless existing protocol requires minimal command for handshake.
- Do not make readiness depend on guest networking.
- Do not hide boot logs on timeout.

## Likely files / crates

- `crates/lsb-proto` if new ready frame is needed
- `crates/lsb-guest`
- `crates/lsb-vm` sandbox startup
- `Windows control transport`

## Design notes

- Preserve existing macOS behavior unless the milestone explicitly states otherwise.
- Keep Windows-specific implementation behind platform/backend boundaries.
- Prefer precise capability errors over silent degradation.
- Update `state.md` when implementation reveals a better file layout or dependency.

## Tests to add or update

The specific tests should match the implementation, but this milestone must include enough validation to satisfy the acceptance criteria below. Prefer unit/golden/fake tests before requiring self-hosted integration tests.

## Acceptance criteria

- [ ] Ready succeeds on boot smoke.
- [ ] Wrong/missing ready message times out cleanly.
- [ ] Capabilities are logged without secrets.
- [ ] macOS path either uses same handshake or remains compatible by explicit design.

## Coding-agent prompt

```text
You are implementing M07: Guest Ready Handshake for the LocalSandbox Windows QEMU + WHPX port.

Read first:
- docs/windows-port/rfc-qemu-whpx.md
- docs/windows-port/AGENTS.md
- docs/windows-port/agent-workspace/state.md
- docs/windows-port/agent-workspace/decisions.md
- docs/windows-port/agent-workspace/milestones/07-guest-ready-handshake.md

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
