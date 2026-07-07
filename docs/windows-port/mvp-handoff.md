# Windows Port MVP Handoff

Last updated: 2026-07-07

## Summary

The Windows port MVP implements a native Windows 11 x64 LocalSandbox backend
using QEMU with WHPX. The implementation keeps the existing Linux guest model
and public LocalSandbox API shape while replacing the macOS Apple
Virtualization.framework backend with a supervised QEMU process. Release
packaging now includes Windows x64 CLI install artifacts and the Windows x64
Node package, plus Windows x64 runtime assets for the QEMU/WHPX guest path.
Post-MVP mux, streaming spawn, guest watch, and direct SMB watch work has also
landed on this branch without changing public CLI, Rust SDK, or Node API shape.

This handoff replaces the completed sprint workspace and milestone prompt files.
It is the current status source for future agents.

## Implemented

| Area | Current behavior |
|---|---|
| Host target | Windows 11 x64. Windows ARM64 is future work. |
| VM backend | QEMU direct Linux boot with `-accel whpx`, `-cpu Westmere`, explicit devices, no display, no default NIC, and Windows Job Object cleanup. |
| QEMU discovery | `LSB_QEMU`, internal config hook, managed QEMU, then `PATH`; structured preflight errors for missing QEMU, invalid paths, version parse, and missing WHPX support. |
| Managed QEMU | `lsb init` installs the pinned Windows x86_64 QEMU package under `%LOCALAPPDATA%\lsb\tools\qemu`, validates artifact SHA-256, safe extraction, manifest executable paths, required notices, and writes `current.json`. |
| Boot assets | Released Windows x64 runtime assets provide `Image`, `initramfs.cpio.gz`, and `rootfs.ext4` with the QEMU/WHPX guest requirements such as virtio-serial. Developers should use `lsb init`; self-hosted CI hydrates/caches assets and prepares disposable per-run rootfs copies. |
| Control transport | `lsb-proto` over virtio-serial through private QEMU pipe chardevs. The host connects during boot because QEMU pipe chardev startup can block until a client connects. After the raw `GuestReady` frame advertises `CAP_SESSION_MUX`, a Windows mux manager owns the physical control stream and opens virtual exec, watch, and file sessions. |
| Readiness | Windows startup succeeds only after a valid LocalSandbox `GuestReady` frame over the established control stream. |
| Exec and spawn | Non-interactive `exec` works through the existing product API and returns stdout, stderr, and exit status. Streaming spawn runs over mux exec sessions and supports stdout, stderr, exit status, cwd, stdin writes, kill, concurrent processes, and large-output fairness. |
| File transfer | Copy-in/copy-out helpers stream guest file content over the control path with validation for path traversal, chunk sizes, final byte counts, and overwrite behavior. |
| Mounts | Windows overlay-style mounts are snapshot imports for CLI no-suffix and `:ro`; explicit direct mounts use SMB/CIFS with ephemeral users, shares, credentials, ACL grants, and cleanup manifests. |
| Watch | Public SDK and Node watch support uses guest-side inotify over mux sessions for normal guest paths and overlay/import paths. Direct SMB mount paths use a host-side Windows directory watcher with guest path mapping. |
| Port forwarding | Host-to-guest forwarding uses a dedicated private virtio-serial channel and host listeners bound to `127.0.0.1`. It does not enable a guest NIC or QEMU `hostfwd`. |
| Networking | Default argv remains `-nic none`. Existing allow-net/proxy configuration attaches a QEMU stream netdev only to a LocalSandbox-owned loopback proxy path. |
| Secrets | Guest environment values are placeholders. Host-side proxy policy performs substitution only for configured destinations. Diagnostics redact secret-bearing values. |
| Checkpoints | Windows checkpoints use private per-instance qcow2 overlays and flattened qcow2 checkpoint artifacts plus JSON metadata. macOS CAS/NBD behavior is unchanged. |
| CLI release/install | Release CI builds a Windows x64 CLI archive containing `lsb.exe`. `install.ps1` supports native PowerShell installs, and `install.sh` supports Git Bash/MSYS/Cygwin installs. After CLI installation, `lsb init` installs the managed QEMU host-tool package. |
| Node binding | Windows x64 package metadata and `x86_64-pc-windows-msvc` NAPI target wiring exist. Node `Sandbox.start()` surfaces Rust backend/preflight error chains. Node streaming spawn and watch run through the same public API as macOS. |
| CI | Hosted Windows CI runs compile/unit/golden coverage without QEMU/WHPX. The self-hosted Windows 11 WHPX workflow runs e2e on trusted `main` pushes and supports manual check, unit, smoke, and e2e lanes, including mux spawn/watch and direct SMB watch smoke coverage. |
| Diagnostics | QEMU argv, stdout/stderr, serial log, preflight, boot status, environment summary, manifest, and redacted control/proxy artifacts are collected through a redacted diagnostic collector. |

