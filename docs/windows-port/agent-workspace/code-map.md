# Code Map for Windows Port Agents

This file maps likely implementation areas. It is not a substitute for reading source. Verify responsibilities from code before editing.

## Current workspace areas

| Area | Current paths | Notes for Windows port |
|---|---|---|
| VM orchestration | `crates/lsb-vm` | Current high-level sandbox lifecycle. Keep public APIs platform-neutral. Replace non-macOS compile failure with backend capability handling. |
| Platform backend | `crates/lsb-platform` | Existing macOS backend lives under platform-specific modules. Add Windows x86_64 QEMU backend here unless implementation reveals a better location. |
| Guest protocol | `crates/lsb-proto` | Existing frame protocol should remain the product protocol. Add transport-agnostic helpers only if needed. |
| Guest agent | `crates/lsb-guest` | Linux guest process. Add virtio-serial transport and readiness handshake while keeping existing vsock behavior for macOS. |
| CLI | `crates/lsb-cli` | Preserve public flags. Add Windows-specific capability errors/preflight display as needed. |
| Rust SDK | `crates/lsb-sdk` | Preserve API shape. Avoid platform-specific leaks. |
| Proxy | `crates/lsb-proxy` | Current proxy assumes Unix socketpair/file-handle VM attachment. Windows integration will require a new backend strategy. |
| Store/checkpoints | `crates/lsb-store` | Current CAS/NBD path is Unix-socket oriented. Windows MVP should use a simpler artifact/overlay path first. |
| Node binding | `bindings/nodejs` | Add Windows package after backend is stable. Keep unsupported-platform errors clear until then. |
| Kernel/initramfs | `kernel`, guest build scripts | May need x86_64 config updates for virtio-serial, diagnostics, and transport support. |
| CI/release | `.github/workflows` | Add Windows compile/golden jobs first; self-hosted WHPX smoke later. |

## Expected new Windows backend layout

Recommended starting layout:

```text
crates/lsb-platform/src/
  lib.rs
  macos_aarch64/
  macos_x86_64/
  windows_x86_64/
    mod.rs
    backend.rs
    config.rs
    errors.rs
    qemu/
      mod.rs
      discovery.rs
      preflight.rs
      argv.rs
      process.rs
      qmp.rs
    control/
      mod.rs
      virtio_serial.rs
      framed_stream.rs
    fs/
      mod.rs
      copy.rs
      mount_plan.rs
    network/
      mod.rs
      policy.rs
      port_forward.rs
    store/
      mod.rs
      checkpoint.rs
```

Agents may adjust this layout, but changes must preserve layering and be recorded in `state.md`.

## Layering rules

- `lsb-sdk` must not know whether the backend is Apple VZ or QEMU.
- `lsb-cli` may display platform-specific diagnostics, but should not build QEMU argv directly.
- `lsb-vm` should own platform-neutral sandbox lifecycle and call backend traits.
- `lsb-platform` should own platform-specific VM/device/process details.
- `lsb-guest` should expose the same protocol operations regardless of transport.
- `lsb-proxy` should own network/secret policy; QEMU networking must not become the policy engine.
- `lsb-store` should own product checkpoint semantics; QEMU snapshots are implementation details at most.

## Public API compatibility guardrails

Do not change without explicit decision:

- CLI flag names and default behavior.
- Rust SDK public types and methods.
- Node public API shape.
- Existing macOS behavior.
- Existing `lsb-proto` operation semantics.

Add compatibility tests when touching these surfaces.
