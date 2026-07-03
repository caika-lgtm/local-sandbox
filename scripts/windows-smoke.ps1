$ErrorActionPreference = "Stop"

Write-Host "== Windows LSB smoke test =="

cargo run -p lsb-cli -- --help

Write-Host "== Windows QEMU preflight smoke =="
$env:LSB_TEST_REAL_QEMU = "1"
cargo test -p lsb-platform real_qemu_preflight_when_explicitly_enabled -- --ignored --nocapture

$bootVars = @(
  "LSB_WINDOWS_BOOT_KERNEL",
  "LSB_WINDOWS_BOOT_INITRD",
  "LSB_WINDOWS_BOOT_ROOTFS"
)
$missingBootVars = @($bootVars | Where-Object { -not [Environment]::GetEnvironmentVariable($_) })
if ($missingBootVars.Count -eq 0) {
  Write-Host "== Windows QEMU direct boot smoke =="
  cargo test -p lsb-platform windows_qemu_boot_smoke -- --ignored --nocapture
} else {
  Write-Warning "Skipping Windows QEMU direct boot smoke. Set $($missingBootVars -join ', ') to disposable LocalSandbox boot asset paths."
}

# Later:
# cargo run -p lsb-cli -- run --cpus 2 --memory 2048 -- echo resource-test
# cargo run -p lsb-cli -- run --port 8080:8080 -- your-port-forward-test
