# PLAN: Managed Windows QEMU Host Tool Install

## Goal

Implement Windows QEMU packaging as a LocalSandbox-managed host tool installed by
`lsb init`.

The maintained behavior should be:

- On Windows x86_64, `lsb init` initializes both runtime boot assets and the
  managed QEMU host tool package.
- QEMU remains a host tool, not part of the guest OS image assets.
- `LSB_QEMU` and `LSB_QEMU_IMG` remain supported overrides.
- Normal Windows execution still requires WHPX; no TCG fallback is introduced.
- The self-hosted Windows WHPX smoke/e2e workflow validates the managed QEMU
  package rather than relying on `C:\Program Files\qemu`.

The maintainer will curate the slim x86_64-only QEMU artifact. The coding agent
should implement the downloader, installer, discovery path, docs, workflows, and
tests assuming that artifact already exists.

## Maintainer Artifact Contract

Make the curated QEMU artifact available as a GitHub Release asset in the
existing `LocalSandBox/local-sandbox` repository.

Recommended location:

```text
Release tag:
  qemu-windows-x86_64-v<QEMU_VERSION>-<PACKAGE_REVISION>

Asset:
  lsb-qemu-windows-x86_64-qemu-<QEMU_VERSION>-<PACKAGE_REVISION>.tar.gz

Current uploaded artifact:
  QEMU version: 11.0.50
  LSB version: 0.4.0
  package revision: lsb0.4.0
  release tag: qemu-windows-x86_64-v11.0.50-lsb0.4.0
  asset: lsb-qemu-windows-x86_64-qemu-11.0.50-lsb0.4.0.tar.gz
  URL: https://github.com/caika-lgtm/local-sandbox/releases/download/qemu-windows-x86_64-v11.0.50-lsb0.4.0/lsb-qemu-windows-x86_64-qemu-11.0.50-lsb0.4.0.tar.gz
  sha256: 49021ed8481ad8bc3e2d71ab3d088e60414ec2bb78654c96f6da33b2dd0c6251
```

Use a dedicated QEMU release tag instead of attaching the tool artifact to every
LocalSandbox product release. This keeps `lsb init --version <runtime-version>`
focused on guest runtime assets while the current CLI always installs the single
managed QEMU package pinned in code.

The artifact should extract to a single top-level directory:

```text
qemu-<QEMU_VERSION>-<PACKAGE_REVISION>/
  qemu-system-x86_64.exe
  qemu-img.exe
  *.dll
  lib/
    ...
  share/
    qemu/
      ...
  COPYING
  COPYING.LIB
  VERSION
  README.rst
  manifest.json
```

This artifact layout intentionally keeps the curated QEMU files in their
upstream/packaged relative locations. Do not require or move executables into a
`bin/` directory. The implementation should read executable relative paths from
`manifest.json`; for the current curated package, both executables are at the
package root.

`manifest.json` should include at least:

- `schema_version`
- `package_version`, currently `qemu-11.0.50-lsb0.4.0`
- `qemu_version`, currently `11.0.50`
- `lsb_version`, currently `0.4.0`
- `platform`, exactly `windows-x86_64`
- `qemu_system_x86_64`, relative path to the emulator executable; current value:
  `qemu-system-x86_64.exe`
- `qemu_img`, relative path to the image utility executable; current value:
  `qemu-img.exe`
- `files` with relative paths, sizes, and sha256 values
- license/source provenance fields sufficient for GPL and bundled dependency
  notices

Also publish the artifact sha256. The implementation should pin the expected
sha256 in code or in a checked-in metadata file so `lsb init` verifies the
download before extraction.

## Proposed Installed Layout

Install under the existing Windows data directory:

```text
%LOCALAPPDATA%\lsb\
  tools\
    qemu\
      qemu-<QEMU_VERSION>-<PACKAGE_REVISION>\
        qemu-system-x86_64.exe
        qemu-img.exe
        lib\...
        share\...
        manifest.json
      current.json
```

`current.json` should point to the active managed package and record:

- package version
- artifact URL
- artifact sha256
- install time
- resolved `qemu-system-x86_64.exe` path
- resolved `qemu-img.exe` path

Do not add the managed QEMU directory to the user or system `PATH`. The backend
should execute QEMU by absolute path.

## User-Facing Behavior

`lsb init` on Windows x86_64 should:

