use std::{fmt::Debug, fs, os::unix::fs::PermissionsExt, path::Path, path::PathBuf };
use crate::JSPError;
use log;
use lazy_static::lazy_static;

lazy_static! {
    static ref ROOT_PATH: PathBuf = Path::new("/").to_path_buf();
}

/// Set permissions on a path.
pub fn set_path_perms<P: AsRef<Path> + Debug>(path: P, perms: &str) -> Result<(), JSPError> {
    let path = path.as_ref();
    if path == ROOT_PATH.as_path() { return Ok(()); }

    log::debug!("set_path_perms(path: {:?},perms: {})", path, perms);
    let perms_u32 = u32::from_str_radix(&perms, 8)?;
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(perms_u32);
    log::debug!("fs::set_permissions(path: {:?}, perms: {:?}", path, &perms);
    fs::set_permissions(&path, perms)?;
    Ok(())
}