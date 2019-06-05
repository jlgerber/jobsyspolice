use std::{env, fmt::Debug, fs, os::unix::fs::PermissionsExt, path::Path, path::PathBuf };
use crate::{JSPError, User, constants, get_default_user};
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

/// retrieve the uid from a user
pub fn get_uid(user: &str) -> Result<u32, JSPError> {
    Ok(get_user_by_name(&user).ok_or( JSPError::InvalidUserName(user.to_string()))?.uid())
}

/// Set permissions on a path.
pub fn set_path_perms<P: AsRef<Path> + Debug>(path: P, perms: &str) -> Result<(), JSPError> {
    let path = path.as_ref();
    if path == ROOT_PATH.as_path() { return Ok(()); }

    //log::debug!("set_path_perms(path: {:?},perms: {})", path, perms);
    let perms_u32 = u32::from_str_radix(&perms, 8)?;
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(perms_u32);
    //log::debug!("fs::set_permissions(path: {:?}, perms: {:?}", path, &perms);
    fs::set_permissions(&path, perms)?;
    Ok(())
}

pub fn set_path_owner<P>(path: P, owner: &User ) -> Result<(), JSPError>
    where P: NixPath + Debug
{
    match &owner {
        &User::Named(name) => {
            //log::debug!("setting path {:?} owner to named {}", &path, name);
            // attempt to get name
            let uid = get_user_by_name(&name).ok_or( JSPError::Placeholder)?;
            //log::debug!("uid is {:?}", uid);
            return Ok(chown(&path, Some(Uid::from_raw(uid.uid())), None )?);
        }
        &User::Me => {
            let user = match env::var(constants::USER_ENV_VAR) {
                Ok(u) => u,
                Err(_) => {
                    log::warn!("unable to look up current user from environment!");
                    get_default_user()
                }
            };
            // get uid
            //log::debug!("setting path {:?} owner to Me {}",&path, &user);
            if user == "root" {panic!("Attempt to change ownership to root not allowed");}
            let uid = get_user_by_name(&user).ok_or( JSPError::InvalidUserName(user.to_string()))?;
            //log::debug!("uid of me {:?}", uid);
            return Ok(chown(&path, Some(Uid::from_raw(uid.uid())), None )?);
        }
        &User::Captured(_name) => {
            unimplemented!();
        }
    }
}

// Sets the owner for a path
pub fn chown_owner(path: PathBuf, owner: &User) -> Result<(), JSPError> {
    let owner_id = get_uid(owner.to_string().as_str())?;
    let euid = Uid::effective().as_raw();
    log::debug!("owner: {:?}, id: {}, euid: {}", owner, owner_id, euid);
    if owner_id != euid {
        log::info!("owner id ({}) and euid ({}) differ. Chowning {:?} to {}", owner_id, euid, &path, owner);
        let _result = _chown(path.to_str().unwrap(), owner_id)?;
    } else {
        ;
        set_path_owner(path, owner)?;
    }
    Ok(())
}

#[shell]
fn _chown(dir: &str, owner: u32 ) -> Result<String, Box<Error>> { r#"
    sudo chown $OWNER $DIR
"# }