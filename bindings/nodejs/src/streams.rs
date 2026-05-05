use napi_derive::napi;

#[cfg(lsb_nodejs_supported)]
use napi::bindgen_prelude::AsyncGenerator;
#[cfg(lsb_nodejs_supported)]
use napi::bindgen_prelude::{Buffer, Result};
#[cfg(lsb_nodejs_supported)]
use std::sync::Arc;
#[cfg(lsb_nodejs_supported)]
use tokio::sync::mpsc;

#[cfg(lsb_nodejs_supported)]
use crate::error::to_napi_error;
#[cfg(lsb_nodejs_supported)]
use crate::types::FileChangeEvent;

// Async iterator wrappers bridge SDK mpsc receivers into JavaScript
// `for await ... of` streams without buffering in the binding layer.
/// Async byte stream returned by process stdout and stderr.
///
/// Usage: `for await (const chunk of proc.stdout) process.stdout.write(chunk)`
#[cfg_attr(lsb_nodejs_supported, napi(async_iterator))]
#[cfg_attr(not(lsb_nodejs_supported), napi)]
pub struct ByteStream {
  #[cfg(lsb_nodejs_supported)]
  pub(crate) receiver: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<Vec<u8>>>>,
}

#[cfg(lsb_nodejs_supported)]
impl AsyncGenerator for ByteStream {
  type Yield = Buffer;
  type Next = ();
  type Return = ();

  fn next(
    &mut self,
    _value: Option<Self::Next>,
  ) -> impl std::future::Future<Output = Result<Option<Self::Yield>>> + Send + 'static {
    let receiver = self.receiver.clone();
    async move {
      let mut receiver = receiver.lock().await;
      Ok(receiver.recv().await.map(Buffer::from))
    }
  }
}

/// Async stream of file change events returned by `sandbox.watch`.
///
/// Usage: `for await (const event of await sandbox.watch('/workspace')) console.log(event.path)`
#[cfg_attr(lsb_nodejs_supported, napi(async_iterator))]
#[cfg_attr(not(lsb_nodejs_supported), napi)]
pub struct WatchStream {
  #[cfg(lsb_nodejs_supported)]
  pub(crate) receiver:
    Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<anyhow::Result<lsb_sdk::WatchEvent>>>>,
}

#[cfg(lsb_nodejs_supported)]
impl AsyncGenerator for WatchStream {
  type Yield = FileChangeEvent;
  type Next = ();
  type Return = ();

  fn next(
    &mut self,
    _value: Option<Self::Next>,
  ) -> impl std::future::Future<Output = Result<Option<Self::Yield>>> + Send + 'static {
    let receiver = self.receiver.clone();
    async move {
      let mut receiver = receiver.lock().await;
      match receiver.recv().await {
        Some(Ok(event)) => Ok(Some(FileChangeEvent {
          path: event.path,
          event: event.event,
        })),
        Some(Err(error)) => Err(to_napi_error(error)),
        None => Ok(None),
      }
    }
  }
}
