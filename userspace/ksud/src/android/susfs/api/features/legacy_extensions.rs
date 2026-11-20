use anyhow::{Result, ensure};

use super::show;
use crate::android::susfs::{
    api::{
        magic::{
            CMD_SUSFS_ADD_SUS_MEMFD_LEGACY, CMD_SUSFS_ADD_SUS_MEMFD_V24_EXT,
            CMD_SUSFS_ADD_TRY_UMOUNT, ERR_CMD_NOT_SUPPORTED, SUSFS_MAX_LEN_PATHNAME,
        },
        susfsctl::{parse_err, susfsctl},
    },
    utils::str_to_c_array,
};

#[repr(C)]
struct SusfsTryUmount {
    target_pathname: [u8; SUSFS_MAX_LEN_PATHNAME],
    mnt_mode: i32,
    err: i32,
}

#[repr(C)]
struct SusfsSusMemfd {
    target_pathname: [u8; SUSFS_MAX_LEN_PATHNAME],
    err: i32,
}

fn validate_field(value: &str) -> Result<()> {
    ensure!(!value.is_empty(), "Name/path must not be empty");
    ensure!(!value.as_bytes().contains(&0), "Name/path contains a NUL byte");
    ensure!(value.len() < SUSFS_MAX_LEN_PATHNAME, "Name/path is too long");
    Ok(())
}

pub fn add_try_umount(path: &str, mode: i32) -> Result<()> {
    validate_field(path)?;
    ensure!(path.starts_with('/'), "Mount path must be absolute");
    ensure!((0..=1).contains(&mode), "Unmount mode must be 0 or 1");
    let features = show::enabled_features()?;
    ensure!(features.lines().any(|f| f == "CONFIG_KSU_SUSFS_TRY_UMOUNT"), "Kernel lacks SUSFS try_umount");
    let mut info = SusfsTryUmount {
        target_pathname: [0; SUSFS_MAX_LEN_PATHNAME],
        mnt_mode: mode,
        err: ERR_CMD_NOT_SUPPORTED,
    };
    str_to_c_array(path, &mut info.target_pathname);
    susfsctl(CMD_SUSFS_ADD_TRY_UMOUNT, &mut info);
    parse_err(CMD_SUSFS_ADD_TRY_UMOUNT, info.err)
}

pub fn add_sus_memfd(name: &str) -> Result<()> {
    ensure!(!name.is_empty() && name != "memfd:", "Memfd name must not be empty");
    let name = if name.starts_with("memfd:") { name.to_owned() } else { format!("memfd:{name}") };
    validate_field(&name)?;
    let features = show::enabled_features()?;
    ensure!(features.lines().any(|f| f == "CONFIG_KSU_SUSFS_SUS_MEMFD"), "Kernel lacks SUSFS memfd blocking");
    // Never guess from the version or send the memfd struct to 2.4's priority filter.
    let cmd = if features.lines().any(|f| f == "SUSFS_KAZESU_LEGACY_EXTENSIONS") {
        CMD_SUSFS_ADD_SUS_MEMFD_V24_EXT
    } else {
        ensure!(!show::version()?.starts_with("v2.4"), "SUSFS 2.4 kernel lacks the Kazesu memfd extension ABI");
        CMD_SUSFS_ADD_SUS_MEMFD_LEGACY
    };
    let mut info = SusfsSusMemfd {
        target_pathname: [0; SUSFS_MAX_LEN_PATHNAME],
        err: ERR_CMD_NOT_SUPPORTED,
    };
    str_to_c_array(&name, &mut info.target_pathname);
    susfsctl(cmd, &mut info);
    parse_err(cmd, info.err)
}
