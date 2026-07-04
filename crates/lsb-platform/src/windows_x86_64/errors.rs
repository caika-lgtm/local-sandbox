use anyhow::anyhow;

pub(crate) fn unsupported(capability: &str, milestone: &str) -> anyhow::Error {
    anyhow!(
        "Windows support is in progress: {capability} is not implemented yet ({milestone}); current Windows runtime support is limited to direct QEMU boot through the M07 guest-ready handshake"
    )
}
