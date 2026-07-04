# M07: Guest Ready Handshake

Status: Done
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

- [x] Ready succeeds on boot smoke.
- [x] Wrong/missing ready message times out cleanly.
- [x] Capabilities are logged without secrets.
- [x] macOS path either uses same handshake or remains compatible by explicit design.

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

- Branch/PR: `codex/windows-m07-guest-ready-handshake`
- Summary: Added a platform-neutral `GuestReady` frame, emitted it from `lsb-guest` on the Windows virtio-serial transport after minimal init, and changed the Windows boot lifecycle to wait for that frame before reporting VM startup success. Readiness failures now distinguish early QEMU exit, missing/failed control transport, protocol/invalid-frame errors, timeout, and unsupported reported capabilities. The initial Windows capability list is empty.
- Tests run: `cargo fmt --all -- --check`; `cargo check --workspace`; `cargo test -p lsb-proto`; `cargo test -p lsb-guest`; `cargo test -p lsb-platform`; `cargo test -p lsb-vm`; `cargo check -p lsb-platform --tests --target x86_64-pc-windows-msvc`; `cargo test --workspace`; `./scripts/win-gh-test unit`; `./scripts/win-gh-test smoke`.
- Debug artifacts: Windows smoke run `28703154530`, job `85125213672`, artifact `windows-lsb-diagnostics` ID `8080915182`. `lsb-assets-work/28703154530-1/boot.status.json` recorded `state: "guest_ready"`, success definition `localsandbox_guest_ready_frame_received_over_control_transport`, elapsed readiness `1727` ms, protocol version `1`, transport `virtio_serial`, guest version `0.3.12`, and empty capabilities. The same diagnostics include `qemu.argv.redacted.txt`, `qemu.stdout.log`, `qemu.stderr.log`, `serial.log`, `preflight.json`, and `qemu.status.json`.
- New decisions: None; this follows the RFC guidance to use the LocalSandbox protocol over the M06 transport rather than QMP or ad hoc Windows bytes.
- New risks: None added.
- Next milestone: M08 exec command.

Security review:
- No-network default preserved: yes
- Secret redaction verified: yes
- Host file exposure reviewed: n/a
- Control/QMP endpoint privacy reviewed: yes
- Process cleanup reviewed: yes
- New risks added to risk-register.md: no
