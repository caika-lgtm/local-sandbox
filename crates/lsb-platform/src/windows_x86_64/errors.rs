use anyhow::anyhow;

pub(crate) fn unsupported(capability: &str, milestone: &str) -> anyhow::Error {
    anyhow!(
        "Windows support is in progress: {capability} is not implemented yet ({milestone}); current Windows runtime support includes direct QEMU boot, guest-ready, non-interactive exec, copy-in/copy-out, mount import/export MVP, loopback port forwarding, policy-mediated proxy networking, and the M13 qcow2 checkpoint/store MVP"
    )
}
