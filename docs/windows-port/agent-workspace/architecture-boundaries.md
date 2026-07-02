# Architecture Boundaries

This document gives implementation-level guidance for keeping the Windows backend cleanly separated from product semantics.

## Backend responsibility split

```text
CLI / SDK / Node
        |
        v
lsb-vm: product lifecycle and platform-neutral API
        |
        v
VmBackend trait / platform adapter boundary
        |
        +--> macOS Virtualization.framework backend
        |
        +--> Windows QEMU + WHPX backend
                    |
                    +--> QEMU discovery and preflight
                    +--> QEMU argv builder
                    +--> QEMU process supervisor
                    +--> QMP management client, if needed
                    +--> virtio-serial control transport
                    +--> Windows file/mount/checkpoint strategies
                    +--> Windows network/proxy attachment strategy
```

## Suggested platform-neutral traits

Agents should prefer small traits that describe product needs rather than QEMU concepts.

```rust
trait VmBackend {
    type Vm: RunningVm;

    async fn create(config: VmConfig) -> Result<Self::Vm, VmError>;
}

trait RunningVm {
    async fn start(&mut self) -> Result<(), VmError>;
    async fn wait_ready(&mut self) -> Result<(), VmError>;
    async fn connect_control(&self) -> Result<Box<dyn ControlTransport>, VmError>;
    async fn shutdown(&mut self) -> Result<(), VmError>;
    async fn kill(&mut self) -> Result<(), VmError>;
}

trait ControlTransport: AsyncRead + AsyncWrite + Send + Unpin {}
```

These are sketches, not mandatory exact code. The key requirement is that QEMU argv, chardev syntax, named-pipe paths, QMP, and Windows process handles stay out of the public SDK.

## QEMU-specific components

| Component | Owns | Must not own |
|---|---|---|
| `QemuDiscovery` | Finding `qemu-system-x86_64.exe`, version capture, path validation | Product feature policy |
| `QemuPreflight` | WHPX availability checks, QEMU capability probes, diagnostic messages | Auto-enabling TCG for production |
| `QemuArgvBuilder` | Deterministic argv construction and redaction | Reading CLI flags directly |
| `QemuProcess` | Process start/stop, pipes, Job Object cleanup, log capture | Guest protocol semantics |
| `QmpClient` | QEMU lifecycle introspection and controlled commands | LocalSandbox exec/file API |
| `VirtioSerialTransport` | Opening and framing the host side of the control pipe | Deciding network/mount policy |

## Guest transport rule

The guest protocol is `lsb-proto`. Transport is replaceable.

```text
macOS today:    host lsb-proto <-> AF_VSOCK <-> guest lsb-guest
Windows MVP:    host lsb-proto <-> virtio-serial/named pipe <-> guest lsb-guest
Future option:  host lsb-proto <-> vsock/Hyper-V sockets <-> guest lsb-guest
```

Do not introduce QGA as a replacement for LocalSandbox guest operations during MVP.

## Error taxonomy

Prefer structured errors with stable categories:

- `UnsupportedPlatform`
- `MissingQemu`
- `UnsupportedQemuVersion`
- `WhpxUnavailable`
- `AssetMissing`
- `QemuStartFailed`
- `GuestBootTimeout`
- `ControlTransportUnavailable`
- `GuestProtocolError`
- `FeatureUnsupportedOnWindows`
- `NetworkPolicyViolation`
- `CheckpointUnsupported`

Every error should include:

- what failed,
- likely cause,
- safe remediation,
- path or command only when not sensitive,
- redacted QEMU argv if relevant.

## Redaction boundary

Redact before logs leave the owning component. Do not rely on callers to redact.

Must redact:

- environment values containing secrets,
- proxy secret literals,
- auth tokens,
- QMP socket/pipe paths if they encode user-private temp names and are included in external reports,
- host paths when configured as private in future.
