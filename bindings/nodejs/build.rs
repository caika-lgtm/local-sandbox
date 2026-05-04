fn main() {
  println!("cargo:rustc-check-cfg=cfg(lsb_nodejs_supported)");

  let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
  let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

  if target_os == "macos" && matches!(target_arch.as_str(), "aarch64" | "x86_64") {
    println!("cargo:rustc-cfg=lsb_nodejs_supported");
  }

  napi_build::setup();
}
