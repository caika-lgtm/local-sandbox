use anyhow::{bail, Context, Result};

#[derive(Debug, Clone, Copy)]
pub struct StoragePrepareOptions<'a> {
    pub data_dir: &'a str,
    pub checkpoints_dir: &'a str,
    pub rootfs_path: &'a str,
    pub from: Option<&'a str>,
    pub base_version: Option<&'a str>,
    pub custom_rootfs: bool,
    pub direct: bool,
}

#[derive(Debug, Clone)]
pub struct PreparedStorage {
    pub direct_source_rootfs: String,
    pub nbd_source: Option<NbdSource>,
    pub logical_size: u64,
}

impl PreparedStorage {
    pub fn active_source_rootfs(&self) -> &str {
        self.nbd_source
            .as_ref()
            .map(|source| source.rootfs_path.as_str())
            .unwrap_or(&self.direct_source_rootfs)
    }

    pub fn cas_index(&self) -> Option<&str> {
        self.nbd_source
            .as_ref()
            .map(|source| source.index_path.as_str())
    }

    pub fn is_nbd(&self) -> bool {
        self.nbd_source.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct NbdSource {
    pub rootfs_path: String,
    pub index_path: String,
}

pub fn prepare_storage(options: StoragePrepareOptions<'_>) -> Result<PreparedStorage> {
    if options.from.is_some() && options.base_version.is_some() {
        bail!("checkpoint and base_version cannot be used together");
    }

    if options.direct {
        let direct_source_rootfs = resolve_direct_source(options)?;
        let logical_size = file_size(&direct_source_rootfs)?;
        return Ok(PreparedStorage {
            direct_source_rootfs,
            nbd_source: None,
            logical_size,
        });
    }

    let source = resolve_nbd_source(options)?;
    Ok(PreparedStorage {
        direct_source_rootfs: source.direct_source_rootfs,
        nbd_source: Some(NbdSource {
            rootfs_path: source.nbd_rootfs_path,
            index_path: source.nbd_index_path,
        }),
        logical_size: source.logical_size,
    })
}

fn resolve_direct_source(options: StoragePrepareOptions<'_>) -> Result<String> {
    match options.from {
        Some(name) => {
            lsb_vm::validate_checkpoint_name(name).map_err(|e| anyhow::anyhow!(e))?;
            let idx_path = format!("{}/{}.idx", options.checkpoints_dir, name);
            let ext4_path = format!("{}/{}.ext4", options.checkpoints_dir, name);
            if std::path::Path::new(&idx_path).exists() {
                bail!(
                    "Checkpoint '{}' is a CAS index and requires NBD storage; unset LSB_STORAGE=direct",
                    name
                );
            }
            if std::path::Path::new(&ext4_path).exists() {
                Ok(ext4_path)
            } else {
                bail!("Checkpoint '{}' not found", name);
            }
        }
        None => {
            ensure_rootfs_exists(options.rootfs_path)?;
            if options.base_version.is_some() {
                Ok(resolve_base_rootfs(
                    options.data_dir,
                    options.rootfs_path,
                    options.base_version,
                )?
                .rootfs_path)
            } else {
                Ok(options.rootfs_path.to_string())
            }
        }
    }
}

struct ResolvedNbdSource {
    direct_source_rootfs: String,
    nbd_rootfs_path: String,
    nbd_index_path: String,
    logical_size: u64,
}

fn resolve_nbd_source(options: StoragePrepareOptions<'_>) -> Result<ResolvedNbdSource> {
    match options.from {
        Some(name) => {
            lsb_vm::validate_checkpoint_name(name).map_err(|e| anyhow::anyhow!(e))?;
            let idx_path = format!("{}/{}.idx", options.checkpoints_dir, name);
            let ext4_path = format!("{}/{}.ext4", options.checkpoints_dir, name);
            if std::path::Path::new(&idx_path).exists() {
                let logical_size = index_size(&idx_path)?;
                Ok(ResolvedNbdSource {
                    direct_source_rootfs: options.rootfs_path.to_string(),
                    nbd_rootfs_path: options.rootfs_path.to_string(),
                    nbd_index_path: idx_path,
                    logical_size,
                })
            } else if std::path::Path::new(&ext4_path).exists() {
                let pinned = lsb_store::pin_rootfs(options.data_dir, &ext4_path)?;
                let logical_size = index_size(&pinned.index_path)?;
                Ok(ResolvedNbdSource {
                    direct_source_rootfs: ext4_path,
                    nbd_rootfs_path: pinned.rootfs_path,
                    nbd_index_path: pinned.index_path,
                    logical_size,
                })
            } else {
                bail!("Checkpoint '{}' not found", name);
            }
        }
        None => {
            ensure_rootfs_exists(options.rootfs_path)?;
            let pinned = if let Some(version) = options.base_version {
                resolve_base_rootfs(options.data_dir, options.rootfs_path, Some(version))?
            } else if options.custom_rootfs {
                lsb_store::pin_rootfs(options.data_dir, options.rootfs_path)?
            } else {
                resolve_base_rootfs(options.data_dir, options.rootfs_path, None)?
            };
            let logical_size = index_size(&pinned.index_path)?;
            Ok(ResolvedNbdSource {
                direct_source_rootfs: options.rootfs_path.to_string(),
                nbd_rootfs_path: pinned.rootfs_path,
                nbd_index_path: pinned.index_path,
                logical_size,
            })
        }
    }
}

fn resolve_base_rootfs(
    data_dir: &str,
    rootfs_path: &str,
    base_version: Option<&str>,
) -> Result<lsb_store::PinnedRootfs> {
    let version = match base_version {
        Some(version) => version.to_string(),
        None => lsb_store::read_data_dir_version(data_dir)?,
    };

    match lsb_store::resolve_base_version(data_dir, &version) {
        Ok(record) => Ok(lsb_store::PinnedRootfs {
            hash: record.hash,
            rootfs_path: record.rootfs_path,
            index_path: record.index_path,
        }),
        Err(err) => {
            if base_version.is_some() {
                let current = lsb_store::read_data_dir_version(data_dir).ok();
                if current.as_deref() != Some(version.as_str()) {
                    return Err(err).with_context(|| {
                        format!(
                            "base version '{}' is not pinned; omit baseVersion to use the current initialized VERSION, or prepare this version with `lsb init --version {}` / initSandbox({{ version: '{}' }}) first",
                            version, version, version
                        )
                    });
                }
            }
            let record = lsb_store::pin_base_version(data_dir, rootfs_path, &version, false)?;
            Ok(lsb_store::PinnedRootfs {
                hash: record.hash,
                rootfs_path: record.rootfs_path,
                index_path: record.index_path,
            })
        }
    }
}

fn ensure_rootfs_exists(rootfs_path: &str) -> Result<()> {
    if !std::path::Path::new(rootfs_path).exists() {
        bail!(
            "Rootfs not found at {}. Run `lsb init` to download.",
            rootfs_path
        );
    }
    Ok(())
}

fn file_size(path: &str) -> Result<u64> {
    Ok(std::fs::metadata(path)
        .with_context(|| format!("failed to stat rootfs: {}", path))?
        .len())
}

fn index_size(path: &str) -> Result<u64> {
    Ok(lsb_store::ChunkIndex::load(path)
        .with_context(|| format!("failed to load CAS index: {}", path))?
        .disk_size())
}
