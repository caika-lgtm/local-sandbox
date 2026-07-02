# Self-Hosted Windows 11 Runner Setup Notes

This document is a placeholder for the maintainer-owned Windows 11 WHPX runner. Fill in exact organization/repository commands after the runner exists.

## Intended runner labels

Initial label proposal:

```text
self-hosted, windows, x64, whpx, local-sandbox
```

Update `validation.md` and CI workflows if the final labels differ.

## Runner requirements

- Windows 11 x86_64 host.
- Hardware virtualization enabled in firmware.
- Windows Hypervisor Platform enabled.
- Hyper-V compatible configuration sufficient for QEMU WHPX.
- QEMU installed and discoverable by either `LSB_QEMU` or `PATH`.
- Rust toolchain matching repository expectations.
- Node toolchain for M14 and later.
- Git configured for long paths if the repository needs it.
- LocalSandbox guest assets available or buildable by CI.

## Suggested environment variables

```powershell
$env:LSB_QEMU="C:\Program Files\qemu\qemu-system-x86_64.exe"
$env:LSB_WINDOWS_INTEGRATION="1"
```

Do not store secrets in runner-level environment variables unless the CI job explicitly requires them and masks them.

## Preflight checklist

Record output in a secure maintainer note or CI artifact after M02 exists.

```powershell
systeminfo
where qemu-system-x86_64
qemu-system-x86_64 --version
cargo --version
rustc --version
node --version
npm --version
```

After M02:

```powershell
lsb doctor windows
```

## CI safety

- Do not run untrusted pull request code on the self-hosted runner unless repository policy allows it.
- Prefer maintainer-triggered integration jobs for branches under review.
- Upload redacted artifacts only.
- Periodically clean LocalSandbox debug/temp directories.
- Ensure QEMU processes are not left running after failed jobs.

## Artifact retention

For failed WHPX jobs, retain:

- redacted QEMU argv,
- serial log,
- QEMU stderr/stdout,
- preflight output,
- host LocalSandbox logs,
- test report.

Do not retain secret-bearing env dumps or unredacted proxy logs.