1. Ensure the managed QEMU package is installed and valid.
2. Download/extract it if missing, invalid, or `--force` is passed.
3. Initialize OS image assets using the existing version behavior.
4. Print concise status lines for both host tools and OS image assets.

Example output shape:

```text
lsb: initializing Windows host tools...
lsb: QEMU host tools installed (qemu-11.0.50-lsb0.4.0)
lsb: initializing OS image (v0.3.12)...
lsb: OS image already up to date (0.3.12)
```

Add a hidden or explicitly documented CI/developer flag if needed:

```text
lsb init --host-tools-only
```

This is useful because the self-hosted branch workflow currently prepares boot
assets from source or cache instead of downloading released OS image assets for
the current branch version. If the flag is added, it should still be Windows-only
and should not change normal user behavior.

## Discovery Order

Update QEMU discovery to this order:

1. `LSB_QEMU`
2. internal/configured QEMU path
3. LocalSandbox managed QEMU path under the active data directory
4. `PATH`

Update `qemu-img.exe` discovery to this order:

1. `LSB_QEMU_IMG`
2. sibling of `LSB_QEMU`
3. sibling of internal/configured QEMU path, if available
4. sibling of LocalSandbox managed QEMU path
5. `PATH`

Add a `managed` source to discovery/preflight diagnostics so support reports can
distinguish user-installed QEMU from LocalSandbox-managed QEMU.

## Implementation Areas

### SDK/CLI asset initialization

Files to inspect/change:

- `crates/lsb-cli/src/cli.rs`
- `crates/lsb-cli/src/assets.rs`
- `crates/lsb-cli/src/main.rs`
- `crates/lsb-sdk/src/assets.rs`
- possibly a new `crates/lsb-sdk/src/host_tools.rs`

Tasks:

- Add a Windows-only managed host tool initializer.
- Keep OS image asset initialization behavior unchanged on macOS.
- Make `SandboxInitResult` report host-tool status if the SDK API needs to
  expose it; otherwise keep the public result stable and only print CLI status.
- Ensure `--force` revalidates/reinstalls the managed QEMU package on Windows.
- Keep SDK `AsyncSandbox::boot` from downloading implicitly. Initialization
  remains explicit through `initSandbox()` or `lsb init`.

### Platform paths and discovery

Files to inspect/change:

- `crates/lsb-platform/src/lib.rs`
- `crates/lsb-platform/src/windows_x86_64/qemu/discovery.rs`
- `crates/lsb-platform/src/windows_x86_64/qemu/preflight.rs`
- `crates/lsb-platform/src/windows_x86_64/qemu/version.rs`
- `crates/lsb-platform/src/windows_x86_64/qemu/process.rs`
- `crates/lsb-store/src/windows_checkpoint.rs`

Tasks:

- Add managed host-tool path helpers without overloading `AssetPaths`, which is
  currently guest runtime asset-focused.
- Add managed QEMU discovery and diagnostics.
- Make `qemu-img.exe` use the same managed install.
- Preserve existing env/config/PATH behavior for users and CI overrides.
- Preserve WHPX preflight and production `-accel whpx` behavior.

### Archive extraction and validation

Tasks:

- Download to a temporary staging file under the data directory or system temp.
- Verify sha256 before extraction.
- Extract into a temporary install directory, then atomically move or rename into
  `%LOCALAPPDATA%\lsb\tools\qemu\...`.
- Reject archive entries with absolute paths, `..`, path separators that escape
  the target, or unsupported Windows path prefixes.
- Validate required files exist after extraction:
  - manifest `qemu_system_x86_64` path, currently `qemu-system-x86_64.exe`
  - manifest `qemu_img` path, currently `qemu-img.exe`
  - `manifest.json`
  - license notice files, currently `COPYING`, `COPYING.LIB`, `VERSION`, and
    `README.rst`
- Probe `qemu-system-x86_64.exe --version`, `--help`, and `-accel help` through
  the existing preflight path.
- Probe `qemu-img.exe --version` or `qemu-img.exe info --help` for checkpoint
  support.
- Write `current.json` only after all validation succeeds.
- Do not delete older managed QEMU directories automatically in the first pass;
  stale cleanup can be future work unless disk growth becomes a concern.

### Release metadata

Files to inspect/change:

