use std::io::BufReader;
use std::net::TcpStream;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::WatchEvent;

/// Handle to a file watch stream in the VM.
pub struct WatchHandle {
    events_rx: mpsc::UnboundedReceiver<Result<WatchEvent>>,
}

impl WatchHandle {
    pub async fn next(&mut self) -> Option<Result<WatchEvent>> {
        self.events_rx.recv().await
    }

    pub fn into_events(self) -> mpsc::UnboundedReceiver<Result<WatchEvent>> {
        self.events_rx
    }
}

pub(crate) fn spawn_watch_thread(stream: TcpStream) -> WatchHandle {
    let (events_tx, events_rx) = mpsc::unbounded_channel();

    let _ = std::thread::Builder::new()
        .name("lsb-watch".into())
        .spawn(move || {
            let mut reader = BufReader::new(stream);
            loop {
                match lsb_proto::frame::read_frame(&mut reader) {
                    Ok(Some((lsb_proto::frame::WATCH_EVENT, payload))) => {
                        let result = serde_json::from_slice::<lsb_proto::WatchEvent>(&payload)
                            .map(|event| WatchEvent {
                                path: event.path,
                                event: event.event,
                            })
                            .map_err(anyhow::Error::from);
                        if events_tx.send(result).is_err() {
                            break;
                        }
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => break,
                    Err(error) => {
                        let _ = events_tx.send(Err(error.into()));
                        break;
                    }
                }
            }
        });

    WatchHandle { events_rx }
}
