# M05: Direct Linux Boot and Serial Logs

Status: In progress
Depends on: See `00-index.md`
RFC sections: See `traceability.md`

## Objective

Boot the Linux guest under QEMU + WHPX and capture enough logs to diagnose boot/init failures.

## Scope

- Use existing kernel/initramfs/rootfs assets.
- Boot with `-accel whpx` in production path.
- Pass direct Linux boot args compatible with rootfs virtio-blk.
- Capture serial console to file and/or host stream.
- Define boot timeout and artifact preservation.

## Out of scope

- Do not implement LocalSandbox exec.
- Do not enable guest NIC.
- Do not implement mounts/checkpoints.
- Do not mask boot failures as success.

## Likely files / crates

- `QEMU backend boot config`
- `kernel` config only if validation requires it
- `diagnostics artifacts`

## Design notes

- Preserve existing macOS behavior unless the milestone explicitly states otherwise.
- Keep Windows-specific implementation behind platform/backend boundaries.
- Prefer precise capability errors over silent degradation.
- Update `state.md` when implementation reveals a better file layout or dependency.

## Tests to add or update

The specific tests should match the implementation, but this milestone must include enough validation to satisfy the acceptance criteria below. Prefer unit/golden/fake tests before requiring self-hosted integration tests.

## Acceptance criteria

- [ ] Self-hosted Windows boot smoke reaches guest init/agent or fails with serial evidence.
- [ ] WHPX missing fails before QEMU production launch.
- [ ] Serial logs are captured in a known debug artifact location.
- [ ] macOS boot path unchanged.

## Coding-agent prompt

```text
You are implementing M05: Direct Linux Boot and Serial Logs for the LocalSandbox Windows QEMU + WHPX port.

Read first:
- docs/windows-port/rfc-qemu-whpx.md
- docs/windows-port/AGENTS.md
- docs/windows-port/agent-workspace/state.md
- docs/windows-port/agent-workspace/decisions.md
- docs/windows-port/agent-workspace/milestones/05-direct-linux-boot-serial-logs.md

Implement only this milestone. Preserve public CLI/SDK/Node APIs and existing macOS behavior. Add tests required by the milestone. Do not implement later milestones opportunistically. Update state.md and this milestone handoff before finishing.
```

## Security checklist

Complete the checklist in `../security-checklist.md`. Record any new risk in `../risk-register.md`.

## Handoff

- Branch/PR: `codex/windows-m05-direct-linux-boot-serial-logs`
- Summary: TBD
- Tests run: TBD
- Debug artifacts: TBD
- New decisions: TBD
- New risks: TBD
- Next milestone: TBD
