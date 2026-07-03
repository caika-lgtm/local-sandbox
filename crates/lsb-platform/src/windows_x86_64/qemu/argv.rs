use std::ffi::OsString;
use std::fmt;
use std::path::{Path, PathBuf};

use super::config::{
    QemuBootConfig, QemuControlChannelConfig, QemuDiskConfig, QemuKernelAppend, QemuQmpEndpoint,
    QemuSerialConfig, CONTROL_BUS_ID, CONTROL_CHARDEV_ID, DEFAULT_CPU_MODEL, DEFAULT_MACHINE_TYPE,
    ROOT_DRIVE_ID,
};
use super::preflight::PRODUCTION_ACCELERATOR;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QemuCommand {
    pub program: PathBuf,
    pub argv: Vec<OsString>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QemuArgvBuilder {
    config: QemuBootConfig,
}

impl QemuArgvBuilder {
    pub(crate) fn new(config: QemuBootConfig) -> Self {
        Self { config }
    }

    pub(crate) fn build(&self) -> Result<QemuCommand, QemuArgvError> {
        validate_path("qemu_executable", &self.config.qemu_executable)?;
        validate_path("kernel_image", &self.config.kernel_image)?;
        validate_path("initrd_image", &self.config.initrd_image)?;
        validate_path("root_disk.path", &self.config.root_disk.path)?;
        validate_machine(&self.config)?;

        let mut argv = Vec::new();
        push_arg(&mut argv, "-nodefaults");
        push_pair(
            &mut argv,
            "-machine",
            format!("{DEFAULT_MACHINE_TYPE},accel={PRODUCTION_ACCELERATOR}"),
        );
        push_pair(&mut argv, "-cpu", DEFAULT_CPU_MODEL);
        push_pair(
            &mut argv,
            "-smp",
            self.config.machine.vcpu_count.to_string(),
        );
        push_pair(
            &mut argv,
            "-m",
            format!("{}M", self.config.machine.memory_mib),
        );
        push_arg(&mut argv, "-no-reboot");
        push_pair(&mut argv, "-display", "none");
        push_pair(&mut argv, "-monitor", "none");
        push_path_pair(&mut argv, "-kernel", &self.config.kernel_image);
        push_path_pair(&mut argv, "-initrd", &self.config.initrd_image);
        push_pair(
            &mut argv,
            "-append",
            kernel_append(
                &self.config.kernel_append,
                self.config.control_channel.is_some(),
            )?,
        );
        push_pair(&mut argv, "-drive", drive_arg(&self.config.root_disk)?);
        push_pair(
            &mut argv,
            "-device",
            format!("virtio-blk-pci,drive={ROOT_DRIVE_ID}"),
        );
        push_serial(&mut argv, &self.config.serial)?;

        if let Some(control_channel) = &self.config.control_channel {
            push_control_channel(&mut argv, control_channel)?;
        }

        if let Some(qmp) = &self.config.qmp {
            push_qmp(&mut argv, qmp)?;
        }

        push_pair(&mut argv, "-nic", "none");

        Ok(QemuCommand {
            program: self.config.qemu_executable.clone(),
            argv,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum QemuArgvError {
    MissingRequiredInput {
        field: &'static str,
    },
    InvalidNumericInput {
        field: &'static str,
        reason: &'static str,
    },
    InvalidKernelAppend {
        field: &'static str,
        reason: &'static str,
    },
    InvalidQemuOptionValue {
        field: &'static str,
        reason: &'static str,
    },
}

impl fmt::Display for QemuArgvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredInput { field } => {
                write!(f, "missing required QEMU argv input: {field}")
            }
            Self::InvalidNumericInput { field, reason } => {
                write!(f, "invalid QEMU argv numeric input {field}: {reason}")
            }
            Self::InvalidKernelAppend { field, reason } => {
                write!(f, "invalid QEMU kernel append input {field}: {reason}")
            }
            Self::InvalidQemuOptionValue { field, reason } => {
                write!(f, "invalid QEMU option value {field}: {reason}")
            }
        }
    }
}

impl std::error::Error for QemuArgvError {}

fn validate_machine(config: &QemuBootConfig) -> Result<(), QemuArgvError> {
    if config.machine.vcpu_count == 0 {
        return Err(QemuArgvError::InvalidNumericInput {
            field: "machine.vcpu_count",
            reason: "must be greater than zero",
        });
    }
    if config.machine.memory_mib == 0 {
        return Err(QemuArgvError::InvalidNumericInput {
            field: "machine.memory_mib",
            reason: "must be greater than zero",
        });
    }
    Ok(())
}

fn validate_path(field: &'static str, path: &Path) -> Result<(), QemuArgvError> {
    if path.as_os_str().is_empty() {
        Err(QemuArgvError::MissingRequiredInput { field })
    } else {
        Ok(())
    }
}

fn push_arg(argv: &mut Vec<OsString>, value: impl Into<OsString>) {
    argv.push(value.into());
}

fn push_pair(argv: &mut Vec<OsString>, flag: &'static str, value: impl Into<OsString>) {
    push_arg(argv, flag);
    push_arg(argv, value);
}

fn push_path_pair(argv: &mut Vec<OsString>, flag: &'static str, path: &Path) {
    push_arg(argv, flag);
    push_arg(argv, path.as_os_str().to_owned());
}

fn kernel_append(
    append: &QemuKernelAppend,
    control_channel_enabled: bool,
) -> Result<String, QemuArgvError> {
    validate_kernel_token("kernel_append.console", &append.console)?;
    validate_kernel_token("kernel_append.root_device", &append.root_device)?;

    let mut values = vec![
        format!("console={}", append.console),
        format!("root={}", append.root_device),
        append.root_mode.as_kernel_arg().to_string(),
    ];

    if let Some(timeout) = append.panic_timeout {
        values.push(format!("panic={timeout}"));
    }

    let guest_transport = append.guest_transport.or_else(|| {
        control_channel_enabled.then_some(super::config::QemuGuestTransport::VirtioSerial)
    });
    if let Some(transport) = guest_transport {
        values.push(transport.as_kernel_arg().to_string());
    }

    Ok(values.join(" "))
}

fn validate_kernel_token(field: &'static str, value: &str) -> Result<(), QemuArgvError> {
    if value.is_empty() {
        return Err(QemuArgvError::InvalidKernelAppend {
            field,
            reason: "must not be empty",
        });
    }
    if value.chars().any(char::is_whitespace) {
        return Err(QemuArgvError::InvalidKernelAppend {
            field,
            reason: "must not contain whitespace",
        });
    }
    Ok(())
}

fn drive_arg(disk: &QemuDiskConfig) -> Result<String, QemuArgvError> {
    Ok(format!(
        "if=none,id={ROOT_DRIVE_ID},file={},format={}",
        path_as_qemu_option_value("root_disk.path", &disk.path)?,
        disk.format.as_qemu_arg()
    ))
}

fn push_serial(argv: &mut Vec<OsString>, serial: &QemuSerialConfig) -> Result<(), QemuArgvError> {
    match serial {
        QemuSerialConfig::File(path) => {
            validate_path("serial.file", path)?;
            push_pair(
                argv,
                "-serial",
                format!("file:{}", path_as_qemu_string("serial.file", path)?),
            );
        }
        QemuSerialConfig::Null => push_pair(argv, "-serial", "null"),
    }
    Ok(())
}

fn push_control_channel(
    argv: &mut Vec<OsString>,
    control_channel: &QemuControlChannelConfig,
) -> Result<(), QemuArgvError> {
    validate_qemu_suboption("control_channel.pipe_name", &control_channel.pipe_name)?;
    validate_qemu_suboption("control_channel.port_name", &control_channel.port_name)?;

    push_pair(
        argv,
        "-device",
        format!("virtio-serial-pci,id={CONTROL_BUS_ID}"),
    );
    push_pair(
        argv,
        "-chardev",
        format!(
            "pipe,id={CONTROL_CHARDEV_ID},path={}",
            control_channel.pipe_name
        ),
    );
    push_pair(
        argv,
        "-device",
        format!(
            "virtserialport,chardev={CONTROL_CHARDEV_ID},name={}",
            control_channel.port_name
        ),
    );
    Ok(())
}

fn push_qmp(argv: &mut Vec<OsString>, qmp: &QemuQmpEndpoint) -> Result<(), QemuArgvError> {
    match qmp {
        QemuQmpEndpoint::NamedPipe { pipe_name } => {
            validate_qemu_suboption("qmp.pipe_name", pipe_name)?;
            push_pair(argv, "-qmp", format!("pipe:{pipe_name},server=on,wait=off"));
        }
    }
    Ok(())
}

fn path_as_qemu_option_value(field: &'static str, path: &Path) -> Result<String, QemuArgvError> {
    let value = path_as_qemu_string(field, path)?;
    Ok(value.replace(',', ",,"))
}

fn path_as_qemu_string(field: &'static str, path: &Path) -> Result<String, QemuArgvError> {
    path.as_os_str()
        .to_str()
        .map(str::to_string)
        .ok_or(QemuArgvError::InvalidQemuOptionValue {
            field,
            reason: "must be valid UTF-8 for QEMU option syntax",
        })
}

fn validate_qemu_suboption(field: &'static str, value: &str) -> Result<(), QemuArgvError> {
    if value.is_empty() {
        return Err(QemuArgvError::MissingRequiredInput { field });
    }
    if value.contains(',') {
        return Err(QemuArgvError::InvalidQemuOptionValue {
            field,
            reason: "must not contain comma separators",
        });
    }
    Ok(())
}
