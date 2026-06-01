use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::backend::FlatFileBackend;
use crate::cas::ChunkIndex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinnedRootfs {
    pub hash: String,
    pub rootfs_path: String,
    pub index_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseVersionRecord {
    pub version: String,
    pub hash: String,
    pub rootfs_path: String,
    pub index_path: String,
}

pub fn pin_rootfs(data_dir: &str, rootfs_path: &str) -> Result<PinnedRootfs> {
    let hash = hash_file(rootfs_path)?;
    let cas_dir = Path::new(data_dir).join("cas");
    let bases_dir = cas_dir.join("bases");
    let indexes_dir = cas_dir.join("indexes");
    fs::create_dir_all(&bases_dir)
        .with_context(|| format!("failed to create base rootfs dir: {}", bases_dir.display()))?;
    fs::create_dir_all(&indexes_dir).with_context(|| {
        format!(
            "failed to create CAS indexes dir: {}",
            indexes_dir.display()
        )
    })?;

    let pinned_rootfs = bases_dir.join(format!("{hash}.ext4"));
    if !pinned_rootfs.exists() {
        copy_rootfs_atomic(rootfs_path, &pinned_rootfs)?;
    }

    let index_path = indexes_dir.join(format!("base-{hash}.idx"));
    if !index_path.exists() {
        let fallback = FlatFileBackend::open(path_str(&pinned_rootfs)?).with_context(|| {
            format!("failed to open pinned rootfs: {}", pinned_rootfs.display())
        })?;
        let mut index = ChunkIndex::new(fallback.size());
        index.fallback_path = Some(path_str(&pinned_rootfs)?.to_string());
        index.save(path_str(&index_path)?)?;
    }

    Ok(PinnedRootfs {
        hash,
        rootfs_path: path_str(&pinned_rootfs)?.to_string(),
        index_path: path_str(&index_path)?.to_string(),
    })
}

pub fn pin_base_version(
    data_dir: &str,
    rootfs_path: &str,
    version: &str,
    force: bool,
) -> Result<BaseVersionRecord> {
    validate_base_version(version)?;
    let pinned = pin_rootfs(data_dir, rootfs_path)?;
    let record_path = base_version_record_path(data_dir, version)?;

    if record_path.exists() {
        let existing = read_base_version_record_path(&record_path)?;
        if existing.hash == pinned.hash {
            return Ok(existing);
        }
        if !force {
            bail!(
                "base version '{}' is already pinned to rootfs hash {}, refusing to repin to {}",
                version,
                existing.hash,
                pinned.hash
            );
        }
    }

    let record = BaseVersionRecord {
        version: version.to_string(),
        hash: pinned.hash,
        rootfs_path: pinned.rootfs_path,
        index_path: pinned.index_path,
    };
    write_base_version_record_path(&record_path, &record)?;
    Ok(record)
}

pub fn resolve_base_version(data_dir: &str, version: &str) -> Result<BaseVersionRecord> {
    validate_base_version(version)?;
    let record_path = base_version_record_path(data_dir, version)?;
    let record = read_base_version_record_path(&record_path)?;

    ensure_existing_file(&record.rootfs_path, "pinned base rootfs")?;
    ensure_existing_file(&record.index_path, "pinned base index")?;

    Ok(record)
}

pub fn read_data_dir_version(data_dir: &str) -> Result<String> {
    let version_path = Path::new(data_dir).join("VERSION");
    let version = fs::read_to_string(&version_path)
        .with_context(|| format!("failed to read VERSION file: {}", version_path.display()))?;
    let version = version.trim().to_string();
    if version.is_empty() {
        bail!("VERSION file is empty: {}", version_path.display());
    }
    Ok(version)
}

fn hash_file(path: &str) -> Result<String> {
    let mut file =
        fs::File::open(path).with_context(|| format!("failed to open rootfs: {path}"))?;
    let mut hasher = blake3::Hasher::new();
    let mut buf = vec![0u8; 1024 * 1024];
    loop {
        let n = file
            .read(&mut buf)
            .with_context(|| format!("failed to read rootfs: {path}"))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

fn copy_rootfs_atomic(src: &str, dst: &Path) -> Result<()> {
    let tmp = temp_path_for(dst);
    let _ = fs::remove_file(&tmp);
    lsb_platform::copy_file_cow(src, path_str(&tmp)?)
        .with_context(|| format!("failed to pin rootfs {} -> {}", src, dst.display()))?;
    fs::rename(&tmp, dst)
        .with_context(|| format!("failed to install pinned rootfs: {}", dst.display()))?;
    Ok(())
}

fn temp_path_for(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("rootfs.ext4");
    path.with_file_name(format!(".{file_name}.{}.tmp", std::process::id()))
}

fn base_version_record_path(data_dir: &str, version: &str) -> Result<PathBuf> {
    validate_base_version(version)?;
    let versions_dir = Path::new(data_dir).join("cas").join("base-versions");
    fs::create_dir_all(&versions_dir).with_context(|| {
        format!(
            "failed to create base version registry dir: {}",
            versions_dir.display()
        )
    })?;
    Ok(versions_dir.join(format!("{version}.json")))
}

fn read_base_version_record_path(path: &Path) -> Result<BaseVersionRecord> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read base version record: {}", path.display()))?;
    serde_json::from_str(&contents)
        .with_context(|| format!("failed to parse base version record: {}", path.display()))
}

fn write_base_version_record_path(path: &Path, record: &BaseVersionRecord) -> Result<()> {
    let tmp = temp_path_for(path);
    let contents = serde_json::to_string_pretty(record)?;
    fs::write(&tmp, format!("{contents}\n"))
        .with_context(|| format!("failed to write base version record: {}", tmp.display()))?;
    fs::rename(&tmp, path)
        .with_context(|| format!("failed to install base version record: {}", path.display()))?;
    Ok(())
}

fn validate_base_version(version: &str) -> Result<()> {
    if version.is_empty()
        || version.contains('/')
        || version.contains('\\')
        || version.contains('\0')
        || version.contains("..")
    {
        bail!("invalid base version: '{}'", version);
    }
    Ok(())
}

fn ensure_existing_file(path: &str, label: &str) -> Result<()> {
    if !Path::new(path).is_file() {
        bail!("{} is missing: {}", label, path);
    }
    Ok(())
}

fn path_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| anyhow::anyhow!("path is not valid UTF-8: {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pins_and_resolves_base_version() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let data_dir = tmp.path().join("data");
        let rootfs = tmp.path().join("rootfs.ext4");
        fs::write(&rootfs, b"base-v1").expect("write rootfs");

        let data_dir = data_dir.to_string_lossy().into_owned();
        let rootfs = rootfs.to_string_lossy().into_owned();

        let record = pin_base_version(&data_dir, &rootfs, "1.2.3", false).expect("pin");
        assert_eq!(record.version, "1.2.3");
        assert!(Path::new(&record.rootfs_path).is_file());
        assert!(Path::new(&record.index_path).is_file());

        let resolved = resolve_base_version(&data_dir, "1.2.3").expect("resolve");
        assert_eq!(resolved, record);
    }

    #[test]
    fn refuses_to_repin_existing_version_without_force() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let data_dir = tmp.path().join("data");
        let rootfs = tmp.path().join("rootfs.ext4");
        fs::write(&rootfs, b"base-v1").expect("write rootfs");

        let data_dir = data_dir.to_string_lossy().into_owned();
        let rootfs = rootfs.to_string_lossy().into_owned();

        let first = pin_base_version(&data_dir, &rootfs, "1.2.3", false).expect("pin");
        fs::write(&rootfs, b"base-v2").expect("rewrite rootfs");

        let err = pin_base_version(&data_dir, &rootfs, "1.2.3", false).unwrap_err();
        assert!(err.to_string().contains("already pinned"));

        let forced = pin_base_version(&data_dir, &rootfs, "1.2.3", true).expect("force repin");
        assert_ne!(forced.hash, first.hash);
        assert_eq!(
            resolve_base_version(&data_dir, "1.2.3").expect("resolve"),
            forced
        );
    }
}
