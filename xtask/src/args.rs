use anyhow::{anyhow, Result};
use lsb_platform::{host_platform, platform_by_id, PlatformSpec};

use crate::context::env_value;

pub fn resolve_platform(args: &[String]) -> Result<&'static PlatformSpec> {
    let env_platform = env_value("LSB_PLATFORM");
    let platform_id = select_platform_id(
        flag_value(args, "--platform"),
        env_platform.as_deref(),
        host_platform().map(|platform| platform.id),
    )?;
    platform_by_id(platform_id).ok_or_else(|| anyhow!("unknown platform id: {platform_id}"))
}

fn select_platform_id<'a>(
    flag_platform: Option<&'a str>,
    env_platform: Option<&'a str>,
    host_platform: Option<&'a str>,
) -> Result<&'a str> {
    flag_platform
        .or(env_platform)
        .or(host_platform)
        .ok_or_else(|| anyhow!("unable to infer platform; pass --platform or set LSB_PLATFORM"))
}

pub fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| pair[1].as_str())
}

pub fn required_flag_value<'a>(args: &'a [String], flag: &str) -> Result<&'a str> {
    flag_value(args, flag).ok_or_else(|| anyhow!("missing required flag: {flag}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_value_reads_adjacent_argument() {
        let args = vec![
            "--platform".to_string(),
            "macos-aarch64".to_string(),
            "--format".to_string(),
            "env".to_string(),
        ];

        assert_eq!(flag_value(&args, "--platform"), Some("macos-aarch64"));
        assert_eq!(flag_value(&args, "--format"), Some("env"));
        assert_eq!(flag_value(&args, "--missing"), None);
    }

    #[test]
    fn select_platform_id_prefers_explicit_flag() {
        let platform_id = select_platform_id(
            Some("macos-aarch64"),
            Some("macos-x86_64"),
            Some("linux-x86_64"),
        )
        .expect("flag should win");

        assert_eq!(platform_id, "macos-aarch64");
    }

    #[test]
    fn select_platform_id_falls_back_to_environment() {
        let platform_id = select_platform_id(None, Some("macos-x86_64"), Some("linux-x86_64"))
            .expect("env should win");

        assert_eq!(platform_id, "macos-x86_64");
    }

    #[test]
    fn select_platform_id_falls_back_to_host_platform() {
        let platform_id =
            select_platform_id(None, None, Some("macos-x86_64")).expect("host should win");

        assert_eq!(platform_id, "macos-x86_64");
    }

    #[test]
    fn select_platform_id_errors_when_no_source_is_available() {
        let err = select_platform_id(None, None, None).expect_err("missing source should fail");

        assert_eq!(
            err.to_string(),
            "unable to infer platform; pass --platform or set LSB_PLATFORM"
        );
    }
}
