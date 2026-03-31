#![forbid(unsafe_code)]

#[cfg(not(target_os = "macos"))]
compile_error!(
    "shuru-vm currently only supports macOS hosts. Future platform slots exist in shuru-platform, but their runtimes are not implemented yet."
);

mod sandbox;

pub use sandbox::{MountConfig, PortForwardHandle, Sandbox, VmConfigBuilder};
pub use shuru_platform::VmState;
pub use shuru_proto::{
    frame, ExecRequest, ForwardRequest, ForwardResponse, MountRequest, MountResponse, PortMapping,
    ReadFileRequest, WriteFileRequest, WriteFileResponse, VSOCK_PORT, VSOCK_PORT_FORWARD,
};

pub fn default_data_dir() -> String {
    shuru_platform::default_data_dir()
}
