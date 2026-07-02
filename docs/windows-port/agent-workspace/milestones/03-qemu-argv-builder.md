# M03: QEMU Argv Builder

Status: Not started
Depends on: See `00-index.md`
RFC sections: See `traceability.md`

## Objective

Create deterministic, testable QEMU argv construction for the Windows backend.

## Scope

- Build minimal direct Linux boot argv.
- Add rootfs virtio-blk args.
- Add serial console log args.
- Add virtio-serial control chardev args.
- Add private QMP endpoint args if needed for lifecycle diagnostics.
- Ensure default argv contains no guest NIC.
- Provide redacted argv rendering.

## Out of scope

- Do not spawn QEMU.
- Do not implement process cleanup.
- Do not enable QEMU user networking by default.
- Do not include secrets in argv.

## Likely files / crates

- `crates/lsb-platform/src/windows_x86_64/qemu/argv.rs`
- `tests under `crates/lsb-platform/tests/` or module tests`

## Design notes

- Preserve existing macOS behavior unless the milestone explicitly states otherwise.
- Keep Windows-specific implementation behind platform/backend boundaries.
- Prefer precise capability errors over silent degradation.
- Update `state.md` when implementation reveals a better file layout or dependency.

## Tests to add or update

The specific tests should match the implementation, but this milestone must include enough validation to satisfy the acceptance criteria below. Prefer unit/golden/fake tests before requiring self-hosted integration tests.

## Acceptance criteria

- [ ] Golden tests for minimal boot argv.
- [ ] Golden tests for virtio-serial + QMP argv.
- [ ] Golden test proving no network device by default.
- [ ] Path quoting/escaping tests for Windows paths.
- [ ] Redacted argv test.

## Coding-agent prompt

```text
You are implementing M03: QEMU Argv Builder for the LocalSandbox Windows QEMU + WHPX port.

Read first:
- docs/windows-port/rfc-qemu-whpx.md
- docs/windows-port/AGENTS.md
- docs/windows-port/agent-workspace/state.md
- docs/windows-port/agent-workspace/decisions.md
- docs/windows-port/agent-workspace/milestones/03-qemu-argv-builder.md

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
