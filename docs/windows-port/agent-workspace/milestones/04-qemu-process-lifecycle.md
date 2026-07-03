# M04: QEMU Process Lifecycle and Cleanup

Status: In progress
Depends on: See `00-index.md`
RFC sections: See `traceability.md`

## Objective

Start, supervise, stop, and kill QEMU reliably on Windows.

## Scope

- Implement process launch wrapper using argv from M03.
- Capture stdout/stderr and serial artifacts.
- Use Windows Job Objects or equivalent cleanup so QEMU/helper processes are terminated.
- Implement graceful shutdown path and forced kill fallback.
- Add timeouts and structured errors.

## Out of scope

- Do not require successful guest boot.
- Do not implement guest protocol.
- Do not expose QMP publicly.
- Do not leave orphan QEMU processes in tests.

## Likely files / crates

- `crates/lsb-platform/src/windows_x86_64/qemu/process.rs`
- `errors.rs`
- `diagnostics artifact helpers`

## Design notes

- Preserve existing macOS behavior unless the milestone explicitly states otherwise.
- Keep Windows-specific implementation behind platform/backend boundaries.
- Prefer precise capability errors over silent degradation.
- Update `state.md` when implementation reveals a better file layout or dependency.

## Tests to add or update

The specific tests should match the implementation, but this milestone must include enough validation to satisfy the acceptance criteria below. Prefer unit/golden/fake tests before requiring self-hosted integration tests.

## Acceptance criteria

- [ ] Fake process tests for timeout/kill.
- [ ] Windows integration test for cleanup if possible.
- [ ] Failure captures redacted argv and logs.
- [ ] Process lifecycle works with a harmless command before QEMU-specific smoke.

## Coding-agent prompt

```text
You are implementing M04: QEMU Process Lifecycle and Cleanup for the LocalSandbox Windows QEMU + WHPX port.

Read first:
- docs/windows-port/rfc-qemu-whpx.md
- docs/windows-port/AGENTS.md
- docs/windows-port/agent-workspace/state.md
- docs/windows-port/agent-workspace/decisions.md
- docs/windows-port/agent-workspace/milestones/04-qemu-process-lifecycle.md

Implement only this milestone. Preserve public CLI/SDK/Node APIs and existing macOS behavior. Add tests required by the milestone. Do not implement later milestones opportunistically. Update state.md and this milestone handoff before finishing.
```

## Security checklist

Complete the checklist in `../security-checklist.md`. Record any new risk in `../risk-register.md`.

## Handoff

- Branch/PR: `codex/windows-m04-qemu-lifecycle`
- Summary: TBD
- Tests run: TBD
- Debug artifacts: TBD
- New decisions: TBD
- New risks: TBD
- Next milestone: TBD
