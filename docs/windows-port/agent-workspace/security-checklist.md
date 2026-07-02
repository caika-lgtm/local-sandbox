# Windows Port Security Checklist

Every implementation milestone must preserve the security posture below.

## Threat model summary

- Guest workload is untrusted.
- Host secrets are high-value and must not be copied into the guest by default.
- QEMU is an attack surface, not a security boundary by itself.
- The host filesystem must be exposed minimally and read-only from the product perspective.
- Localhost sockets, named pipes, QMP endpoints, and temp directories must be private to the owning user/session.
- Network policy must be enforced by LocalSandbox-controlled code, not by convenience networking defaults.

## Per-milestone checklist

Before marking a milestone done, answer:

- [ ] Does this change preserve no-network-by-default?
- [ ] Does this change avoid putting secrets in guest env, argv, logs, snapshots, or debug artifacts?
- [ ] Are all host listeners bound to loopback or private pipe/socket mechanisms?
- [ ] Are QMP/control endpoints private and unauthenticated only when private by construction?
- [ ] Are temp/debug directories created under an appropriate user-owned location?
- [ ] Does this change avoid direct host writes unless explicitly approved?
- [ ] Are Windows paths normalized and checked for traversal/junction/symlink surprises where relevant?
- [ ] Does failure cleanup terminate QEMU/helper processes?
- [ ] Does the feature fail closed on unsupported Windows/QEMU capability?
- [ ] Are diagnostics redacted?

## Specific requirements

### QEMU process

- Validate QEMU path before execution.
- Prefer absolute path after discovery.
- Do not execute QEMU from world-writable directories unless explicitly allowed for development with a warning.
- Use Windows Job Objects or equivalent cleanup so child/helper processes do not survive unexpectedly.

### QMP

- QMP is a QEMU management channel only.
- Bind QMP to a private named pipe or loopback socket with unpredictable name/path.
- Do not expose QMP on non-loopback interfaces.
- Do not forward QMP into the guest.

### Control transport

- The LocalSandbox guest control transport must not be reachable by other local users under normal configuration.
- Use per-sandbox names and avoid predictable global pipe names unless ACLs are restrictive.
- Protocol traces must redact secrets and large payloads.

### Files and mounts

- Host source data is read-only from the product perspective.
- Reject path traversal in copy-in/copy-out and export paths.
- Define symlink/junction behavior explicitly before following links on Windows.
- Do not support direct `:rw` host mounts in MVP.

### Networking

- Default QEMU argv must not include a guest NIC.
- QEMU user networking must not be enabled by default.
- Allowlisted egress must be mediated by LocalSandbox proxy policy.
- Direct IP bypass and non-proxied traffic must fail unless explicitly allowed.

### Secrets

- Guest receives placeholders only when policy allows.
- Proxy substitutes secrets only for configured host patterns.
- Logs must show placeholder IDs or redacted labels, not secret values.

## Security sign-off template

Use this in milestone handoff notes:

```text
Security review:
- No-network default preserved: yes/no/n/a
- Secret redaction verified: yes/no/n/a
- Host file exposure reviewed: yes/no/n/a
- Control/QMP endpoint privacy reviewed: yes/no/n/a
- Process cleanup reviewed: yes/no/n/a
- New risks added to risk-register.md: yes/no
```
