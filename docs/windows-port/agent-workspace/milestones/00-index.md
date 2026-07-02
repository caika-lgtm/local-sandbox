# Windows Port Milestones

Implement these milestones sequentially. Do not skip ahead unless `state.md` records an explicit maintainer decision.

| ID | Title | Primary outcome | Depends on |
|---|---|---|---|
| M01 | Windows compile stubs | Repo compiles far enough on Windows with clear unsupported/capability errors | None |
| M02 | QEMU discovery and WHPX preflight | Find QEMU, validate host/backend readiness, produce diagnostics | M01 |
| M03 | QEMU argv builder | Deterministic, redacted QEMU command construction | M02 |
| M04 | QEMU process lifecycle | Start/stop/supervise QEMU and clean up with Windows Job Objects | M03 |
| M05 | Direct Linux boot and serial logs | Boot Linux guest under QEMU + WHPX and capture logs | M04 |
| M06 | Virtio-serial control transport | Host and guest exchange `lsb-proto` frames over virtio-serial | M05 |
| M07 | Guest ready handshake | Deterministic readiness and timeout behavior | M06 |
| M08 | Exec command | First end-to-end useful guest operation | M07 |
| M09 | Copy-in/copy-out data plane | Host can import/export files safely without live mounts | M08 |
| M10 | Mount MVP semantics | Preserve product-level read-only source and isolated writes | M09 |
| M11 | Port forwarding | Host-to-guest forwarding without guest arbitrary networking | M07 |
| M12 | Network policy and proxy integration | Strict allowlisted egress and secret substitution | M08, M11 |
| M13 | Checkpoint/store MVP | Product-level checkpoint semantics using Windows-safe disk artifacts | M09, M10 |
| M14 | Node packaging | Windows Node package after Rust backend smoke passes | M08-M13 core smoke |
| M15 | CI and diagnostics hardening | Hosted + self-hosted Windows CI, artifacts, docs | All previous, incremental throughout |

## Recommended issue naming

Use:

```text
windows-port-MNN-short-title
```

Examples:

- `windows-port-M01-compile-stubs`
- `windows-port-M05-direct-linux-boot`
- `windows-port-M12-network-policy-proxy`

## Done definition for every milestone

- Scope completed or explicitly deferred.
- Required tests added and run.
- Existing macOS behavior not regressed.
- Security checklist completed.
- `state.md` updated.
- Current milestone doc updated.
- Handoff written.
