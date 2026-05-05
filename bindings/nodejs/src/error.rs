use napi::{Error, Status};

// Non-macOS builds compile enough of the package to install and report a clear
// runtime error, but they do not link lsb_sdk or boot VMs.
#[cfg(not(lsb_nodejs_supported))]
pub(crate) fn unsupported_platform_error() -> Error {
  Error::new(
    Status::GenericFailure,
    "lsb native bindings currently support only macOS on x86_64 and Apple Silicon (arm64). This package may install elsewhere, but Sandbox.start() is unsupported there.".to_string(),
  )
}

#[cfg(lsb_nodejs_supported)]
pub(crate) fn to_napi_error(error: impl std::fmt::Display) -> Error {
  Error::new(Status::GenericFailure, error.to_string())
}
