# M11: Port Forwarding Without Guest Network

Status: Review
Depends on: See `00-index.md`
RFC sections: See `traceability.md`

## Objective

Preserve host-to-guest port forwarding without enabling arbitrary guest networking.

## Scope

- Implement forwarding over LocalSandbox control/data channel or a dedicated virtio-serial channel.
- Bind host listener to loopback.
- Forward to guest-local service port.
- Handle connection close, backpressure, and errors.
- Keep QEMU `hostfwd` debug-only if present at all.

## Out of scope

- Do not enable guest NIC for normal port forwarding.
- Do not bind public interfaces.
- Do not use QEMU user networking as normal implementation.

## Likely files / crates

- `crates/lsb-vm/src/sandbox.rs` forwarding path
- `Windows control/data transport`
- `crates/lsb-guest` forward handler

## Design notes

- Preserve existing macOS behavior unless the milestone explicitly states otherwise.
- Keep Windows-specific implementation behind platform/backend boundaries.
- Prefer precise capability errors over silent degradation.
- Update `state.md` when implementation reveals a better file layout or dependency.

## Tests to add or update

The specific tests should match the implementation, but this milestone must include enough validation to satisfy the acceptance criteria below. Prefer unit/golden/fake tests before requiring self-hosted integration tests.

## Acceptance criteria

- [ ] Start guest service and reach it from host loopback. Pending final self-hosted WHPX smoke result.
- [x] Golden argv still has no NIC.
- [x] Port conflict produces clear error.
- [x] Forwarding stops cleanly when sandbox exits.

## Coding-agent prompt

```text
You are implementing M11: Port Forwarding Without Guest Network for the LocalSandbox Windows QEMU + WHPX port.

Read first:
- docs/windows-port/rfc-qemu-whpx.md
- docs/windows-port/AGENTS.md
- docs/windows-port/agent-workspace/state.md
- docs/windows-port/agent-workspace/decisions.md
- docs/windows-port/agent-workspace/milestones/11-port-forwarding-no-network.md

Implement only this milestone. Preserve public CLI/SDK/Node APIs and existing macOS behavior. Add tests required by the milestone. Do not implement later milestones opportunistically. Update state.md and this milestone handoff before finishing.
```

## Security checklist

Complete the checklist in `../security-checklist.md`. Record any new risk in `../risk-register.md`.

## Handoff

- Branch/PR: `codex/windows-m11-port-forwarding`
- Summary: Implemented Windows host-to-guest port forwarding over a dedicated private LocalSandbox virtio-serial channel, with host listeners bound to `127.0.0.1`, guest proxying only to guest loopback, and QEMU argv remaining `-nic none` with no normal-product `hostfwd`. The public CLI/SDK/Node API shape is unchanged, and macOS vsock forwarding remains on the existing path for valid nonzero mappings.
- Tests run: `cargo fmt --all -- --check`; `cargo check --workspace`; `cargo test --workspace`; `cargo check -p lsb-platform -p lsb-vm --tests --target x86_64-pc-windows-msvc`; `cargo check --workspace --target x86_64-pc-windows-msvc` (blocked on this macOS host by external MSVC C/assembler tooling: `ring` missing Windows/MSVC `assert.h`, `blake3` missing `ml64.exe`). Final `./scripts/win-gh-test smoke` is pending on the self-hosted Windows runner.
- Debug artifacts: pending final smoke result. Local unit/golden coverage validates no `hostfwd`, no `-netdev`, loopback bind helper behavior, invalid/duplicate port validation, protocol session payload encoding, and the ignored WHPX port-forward smoke hook.
- New decisions: None. The implementation follows the RFC/M11 direction to use a LocalSandbox guest channel rather than QMP, QEMU user networking, or QEMU `hostfwd`.
- New risks: Windows M11 serializes active forwarding sessions over the dedicated forwarding channel until a future mux/session model exists. This preserves the no-network-by-default security model but does not provide concurrent forwarding-session multiplexing yet.
- Next milestone: M12 network policy/proxy integration remains separate; do not treat M11 as general Windows networking support.
