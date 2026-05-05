use std::net::TcpStream;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

/// Events from an interactive shell session.
#[derive(Debug)]
pub enum ShellEvent {
    /// Terminal output bytes (PTY stdout).
    Output(Vec<u8>),
    /// Process exited with code.
    Exit(i32),
    /// Error from guest.
    Error(String),
}

/// Writer half of a shell session. Cloneable, used to send input and resize.
#[derive(Clone)]
pub struct ShellWriter {
    pub(crate) writer: Arc<std::sync::Mutex<TcpStream>>,
}

impl ShellWriter {
    /// Send input bytes (keystrokes) to the shell.
    pub fn send_input(&self, data: &[u8]) -> Result<()> {
        use std::io::Write;

        let mut w = self.writer.lock().unwrap();
        lsb_proto::frame::write_frame(&mut *w, lsb_proto::frame::STDIN, data)?;
        w.flush()?;
        Ok(())
    }

    /// Send a terminal resize event.
    pub fn resize(&self, rows: u16, cols: u16) -> Result<()> {
        let mut w = self.writer.lock().unwrap();
        let payload = lsb_proto::frame::resize_payload(rows, cols);
        lsb_proto::frame::write_frame(&mut *w, lsb_proto::frame::RESIZE, &payload)?;
        Ok(())
    }
}

/// Reader half of a shell session. Receives output events asynchronously.
pub struct ShellReader {
    pub(crate) output_rx: mpsc::UnboundedReceiver<ShellEvent>,
}

impl ShellReader {
    /// Receive the next shell event. Returns `None` when the session ends.
    pub async fn recv(&mut self) -> Option<ShellEvent> {
        self.output_rx.recv().await
    }
}

/// Handle to an interactive shell session with PTY support.
pub struct ShellHandle {
    pub(crate) writer: ShellWriter,
    pub(crate) reader: ShellReader,
    pub(crate) _reader_thread: std::thread::JoinHandle<()>,
}

impl ShellHandle {
    /// Split into writer (cloneable, for input) and reader (for output).
    pub fn split(self) -> (ShellWriter, ShellReader) {
        std::mem::forget(self._reader_thread);
        (self.writer, self.reader)
    }
}
