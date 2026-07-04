# M08: Exec Command

Status: In progress
Depends on: See `00-index.md`
RFC sections: See `traceability.md`

## Objective

Run a command in the Windows-hosted Linux guest and return stdout, stderr, and exit status through existing LocalSandbox APIs.

## Scope

- Wire `Sandbox::exec` or equivalent through Windows backend.
- Preserve existing exec request/response semantics.
- Handle stdout/stderr streaming and backpressure.
- Support timeout/kill behavior consistent with existing product behavior.
- Add basic environment handling with secret redaction.

## Out of scope

- Do not implement mounts beyond what exec needs.
- Do not enable guest networking.
- Do not copy host secrets into the guest except approved placeholders.

## Likely files / crates

- `crates/lsb-vm/src/sandbox.rs`
- `crates/lsb-guest` exec handler
- `Windows control transport`

## Design notes

- Preserve existing macOS behavior unless the milestone explicitly states otherwise.
- Keep Windows-specific implementation behind platform/backend boundaries.
- Prefer precise capability errors over silent degradation.
- Update `state.md` when implementation reveals a better file layout or dependency.

## Tests to add or update

The specific tests should match the implementation, but this milestone must include enough validation to satisfy the acceptance criteria below. Prefer unit/golden/fake tests before requiring self-hosted integration tests.

## Acceptance criteria

- [ ] Exec `true`, `echo`, failing command, large stdout, stderr, timeout/kill tests.
- [ ] Exit status preserved.
- [ ] No guest NIC in exec smoke argv.
- [ ] Public API unchanged.

## Coding-agent prompt

```text
You are implementing M08: Exec Command for the LocalSandbox Windows QEMU + WHPX port.

Read first:
- docs/windows-port/rfc-qemu-whpx.md
- docs/windows-port/AGENTS.md
- docs/windows-port/agent-workspace/state.md
- docs/windows-port/agent-workspace/decisions.md
- docs/windows-port/agent-workspace/milestones/08-exec-command.md

Implement only this milestone. Preserve public CLI/SDK/Node APIs and existing macOS behavior. Add tests required by the milestone. Do not implement later milestones opportunistically. Update state.md and this milestone handoff before finishing.
```

## Security checklist

Complete the checklist in `../security-checklist.md`. Record any new risk in `../risk-register.md`.

## Handoff

- Branch/PR: TBD
- Summary: In progress on `codex/windows-m08-exec-command`.
- Tests run: TBD
- Debug artifacts: TBD
- New decisions: TBD
- New risks: TBD
- Next milestone: TBD
