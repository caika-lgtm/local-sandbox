# Windows Port Documentation

This directory contains the durable documentation for the LocalSandbox Windows
backend after the Windows MVP sprint.

## Current status

The Windows backend supports Windows 11 x64 hosts through QEMU with WHPX. It
boots the existing Linux guest, uses virtio-serial for LocalSandbox control,
negotiates `CAP_SESSION_MUX` for concurrent control sessions, supports
non-interactive exec, streaming spawn with stdin/kill, guest file transfer,
file watch, staged mount imports, explicit SMB/CIFS direct mounts, host-to-guest
port forwarding without a guest NIC, policy-mediated `--allow-net`, flattened
qcow2 checkpoints, the Windows x64 Node package, and hosted/self-hosted CI
coverage. The release path includes Windows x64 CLI and runtime asset artifacts
plus a native PowerShell installer. `lsb init` installs the managed QEMU host
tool package under `%LOCALAPPDATA%\lsb\tools\qemu` and does not mutate global
`PATH`.

For normal Windows development, download released runtime assets with `lsb init`
instead of building them locally. The Windows package contains `Image`,
`initramfs.cpio.gz`, and `rootfs.ext4` built for the QEMU/WHPX path, including
virtio-serial support. Source-building those assets remains possible through
`xtask prepare-rootfs --platform windows-x86_64`, but it is Docker/Linux-hosted
and more complicated than the recommended release download path.

The MVP is complete for upstream review, but it is not a production-readiness
certification. See `mvp-handoff.md` before planning follow-up work.

## Windows direct mounts

Windows direct directory mounts use SMB/CIFS without changing the public CLI,
Rust SDK, or Node API shape.

- CLI no-suffix mounts and CLI `:ro` mounts remain overlay snapshot imports.
- CLI `:rw` mounts continue to require `--allow-host-writes`; on Windows they
  are SMB/CIFS direct read-write mounts.
- SDK and Node `Direct { flags: 0 }` map to SMB/CIFS read-write direct
  mounts, and `Direct { flags: MS_RDONLY }` maps to SMB/CIFS read-only
  direct mounts.
- Windows SMB direct mounts require an elevated Administrator shell.
- SMB direct mounts must use LocalSandbox-controlled proxy networking and must
  not imply arbitrary outbound `allow_net`.
- LocalSandbox creates ephemeral Windows users, shares, credentials, and
  reversible ACL grants, then records a non-secret cleanup manifest for stale
  recovery.
- `lsb doctor windows-smb-policy` diagnoses local policy that blocks generated
  SMB users; `--fix` replaces the broad `NT AUTHORITY\Local account`
  network-logon deny with the narrower local-Administrator-account deny.
- Public SDK and Node `watch()` calls at or below a direct SMB target use a
  host-side Windows directory watcher and map relative host events back to guest
  paths. Host-originated changes and guest-originated CIFS writes are covered.
  Read-only direct mounts still report host-originated changes while guest
  writes remain denied.
- Recursive watches whose root is an ancestor of a direct SMB target are
  rejected unless the requested path is at or below a single direct SMB mount.
  Start separate watches rather than relying on partial hybrid coverage.

See `decisions.md` D024, `mvp-handoff.md`, and `validation.md` for the current
support status and smoke scope.

## Files

| File | Purpose |
|---|---|
| `mvp-handoff.md` | Current Windows MVP support status, known limitations, validation evidence, and future work. Start here. |
| `rfc-qemu-whpx.md` | Original QEMU + WHPX design RFC and rationale. Historical design record, not the current status tracker. |
| `decisions.md` | Accepted Windows backend decisions. Add new decisions here only after review. |
| `architecture.md` | Current crate/module map, backend boundaries, and product invariants. |
| `validation.md` | Test strategy, CI lanes, self-hosted runner commands, and smoke coverage. |
| `diagnostics.md` | Failure triage, diagnostic artifacts, redaction rules, and collector behavior. |
| `runner-setup.md` | Maintainer notes for the self-hosted Windows 11 WHPX runner. |
| `security-checklist.md` | Security checklist for Windows backend changes. |
| `review-checklist.md` | PR review checklist for Windows backend changes. |
| `risk-register.md` | Active, accepted, and retired Windows backend risks. |
| `future-work.md` | Follow-on features and experiments intentionally left out of the MVP. |

## Working rules

- Preserve the public CLI, Rust SDK, and Node API shape unless a new accepted
  decision permits a change.
- Preserve macOS behavior while changing shared code.
- Keep QEMU/WHPX details below the platform/backend boundary.
- Keep default Windows networking disabled.
- Keep host secrets on the host; diagnostics must be redacted.
- Use the self-hosted WHPX workflow for real Windows boot/runtime evidence.

## Standard validation

Use local checks for platform-independent changes, then the Windows hardware
helper when runtime behavior is touched:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace

./scripts/win-gh-test check
./scripts/win-gh-test unit
./scripts/win-gh-test smoke
./scripts/win-gh-test e2e
```

The Windows helper dispatches `.github/workflows/windows-lsb-hardware.yml`,
which also runs the e2e lane automatically on trusted `main` pushes and
requires a clean committed working tree for manual branch runs.
