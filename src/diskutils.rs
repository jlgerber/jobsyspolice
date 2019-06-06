use std::{env, fmt::Debug, fs, os::unix::fs::PermissionsExt, path::Path, path::PathBuf };
use crate::{JSPError, User, constants, get_default_user, Node, NodeType};
use log;
use lazy_static::lazy_static;
// wah!!! I don't like these deps
use nix::{ unistd::{chown, Uid}, NixPath };
use users::{ get_user_by_name };
use shellfn::shell;
use std::error::Error;

lazy_static! {
    static ref ROOT_PATH: PathBuf = Path::new("/").to_path_buf();
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
    };
    Ok(get_user_by_name(&owner).ok_or( JSPError::InvalidUserName(owner.to_string()))?.uid())
}

/// Set permissions on a path.
pub fn set_path_perms<P: AsRef<Path> + Debug>(path: P, perms: &str) -> Result<(), JSPError> {
    let path = path.as_ref();
    if path == ROOT_PATH.as_path() { return Ok(()); }

    log::info!("set_path_perms(path: {:?}, perms: {})", &path, perms);

    let perms_u32 = u32::from_str_radix(&perms, 8)?;
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(perms_u32);
    //log::debug!("fs::set_permissions(path: {:?}, perms: {:?}", path, &perms);
    fs::set_permissions(&path, perms)?;
    Ok(())
}

pub fn set_path_owner_id<P>(path: P, id: u32) -> Result<(), JSPError>
    where P: NixPath + Debug
{
        log::info!("set_path_owner_id(path: {:?}, id: {})", &path, id);

        return Ok(chown(&path, Some(Uid::from_raw(id)), None )?);
}

// Sets the owner for a path
pub fn chown_owner(path: PathBuf, owner: &User, node: &Node) -> Result<(), JSPError> {
    log::info!("chown_owner(path: {:?}, owner: {:?}, node: {:?})", path, &owner, node);

    let dirname = path.as_path()
                    .file_name()
                    .ok_or(JSPError::FilenameFromPathFailed(path.clone()))?
                    .to_str()
                    .ok_or(JSPError::PathBufConvertToStrFailed(path.clone()))?;

    let owner_id = get_uid_for_owner(owner, node, dirname)?;
    let euid = Uid::effective().as_raw();
    log::debug!("effective id of process: {}", euid);
    if owner_id != euid {
        log::info!("owner id ({}) and euid ({}) differ. suproccessing via _chown {:?} to {}", owner_id, euid, &path, owner);
        let _result = _chown(path.to_str().unwrap(), owner_id)?;
    } else {
        set_path_owner_id(path, owner_id)?
    }
    Ok(())
}

#[shell]
fn _chown(dir: &str, owner: u32 ) -> Result<String, Box<Error>> { r#"
    sudo chown $OWNER $DIR
"# }