# Validation Experiments

These experiments are intentionally outside the MVP implementation path unless a milestone explicitly adopts one. Use them to retire uncertainties after the core QEMU + WHPX backend is working.

## Experiment E001: QEMU virtio-vsock on Windows host

Status: Not started
Related decisions: D007
Blocking MVP: No

### Question

Can QEMU on a Windows WHPX host provide a reliable host-side virtio-vsock endpoint compatible with LocalSandbox's existing `lsb-proto` transport?

### Why it matters

If viable, virtio-vsock could reduce divergence from macOS and simplify port forwarding/control semantics in a later release.

### Minimal validation

- Boot the LocalSandbox x86_64 guest with virtio-vsock enabled.
- Confirm the Linux guest sees the vsock device.
- Confirm the Windows host can open a connection to the guest vsock port without TCP host networking.
- Run a framed ping/ready exchange.

### Success criteria

- Host/guest connection is reliable across repeated boots.
- No guest NIC is required.
- Security properties are at least as strong as virtio-serial named pipe MVP.

### Failure handling

Keep virtio-serial as the supported transport. Record findings in this file and do not alter D007 without review.

## Experiment E002: VirtioFS on Windows host

Status: Not started
Related decisions: D009, D010, D011
Blocking MVP: No

### Question

Can Windows-host QEMU provide a production-suitable VirtioFS path with acceptable packaging, performance, security, and file semantics?

### Minimal validation

- Identify required `virtiofsd` binary and Windows support status.
- Share a Windows directory read-only into the Linux guest.
- Test case sensitivity, symlinks/junctions, permissions, file watching, large directories, and concurrent read/write behavior.
- Confirm the guest overlay behavior can preserve LocalSandbox semantics.

### Success criteria

- No admin requirement for normal use.
- Read-only host exposure is enforceable.
- Guest writes can be isolated.
- Behavior is documented and testable.

### Failure handling

Keep copy-in/copy-out MVP. Consider custom sync service later.

## Experiment E003: 9p/virtfs on Windows host

Status: Not started
Related decisions: D009, D010
Blocking MVP: No

### Question

Can QEMU 9p/virtfs on Windows host provide a simpler live sharing path than VirtioFS?

### Minimal validation

- Boot guest with a 9p share from a Windows path.
- Test read-only behavior and metadata fidelity.
- Test performance on large source trees.
- Test symlink/junction behavior and path escape resistance.

### Success criteria

- Product-level mount semantics can be preserved.
- Performance is acceptable for coding-agent workflows.
- Failure modes are understandable.

### Failure handling

Keep copy-in/copy-out MVP.

## Experiment E004: Windows proxy attachment for strict egress

Status: Not started
Related decisions: D012, D013, D014
Blocking MVP: Yes for M12, not for earlier milestones

### Question

What Windows/QEMU attachment path best preserves LocalSandbox's host-side proxy policy without arbitrary guest networking?

### Candidate approaches

- QEMU network backend connected to LocalSandbox proxy process.
- TAP/WinTun with non-admin or managed setup if feasible.
- Guest configured to use explicit HTTP(S)/DNS proxy over a private transport.
- QEMU user networking only when all egress is forced through policy and bypasses are blocked.

### Minimal validation

- Default VM has no NIC.
- Allowed domain succeeds only when policy permits.
- Blocked domain and direct IP fail.
- Secret substitution remains host-side and redacted.

### Success criteria

- No admin requirement for MVP unless separately approved.
- No direct IP bypass.
- No QEMU NAT bypass.
- Policy tests are automatable.

## Experiment E005: Windows ARM64 future path

Status: Not started
Related decisions: D002
Blocking MVP: No

### Question

What changes are required to support Windows ARM64 hosts after x86_64 is stable?

### Minimal validation

- Identify QEMU binary and WHPX ARM64 requirements.
- Identify guest architecture strategy: aarch64 guest or emulated x86_64 guest only for diagnostics.
- Validate asset build and packaging path.

### Success criteria

- Clear follow-on RFC or RFC amendment.
- No disruption to x86_64 Windows backend.