- `crates/lsb-platform/src/lib.rs`
- `xtask/src/release.rs`
- `xtask/src/main.rs`
- optional new checked-in metadata file such as
  `crates/lsb-platform/assets/windows-managed-qemu.json`

Tasks:

- Add one source of truth for:
  - managed QEMU package version
  - GitHub release tag
  - tarball name
  - sha256
  - expected top-level directory
- Prefer checked-in metadata if that makes workflow validation and docs easier.
- Update `xtask platform-meta` to print managed QEMU metadata for
  `windows-x86_64`.
- Do not make `xtask package-release` build QEMU. The maintainer owns curation.

## Workflow Updates

### `.github/workflows/windows-lsb-hardware.yml`

Update the self-hosted Windows workflow so smoke/e2e validates the managed QEMU
path.

Required changes:

- Remove `C:\Program Files\qemu` from required PATH setup.
- Add a step before QEMU version display and before `scripts/windows-smoke.ps1`
  / `scripts/windows-e2e.ps1`:

```powershell
cargo run -p lsb-cli -- init --host-tools-only --force
```

- Change the environment display step to print the managed QEMU paths. Either:
  - add a small `xtask` helper to print paths from the managed metadata, or
  - make `lsb init --host-tools-only` print the resolved paths, or
  - use `current.json` from `%LOCALAPPDATA%\lsb\tools\qemu\current.json`.
- Ensure `qemu-system-x86_64 --version` is not the only proof, because the
  managed tool directory should not be globally added to PATH.
- Keep WHPX runtime proof in the existing smoke/e2e tests.
- Keep no `pull_request` trigger for this workflow.

### `.github/workflows/ci.yml`

Add hosted Windows coverage that does not require WHPX runtime boot:

- Unit tests for managed metadata parsing.
- Unit tests for safe extraction using local test archives.
- Discovery/preflight tests proving managed path precedence.
- Optional network-disabled test using a local file fixture rather than the real
  GitHub artifact.

Do not make hosted CI depend on external QEMU downloads unless there is a
separate opt-in integration lane.

### `.github/workflows/release.yml`

Do not build the QEMU package in the normal release workflow.

Add a lightweight validation step for Windows releases:

- Print the managed QEMU metadata from `xtask platform-meta --platform
windows-x86_64 --format env`.
- Verify the configured QEMU artifact URL exists and its sha256 matches the
  pinned value. This can run with a small download if artifact size is acceptable
  for release jobs; otherwise verify a sidecar checksum file.
- Fail the product release if the pinned managed QEMU artifact is unavailable.

If the maintainer later wants every product release to mirror the QEMU artifact,
add a copy/mirror step then. Do not start with that complexity.

### Node release and binding workflows

Files to inspect/change:

- `.github/workflows/nodejs-binding.yml`
- `.github/workflows/release_nodejs.yml`
- `bindings/nodejs/scripts/windows-preflight-smoke.mjs`
- `bindings/nodejs/README.md`

Tasks:

- Ensure Node smoke/preflight docs and tests expect managed QEMU after
  `initSandbox()` or `lsb init`.
- Do not bundle QEMU inside the npm package in this feature.
- Confirm Windows Node package load errors still distinguish native binding
  missing from QEMU/WHPX/runtime initialization failures.

## Documentation Updates

Update these docs as part of implementation:

- `README.md`
  - Windows prerequisites should require Windows 11 x64 and WHPX, but not a
    separately installed QEMU.
  - Explain that `lsb init` installs managed QEMU host tools and runtime assets.
  - Keep `LSB_QEMU` and `LSB_QEMU_IMG` documented as override/debug paths.

- `bindings/nodejs/README.md`
  - Replace "QEMU is not bundled/install separately" with "initialize host tools
    through `initSandbox()` or `lsb init`".
  - State npm packages do not contain QEMU.

- `install.ps1` and `install.sh`
  - Keep installers focused on CLI installation unless explicitly changed.
  - Update post-install messaging to tell Windows users to run `lsb init`.

- `docs/windows-port/README.md`
  - Current status should say managed QEMU is installed by `lsb init`.

- `docs/windows-port/mvp-handoff.md`
  - Move "No bundled QEMU or QEMU installer" from active limitation to historical
    context or replace with the managed-host-tool behavior.
  - Update open gaps around QEMU version policy.

