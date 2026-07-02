# Issue Backlog

This is a convenient copy/paste backlog. The detailed scope lives in `milestones/`.

## Epic: Windows QEMU + WHPX backend MVP

### M01: Windows compile stubs

Branch: `windows-port-M01-compile-stubs`
Prompt file: `milestones/01-windows-compile-stubs.md`

Goal: Make the repo compile on Windows targets far enough that later work can be implemented behind explicit Windows backend stubs.

### M02: QEMU discovery and WHPX preflight

Branch: `windows-port-M02-qemu-preflight`
Prompt file: `milestones/02-qemu-discovery-preflight.md`

Goal: Find `qemu-system-x86_64.exe`, validate host/backend readiness, and produce actionable diagnostics.

### M03: QEMU argv builder

Branch: `windows-port-M03-qemu-argv`
Prompt file: `milestones/03-qemu-argv-builder.md`

Goal: Create deterministic, redacted QEMU command construction with golden tests.

### M04: QEMU process lifecycle

Branch: `windows-port-M04-qemu-process`
Prompt file: `milestones/04-qemu-process-lifecycle.md`

Goal: Start, supervise, stop, and clean up QEMU on Windows.

### M05: Direct Linux boot and serial logs

Branch: `windows-port-M05-boot-serial`
Prompt file: `milestones/05-direct-linux-boot-serial-logs.md`

Goal: Boot the Linux guest under QEMU + WHPX and capture logs.

### M06: Virtio-serial control transport

Branch: `windows-port-M06-virtio-serial`
Prompt file: `milestones/06-virtio-serial-control-transport.md`

Goal: Exchange `lsb-proto` frames over virtio-serial.

### M07: Guest ready handshake

Branch: `windows-port-M07-ready-handshake`
Prompt file: `milestones/07-guest-ready-handshake.md`

Goal: Make VM readiness deterministic and observable.

### M08: Exec command

Branch: `windows-port-M08-exec`
Prompt file: `milestones/08-exec-command.md`

Goal: Run commands in the Windows-hosted Linux guest through existing APIs.

### M09: Copy-in/copy-out data plane

Branch: `windows-port-M09-copy-in-out`
Prompt file: `milestones/09-copy-in-copy-out-data-plane.md`

Goal: Implement safe host/guest file import and export.

### M10: Mount MVP semantics

Branch: `windows-port-M10-mount-mvp`
Prompt file: `milestones/10-mount-mvp-semantics.md`

Goal: Preserve product-level mount semantics without live shared mounts.

### M11: Port forwarding without guest network

Branch: `windows-port-M11-port-forwarding`
Prompt file: `milestones/11-port-forwarding-no-network.md`

Goal: Preserve host-to-guest port forwarding without arbitrary guest networking.

### M12: Network policy and proxy integration

Branch: `windows-port-M12-network-proxy`
Prompt file: `milestones/12-network-policy-proxy-integration.md`

Goal: Implement strict allowlisted egress and secret substitution.

### M13: Checkpoint/store MVP

Branch: `windows-port-M13-checkpoints`
Prompt file: `milestones/13-checkpoint-store-mvp.md`

Goal: Implement Windows-safe product checkpoint semantics.

### M14: Node packaging

Branch: `windows-port-M14-node-packaging`
Prompt file: `milestones/14-node-packaging.md`

Goal: Add Windows Node package support after Rust smoke tests pass.

### M15: CI and diagnostics hardening

Branch: `windows-port-M15-ci-diagnostics`
Prompt file: `milestones/15-ci-diagnostics-hardening.md`

Goal: Harden hosted and self-hosted CI plus diagnostics artifacts.
