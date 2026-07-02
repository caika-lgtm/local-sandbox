# M06: Virtio-Serial Control Transport

Status: Not started
Depends on: See `00-index.md`
RFC sections: See `traceability.md`

## Objective

Connect host and guest using the existing `lsb-proto` frames over virtio-serial.

## Scope

- Add guest transport abstraction: vsock for macOS remains, virtio-serial for Windows added.
- Configure QEMU virtio-serial device and port.
- Use private Windows named pipe or chosen QEMU chardev backend.
- Implement host-side async stream wrapper.
- Add simple ping/echo or protocol-level smoke if available.

## Out of scope

- Do not replace `lsb-proto`.
- Do not adopt QGA for LocalSandbox guest APIs.
- Do not use hostfwd TCP as production control path.
- Do not break macOS vsock transport.

## Likely files / crates

- `crates/lsb-guest/src/main.rs` or transport modules
- `crates/lsb-platform/src/windows_x86_64/control/virtio_serial.rs`
- `crates/lsb-proto` only for transport-neutral helpers if needed

## Design notes

- Preserve existing macOS behavior unless the milestone explicitly states otherwise.
- Keep Windows-specific implementation behind platform/backend boundaries.
- Prefer precise capability errors over silent degradation.
- Update `state.md` when implementation reveals a better file layout or dependency.

## Tests to add or update

The specific tests should match the implementation, but this milestone must include enough validation to satisfy the acceptance criteria below. Prefer unit/golden/fake tests before requiring self-hosted integration tests.

## Acceptance criteria

- [ ] Guest builds with both macOS/vsock and Windows/virtio-serial support.
- [ ] Host opens control transport reliably after boot.
- [ ] Protocol framing tests pass over in-memory and Windows transport where possible.
- [ ] Timeouts produce clear diagnostics.

## Coding-agent prompt

```text
You are implementing M06: Virtio-Serial Control Transport for the LocalSandbox Windows QEMU + WHPX port.

Read first:
- docs/windows-port/rfc-qemu-whpx.md
- docs/windows-port/AGENTS.md
- docs/windows-port/agent-workspace/state.md
- docs/windows-port/agent-workspace/decisions.md
- docs/windows-port/agent-workspace/milestones/06-virtio-serial-control-transport.md

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
