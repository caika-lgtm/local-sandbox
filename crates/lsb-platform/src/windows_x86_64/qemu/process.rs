use std::fmt;
use std::path::PathBuf;
use std::time::Duration;

use super::argv::QemuCommand;

pub(crate) const DEFAULT_STARTUP_TIMEOUT: Duration = Duration::from_secs(5);
pub(crate) const DEFAULT_TERMINATE_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum QemuProcessState {
    NotStarted,
    Starting,
    Running,
    Exited,
    Failed,
    Terminated,
}

impl QemuProcessState {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not-started",
            Self::Starting => "starting",
            Self::Running => "running",
            Self::Exited => "exited",
            Self::Failed => "failed",
            Self::Terminated => "terminated",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QemuProcessArtifacts {
    pub directory: PathBuf,
    pub argv: PathBuf,
    pub stdout: PathBuf,
    pub stderr: PathBuf,
    pub status: PathBuf,
}

impl QemuProcessArtifacts {
    pub(crate) fn new(directory: impl Into<PathBuf>) -> Self {
        let directory = directory.into();
        Self {
            argv: directory.join("qemu.argv.redacted.txt"),
            stdout: directory.join("qemu.stdout.log"),
            stderr: directory.join("qemu.stderr.log"),
            status: directory.join("qemu.status.json"),
            directory,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QemuSupervisorConfig {
    pub command: QemuCommand,
    pub artifacts: QemuProcessArtifacts,
    pub working_directory: PathBuf,
    pub startup_timeout: Duration,
    pub terminate_timeout: Duration,
}

impl QemuSupervisorConfig {
    pub(crate) fn new(command: QemuCommand, artifact_directory: impl Into<PathBuf>) -> Self {
        let artifacts = QemuProcessArtifacts::new(artifact_directory);
        Self {
            command,
            working_directory: artifacts.directory.clone(),
            artifacts,
            startup_timeout: DEFAULT_STARTUP_TIMEOUT,
            terminate_timeout: DEFAULT_TERMINATE_TIMEOUT,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QemuExitStatus {
    pub code: Option<i32>,
    pub success: bool,
}

impl fmt::Display for QemuExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(f, "exit code {code}"),
            None if self.success => write!(f, "success"),
            None => write!(f, "terminated by signal or system request"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct QemuSupervisor {
    config: QemuSupervisorConfig,
    state: QemuProcessState,
}

impl QemuSupervisor {
    pub(crate) fn new(config: QemuSupervisorConfig) -> Self {
        Self {
            config,
            state: QemuProcessState::NotStarted,
        }
    }

    pub(crate) fn state(&self) -> QemuProcessState {
        self.state
    }

    pub(crate) fn artifacts(&self) -> &QemuProcessArtifacts {
        &self.config.artifacts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::windows_x86_64::qemu::argv::QemuArgvBuilder;
    use crate::windows_x86_64::qemu::config::QemuBootConfig;

    fn command() -> QemuCommand {
        QemuArgvBuilder::new(QemuBootConfig::direct_linux_boot(
            r"C:\qemu\qemu-system-x86_64.exe",
            r"C:\lsb\Image",
            r"C:\lsb\initramfs.cpio.gz",
            r"C:\lsb\instances\abc\root.qcow2",
            r"C:\lsb\instances\abc\serial.log",
            2048,
            2,
        ))
        .build()
        .expect("test command should build")
    }

    #[test]
    fn artifact_paths_are_deterministic() {
        let artifacts = QemuProcessArtifacts::new(r"C:\lsb\instances\abc\diagnostics");

        assert_eq!(
            artifacts.argv,
            PathBuf::from(r"C:\lsb\instances\abc\diagnostics").join("qemu.argv.redacted.txt")
        );
        assert_eq!(
            artifacts.stdout,
            PathBuf::from(r"C:\lsb\instances\abc\diagnostics").join("qemu.stdout.log")
        );
        assert_eq!(
            artifacts.stderr,
            PathBuf::from(r"C:\lsb\instances\abc\diagnostics").join("qemu.stderr.log")
        );
        assert_eq!(
            artifacts.status,
            PathBuf::from(r"C:\lsb\instances\abc\diagnostics").join("qemu.status.json")
        );
    }

    #[test]
    fn supervisor_starts_in_not_started_state() {
        let supervisor = QemuSupervisor::new(QemuSupervisorConfig::new(command(), "artifacts"));

        assert_eq!(supervisor.state(), QemuProcessState::NotStarted);
        assert_eq!(
            supervisor.artifacts().argv,
            PathBuf::from("artifacts").join("qemu.argv.redacted.txt")
        );
    }
}