## Intentional MVP limitations

- No Windows ARM64 support.
- No QEMU bundled inside CLI archives, OS runtime assets, or npm packages.
- No normal TCG fallback. Production Windows execution requires WHPX.
- No live host/guest mount synchronization for overlay, no-suffix, or CLI `:ro`
  mounts. Watch on those paths observes the guest staging view; later
  host-originated source changes are not live-synced. SMB/CIFS direct mounts
  are the supported live-sharing path and require Administrator privileges.
- No interactive shell or PTY support on Windows.
- Port forwarding uses a separate forwarding channel, not the session mux, and
  currently serializes active forwarding sessions.
- Direct SMB watch is supported only for public SDK and Node watch paths at or
  below one direct SMB target. A recursive watch above a direct SMB mount target
  returns a precise unsupported error instead of partial hybrid coverage.
- No CAS/NBD checkpoint parity on Windows.
- Windows SDK `checkpoint()` stops the VM before flattening the active qcow2
  overlay into a checkpoint artifact. This is not live checkpointing.
- Managed QEMU is pinned to QEMU 11.0.50 package `qemu-11.0.50-lsb0.4.0`;
  broader minimum-version policy for overrides remains future work.
- No broad `lsb doctor windows` namespace yet.
- `lsb doctor windows-smb-policy` diagnoses direct-SMB user-rights policy and
  can apply the local runner repair with `--fix --yes`.
- Native Windows build-number probing is deferred.
- Self-hosted runner labels still use the default `self-hosted, Windows, X64`
  set and assume one persistent WHPX runner for smoke/e2e cache reuse.

## Windows SMB/CIFS direct mounts

Windows direct directory mounts use SMB/CIFS. The public API shape remains
unchanged:

- CLI no-suffix mounts and CLI `:ro` mounts stay overlay snapshot imports.
- CLI `:rw` plus `--allow-host-writes` is an SMB/CIFS direct read-write mount
  and requires an elevated Administrator shell.
- SDK and Node `Direct { flags: 0 }` are SMB/CIFS direct read-write mounts.
- SDK and Node `Direct { flags: MS_RDONLY }` are SMB/CIFS direct read-only
  mounts.
- SMB direct mounts must not imply arbitrary outbound `allow_net`; they use the
  LocalSandbox-controlled proxy path.
- Direct SMB preflight rejects hosts whose `SeDenyNetworkLogonRight` contains
  `NT AUTHORITY\Local account` because that blocks generated local SMB users.
  `lsb doctor windows-smb-policy --fix` replaces that broad deny with the
  narrower local-Administrator-account deny while leaving Guests denied.
- LocalSandbox writes a non-secret cleanup manifest into the instance directory
  after resources are prepared. Normal cleanup removes the manifest; stale
  startup recovery scans manifests and retries share, ACL, and user cleanup.
- SDK and Node `watch()` calls at or below one direct SMB target use a
  host-side `ReadDirectoryChangesW` watcher on the canonical source path and map
  events back to guest paths. Host-created, modified, renamed, and deleted files
  are reported; guest writes through CIFS are also reported after they
  materialize on the host filesystem. Read-only direct mounts can be watched for
  host-originated changes while guest writes remain denied.
- Recursive watch roots that would span guest-only paths and one or more direct
  SMB targets are rejected. Start one watch per direct target plus any
  guest-only watches needed.

## Important implementation notes

- `lsb-platform::windows_x86_64` owns QEMU discovery, preflight, argv building,
  process supervision, boot, control transport, filesystem planning, network
  attachment, and backend startup glue.
- QEMU commands are constructed as structured argv values. Do not shell-concat
  QEMU commands.
- The Windows QEMU direct boot path uses `-cpu Westmere`. A self-hosted run with
  `-cpu max` on QEMU 11.0.50 failed before serial output with APX/MPX warnings
  and `WHPX: Unexpected VP exit code 4`; see D020.
- The control pipe must be connected during boot. A self-hosted M06 run showed
  QEMU pipe chardev startup can block guest boot until the host connects; see
  D021.
- Session mux startup preserves the raw `GuestReady` handshake. Once
  `CAP_SESSION_MUX` is advertised, the Windows mux manager owns the established
  physical control pipe; later exec, file, mount init, and guest-side watch
  operations use virtual sessions. Do not reintroduce independent physical
  readers on the control pipe.
- Mount import validation is conservative: traversal, reparse points,
  symlinks/junctions, hardlinks, special files, case-insensitive collisions, and
  plan-to-open replacement are rejected or fail closed.
