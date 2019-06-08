use crate::{JSPError, User, constants, get_default_user, Node, NodeType};
use log;
use lazy_static::lazy_static;
use nix::{ unistd::{chown, Uid, Gid }};
use std::{
    env,
    fs,
    os::unix::{
        fs::{MetadataExt},
    },
    path::Path,
    path::PathBuf,
};
use users::{ get_user_by_name };

lazy_static! {
    static ref ROOT_PATH: PathBuf = Path::new("/").to_path_buf();
}

/// Given a path, owner, and permissions, create the supplied directory with
/// appropriate metadata
///
/// # Parameters
/// * `path` A reference to a std::path::Path
/// * `owner_id` A u32 representing the path owner's id
/// * `perms` A u32 representing file permissions
///
/// # Returns
/// A Unit or JSPError
pub fn create_dir(path: &Path, owner_id: u32, perms: u32) -> Result<(), JSPError> {
    log::info!("create_dir() called");
    fs::create_dir(path)?;
    chown(
        path,
        Some(Uid::from_raw(owner_id)),
        Some(Gid::from_raw( perms )),
    )?;
    Ok(())
}

/// Retrieve the user id for the supplied owner. If the owner is of type User::Captured,
/// this method attempts to extract the user name from the path using the regex supplied
/// by the node parameter, which is expected to have a named regex capture whose name corresponds
/// with the String owned by User::Capture(name)
///
/// # Parameters
///
/// * `owner` - reference to User
/// * `node` - reference to Node which the request relates to. This is used if the owner is of type User::Captured
/// * `dir` - &str of the directory this relates to. This is used if the owner is of type User::Captured.
///
/// # Results
///
/// u32 uid if successful, or a JSPError, otherwise
pub fn get_uid_for_owner(owner: &User, node: &Node, dir: &str) -> Result<u32, JSPError> {

    log::info!("get_uid_for_owner(owner: {:?}, node: {:?}, dir: {})", owner, node, dir);

    let owner = match &owner {
        &User::Named(name) => {
            name.clone()
        }
        &User::Me => {
            let user = match env::var(constants::USER_ENV_VAR) {
                Ok(u) => u,
                Err(_) => {
                    log::warn!("get_uid_for_owner(...) unable to look up current user from environment!");
                    get_default_user()
                }
            };
            if user == "root" {panic!("get_uid_for_owner(...) Attempt to change ownership to root not allowed");}
            user
        }
        &User::Captured(key) => {
            log::info!("get_uid_for_owner(...) User::Captured({})", key);

            if let NodeType::RegEx{name: _, pattern, exclude: _} = node.identity() {
                let caps = pattern.captures(dir).ok_or(JSPError::MissingOwnerInRegex)?;
                let owner = caps.name(key).ok_or(JSPError::MissingOwnerInRegex)?.as_str();
                owner.to_string()
            } else {
                return Err(JSPError::MissingOwnerInRegex);
            }
        }
        &User::Uid(uid) => {
            return Ok(*uid);
        }
    };
    Ok(get_user_by_name(&owner).ok_or( JSPError::InvalidUserName(owner.to_string()))?.uid())
}

/// given a path, retrieve the owner of the path
///
/// # Parameters
/// * `path` - &std::path::Path
///
/// # Returns
///
/// A User::Uid or a JSPError
pub fn get_owner_for_path(path: &Path) -> Result<User, JSPError> {
    let metadata = std::fs::metadata(path)?;
    Ok(User::Uid(metadata.uid()))
}
