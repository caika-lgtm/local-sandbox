#![deny(clippy::all)]
#![allow(non_snake_case)]

// Keep this crate root thin: N-API exports are implemented in focused modules
// and re-exported here so napi-rs still generates a single public JS surface.
mod config;
mod error;
mod process;
mod sandbox;
mod streams;
mod types;

pub use process::SpawnedProcess;
pub use sandbox::Sandbox;
pub use streams::{ByteStream, WatchStream};
pub use types::{
  CopyOptions, DirEntry, ExecOptions, ExecResult, ExposeHostConfig, FileChangeEvent, MkdirOptions,
  MountConfig, NetworkConfig, PortMappingConfig, RemoveOptions, SecretConfig, SpawnOptions,
  StartOptions, StatResult, WatchOptions,
};