- Windows path validation accepts canonicalized drive-verbatim paths such as
  `\\?\C:\...` because `std::fs::canonicalize` returns that shape on Windows.
  UNC verbatim and device paths remain unsupported.
- Allow-net on Windows uses `-netdev stream` to a LocalSandbox-owned loopback
  proxy endpoint. QEMU user networking, `hostfwd`, TAP, bridge, NAT, and
  non-loopback proxy endpoints remain unsupported product paths.
- Explicit network allowlists bind policy-visible SNI/HTTP Host values to
  recent proxy DNS A answers before upstream connect and before secret
  substitution. Direct IP, missing-domain, non-allowlisted-domain, and forged
  allowed Host/SNI-to-arbitrary-IP traffic fail closed.
- Windows checkpoint save writes metadata only after `qemu-img convert` succeeds
  and refuses same-name `.qcow2`, `.json`, `.idx`, or `.ext4` conflicts.
- Direct SMB watch path resolution is owned by the SDK runtime registry. It uses
  longest matching guest-target prefixes with path-boundary checks, then starts
  host watchers only for configured direct SMB source paths. Guest paths are
  never accepted as raw host paths.

## Validation evidence

Key self-hosted WHPX evidence from the MVP sprint:

| Area | Evidence |
|---|---|
| Boot with serial output | `./scripts/win-gh-test smoke` run `28698120131` passed with non-empty Linux serial output and diagnostics artifact `8079375534`. |
| Virtio-serial control | Smoke run `28702513259` passed with guest virtio-serial selection and control-port open; artifact `8080640442`. |
| Guest ready | Smoke run `28703154530` reached `boot.status.json` state `guest_ready`; artifact `8080915182`. |
| Exec | Smoke run `28705865747` passed Windows guest exec; artifact `8081627693`. |
| Copy transfer | Smoke run `28710991403` passed copy transfer after review fixes. |
| Mounts | Smoke run `28730520707` passed mount MVP validation after review fixes; artifact `8088597449`. |
| Port forwarding | Smoke run `28734824475` passed host loopback forwarding with `-nic none`; artifact `8090018787`. |
| Network policy/proxy | Smoke run `28737504945` passed review-hardened allow-net/proxy checks; artifact `8090839974`. |
| Checkpoints | Smoke run `28739351408` passed checkpoint/store smoke; artifact `8091364138`. |
| Node runtime | Smoke run `28742090397` passed Windows Node build/import/start after preserving N-API error chains; artifact `8092196979`. |
| M15 packed package smoke | Smoke run `28743977168` passed packed root + Windows optional npm package import and runtime smokes; artifact `8092721984`. |
| CLI e2e workflow | E2E run `28771975549` passed the user-facing Windows CLI workflow covering boot/exec, no-network denial, mounts, port forwarding, scoped host exposure, and checkpoint operations. |

Post-mux validation coverage is now wired into the durable smoke lane:

| Area | Current smoke coverage |
|---|---|
| Mux spawn and guest watch | `scripts/windows-smoke.ps1` runs `windows_qemu_spawn_guest_watch_smoke`, covering concurrent spawn, stdout/stderr/exit/cwd/stdin/kill, large-output fairness, recursive guest watch events, and watch/spawn coexistence. |
| Node streaming | `scripts/windows-smoke.ps1` runs `bindings/nodejs/test/streaming.spec.ts` against disposable Windows runtime assets, covering positive Node spawn and watch behavior. |
| Direct SMB watch | `windows_qemu_direct_smb_mount_smoke` covers host-originated direct SMB watch events, guest-originated CIFS writes observed by the host watcher, read-only direct SMB host watch events, write denial on read-only mounts, and mount-only proxy egress denial. |

Record the exact final WHPX workflow run and artifact IDs in the PR or release
evidence for the branch under review; keep this file focused on durable support
status and lane scope.

## Open production-readiness gaps

- Decide and document the support policy for user override QEMU versions.
- Decide whether managed QEMU artifacts need additional signing or mirroring in
  a later Windows release.
- Expand `lsb doctor` beyond Windows SMB policy if future Windows diagnostics
  need a single umbrella command.
- Decide dedicated self-hosted runner labels before adding more Windows runners
  with the default `self-hosted, Windows, X64` labels.
- Decide whether to add Windows interactive shell/PTY support over the mux.
- Decide whether to migrate port forwarding onto the mux or otherwise support
  concurrent forwarding sessions without the current serialization.
- Decide whether to add a hybrid watch aggregator for recursive watches that
  span guest-only paths and direct SMB targets.
- Decide the post-MVP storage path: CAS/NBD migration, persistent qcow2 chains,
  or another deduplicated checkpoint format.
- Revisit broader live-sharing work such as Windows VirtioFS, 9p, or custom
  sync only if SMB/CIFS direct mounts prove insufficient.
