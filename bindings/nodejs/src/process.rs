use napi::bindgen_prelude::{Either, Result, Uint8Array};
use napi_derive::napi;

#[cfg(lsb_nodejs_supported)]
use napi::{Error, Status};
#[cfg(lsb_nodejs_supported)]
use std::sync::{Arc, Mutex};
#[cfg(lsb_nodejs_supported)]
use tokio::sync::{mpsc, watch};

#[cfg(lsb_nodejs_supported)]
use crate::error::to_napi_error;
#[cfg(not(lsb_nodejs_supported))]
use crate::error::unsupported_platform_error;
use crate::streams::ByteStream;

// Holds the process handle plus one-shot stdout/stderr receivers. The stream
// getters clone Arcs to expose each receiver through async iterators.
#[cfg(lsb_nodejs_supported)]
struct SpawnedProcessState {
  process: Mutex<lsb_sdk::ProcessHandle>,
  stdout: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<Vec<u8>>>>,
  stderr: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<Vec<u8>>>>,
}

/// Handle for a process spawned inside the sandbox.
///
/// Usage: `const proc = await sandbox.spawn('npm test')`
#[napi]
pub struct SpawnedProcess {
  #[cfg(lsb_nodejs_supported)]
  state: Arc<SpawnedProcessState>,
}

#[cfg(lsb_nodejs_supported)]
impl SpawnedProcess {
  // take_stdout/take_stderr can only be called once, so construction captures
  // both streams before the handle is shared with JS methods.
  pub(crate) fn from_process(mut process: lsb_sdk::ProcessHandle) -> Result<Self> {
    let stdout = process
      .take_stdout()
      .ok_or_else(|| Error::new(Status::GenericFailure, "stdout stream was already taken"))?;
    let stderr = process
      .take_stderr()
      .ok_or_else(|| Error::new(Status::GenericFailure, "stderr stream was already taken"))?;

    Ok(Self {
      state: Arc::new(SpawnedProcessState {
        process: Mutex::new(process),
        stdout: Arc::new(tokio::sync::Mutex::new(stdout)),
        stderr: Arc::new(tokio::sync::Mutex::new(stderr)),
      }),
    })
  }
}

#[napi]
impl SpawnedProcess {
  /// Stream stdout chunks from the spawned process.
  ///
  /// Usage: `for await (const chunk of proc.stdout) process.stdout.write(chunk)`
  #[napi(getter)]
  pub fn stdout(&self) -> Result<ByteStream> {
    #[cfg(lsb_nodejs_supported)]
    {
      return Ok(ByteStream {
        receiver: self.state.stdout.clone(),
      });
    }

    #[cfg(not(lsb_nodejs_supported))]
    {
      Err(unsupported_platform_error())
    }
  }

  /// Stream stderr chunks from the spawned process.
  ///
  /// Usage: `for await (const chunk of proc.stderr) process.stderr.write(chunk)`
  #[napi(getter)]
  pub fn stderr(&self) -> Result<ByteStream> {
    #[cfg(lsb_nodejs_supported)]
    {
      return Ok(ByteStream {
        receiver: self.state.stderr.clone(),
      });
    }

    #[cfg(not(lsb_nodejs_supported))]
    {
      Err(unsupported_platform_error())
    }
  }

  /// Write data to the process stdin.
  ///
  /// Usage: `proc.write('hello')`
  #[napi]
  pub fn write(&self, data: Either<String, Uint8Array>) -> Result<()> {
    #[cfg(lsb_nodejs_supported)]
    {
      let bytes = match data {
        Either::A(text) => text.into_bytes(),
        Either::B(buffer) => buffer.to_vec(),
      };
      let process = self.state.process.lock().unwrap();
      return process.write(&bytes).map_err(to_napi_error);
    }

    #[cfg(not(lsb_nodejs_supported))]
    {
      let _ = data;
      Err(unsupported_platform_error())
    }
  }

  /// Request termination of the process.
  ///
  /// Usage: `proc.kill()`
  #[napi]
  pub fn kill(&self) -> Result<()> {
    #[cfg(lsb_nodejs_supported)]
    {
      let process = self.state.process.lock().unwrap();
      return process.kill().map_err(to_napi_error);
    }

    #[cfg(not(lsb_nodejs_supported))]
    {
      Err(unsupported_platform_error())
    }
  }

  /// Promise resolving to the process exit code.
  ///
  /// Usage: `const exitCode = await proc.exited`
  #[napi(getter)]
  pub async fn exited(&self) -> Result<i32> {
    #[cfg(lsb_nodejs_supported)]
    {
      let mut rx = {
        let process = self.state.process.lock().unwrap();
        process.exit_watcher()
      };
      return wait_for_exit(&mut rx).await;
    }

    #[cfg(not(lsb_nodejs_supported))]
    {
      Err(unsupported_platform_error())
    }
  }
}

#[cfg(lsb_nodejs_supported)]
async fn wait_for_exit(rx: &mut watch::Receiver<Option<i32>>) -> Result<i32> {
  if let Some(code) = *rx.borrow() {
    return Ok(code);
  }

  loop {
    if rx.changed().await.is_err() {
      return Err(Error::new(
        Status::GenericFailure,
        "process exit watcher closed unexpectedly",
      ));
    }

    if let Some(code) = *rx.borrow() {
      return Ok(code);
    }
  }
}