- `docs/windows-port/decisions.md`
  - Add a new decision, for example:
    `D023: lsb init installs a managed Windows QEMU host tool package`.
  - Keep D004 as MVP history or update with a note that env/PATH discovery is
    now fallback/override behavior.

- `docs/windows-port/architecture.md`
  - Add managed host tool package to the Windows backend architecture.

- `docs/windows-port/validation.md`
  - Add managed QEMU install validation and self-hosted smoke/e2e requirements.

- `docs/windows-port/runner-setup.md`
  - Remove persistent runner QEMU install as a prerequisite.
  - Keep WHPX/Hyper-V-compatible host setup as a prerequisite.

- `docs/windows-port/diagnostics.md`
  - Document managed QEMU paths, manifest, and failure modes.

- `docs/windows-port/risk-register.md`
  - Update QEMU provenance/security risk from user-installed mitigation to
    managed artifact hash/provenance/notice validation.

- `docs/windows-port/security-checklist.md`
  - Add checks for archive extraction safety, pinned hashes, license notices,
    managed path ownership, and no global PATH mutation.

- `docs/windows-port/future-work.md`
  - Remove or update "evaluate whether to bundle/sign QEMU" now that managed
    install is the chosen path.

The RFC is historical. Avoid large rewrites to
`docs/windows-port/rfc-qemu-whpx.md`; add a short post-MVP note only if needed.

## Test Plan

### Unit tests

Add tests for:

- Managed metadata URL/name generation.
- `lsb init` host-tool status on non-Windows as no-op.
- Windows managed QEMU install idempotency.
- `--force` reinstall path.
- Hash mismatch rejection.
- Missing required binary rejection.
- Archive traversal rejection.
- Manifest parse and validation.
- QEMU discovery precedence:
  - env beats managed
  - config beats managed
  - managed beats PATH
  - PATH remains fallback
- `qemu-img.exe` discovery from managed path.
- Diagnostics report `managed` source.

### Hosted CI checks

Run:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
```

On hosted Windows, also run targeted managed-QEMU unit tests that do not require
WHPX.

### Self-hosted WHPX validation

After implementation and after the maintainer artifact is published, run:

```bash
./scripts/win-gh-test smoke
./scripts/win-gh-test e2e
```

The self-hosted jobs must:

- start without `C:\Program Files\qemu` on PATH,
- install managed QEMU through `lsb init`,
- show the managed package version in diagnostics,
- pass real QEMU/WHPX preflight,
- pass existing boot, exec, file copy, mount, port-forward, checkpoint, network
  policy, and Node smoke/e2e coverage.

## Acceptance Criteria

- Fresh Windows 11 x64 user flow:
  1. Install `lsb.exe`.
  2. Enable Windows Hypervisor Platform.
  3. Run `lsb init`.
  4. Run `lsb run -- echo hello`.
     No separate QEMU install or PATH edit is required.

- `LSB_QEMU` and `LSB_QEMU_IMG` still work for debugging and override managed
  QEMU.
- Windows checkpoint operations find `qemu-img.exe` from the managed package.
- Diagnostics identify whether QEMU came from env/config/managed/PATH.
- Hosted CI covers the downloader, extraction, metadata, and discovery logic.
- Self-hosted WHPX smoke/e2e proves the managed QEMU package boots the current
  Windows runtime assets.
- Docs no longer tell Windows users to install QEMU separately for the standard
  path.

## Non-Goals

- Do not bundle QEMU inside the CLI archive.
- Do not bundle QEMU inside the OS image/runtime asset tarball.
- Do not bundle QEMU inside npm packages.
- Do not add TCG fallback.
- Do not support Windows ARM64.
- Do not implement automatic QEMU auto-update separate from `lsb init`.
- Do not globally modify user/system PATH.
- Do not remove env/config/PATH fallback discovery.

## Suggested Implementation Sequence

1. Add managed QEMU metadata constants or checked-in metadata.
2. Add path helpers and installer/extractor with tests.
3. Wire `lsb init` to install host tools on Windows.
4. Add managed QEMU discovery and `qemu-img.exe` discovery.
5. Add diagnostics/source reporting.
6. Update Windows hardware workflow and scripts to use managed QEMU.
7. Update hosted CI tests.
8. Update README, Node README, install messages, and Windows port docs.
9. Run hosted checks.
10. Publish/verify maintainer artifact.
11. Run self-hosted WHPX smoke/e2e.
