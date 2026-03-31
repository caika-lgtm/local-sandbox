binary := "target/debug/shuru"

# List available recipes
default:
    @just --list

# Build the guest init binary for the host platform by default.
# Set SHURU_PLATFORM=<platform-id> to cross-build.
build-guest:
    SHURU_PLATFORM="${SHURU_PLATFORM:-}" cargo run -p xtask -- build-guest

# Build the guest kernel image via xtask.
# Set SHURU_PLATFORM=<platform-id> to override the host default.
build-kernel:
    SHURU_PLATFORM="${SHURU_PLATFORM:-}" cargo run -p xtask -- build-kernel

# Build the CLI binary (debug)
build-cli:
    cargo build -p shuru-cli

# Codesign the CLI binary with the selected platform entitlement.
codesign:
    codesign --entitlements "$(SHURU_PLATFORM="${SHURU_PLATFORM:-}" cargo run -q -p xtask -- platform-meta --format env | sed -n 's/^SHURU_CODESIGN_ENTITLEMENTS=//p')" --force -s - {{ binary }}

# Build everything: guest + CLI + codesign
build: build-guest build-cli codesign

# Prepare the rootfs, kernel, and initramfs (requires Docker).
# Set SHURU_PLATFORM=<platform-id> to override the host default.
prepare-rootfs:
    SHURU_PLATFORM="${SHURU_PLATFORM:-}" cargo run -p xtask -- prepare-rootfs

# Run a command inside the VM
run *args:
    {{ binary }} run -- {{ args }}

# Open an interactive shell in the VM
shell:
    {{ binary }} run -- sh

# Full setup from scratch: rootfs + build
setup: prepare-rootfs build

# Check all crates compile (host targets only)
check:
    cargo check --workspace

# Run clippy on all crates
clippy:
    cargo clippy --workspace

# Install the binary to ~/.local/bin with codesign
install: build-guest
    cargo build -p shuru-cli --release
    codesign --entitlements "$(SHURU_PLATFORM="${SHURU_PLATFORM:-}" cargo run -q -p xtask -- platform-meta --format env | sed -n 's/^SHURU_CODESIGN_ENTITLEMENTS=//p')" --force -s - target/release/shuru
    mkdir -p ~/.local/bin
    cp target/release/shuru ~/.local/bin/shuru

# Tag and push a release (triggers GitHub Actions)
release version:
    git tag -a "v{{ version }}" -m "Release v{{ version }}"
    git push origin "v{{ version }}"
