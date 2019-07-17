use crate::{JSPError, User, constants, get_default_user, Node, NodeType, JGraph};
use log;
use lazy_static::lazy_static;
use nix::{ unistd::{chown, Uid, Gid }};
use std::{
    env,
    fs,
    os::unix::{
        fs::{MetadataExt},
    },
    fs::File,
    io::{/*BufWriter,*/ Write},
    path::{Component, Path, PathBuf},
};

use users::{ get_user_by_name, get_group_by_name };

lazy_static! {
    static ref ROOT_PATH: PathBuf = Path::new("/").to_path_buf();
    static ref DOUBLEDOT: PathBuf = PathBuf::from("..");
    static ref DOT: PathBuf = PathBuf::from(".");
}

/// Given a path, owner, and permissions, create the supplied directory with
/// appropriate metadata
///
/// # Parameters
/// * `path` A reference to a std::path::Path
/// * `owner_id` A u32 representing the path owner's id
/// * `group_id` A u43 representing the path oner's group id
/// * `perms` A u32 representing file permissions
///
/// # Returns
/// A Unit or JSPError
pub fn create_dir(path: &Path, owner_id: u32, group_id: u32, perms: u32) -> Result<(), JSPError> {
    log::info!("create_dir(path:{:?}, owner_id:{}, perms:{}) called", path, owner_id, perms);
    fs::create_dir(path)?;
    // todo set perms
    chown(
        path,
        Some(Uid::from_raw(owner_id)),
        Some(Gid::from_raw( group_id )),
    )?;
    Ok(())
}

/// Retrieve the gid gor the supplied group name. 
/// 
/// # Parameters
/// 
/// * `group` A `&str` representing the name of the group
/// 
/// # Returns
/// * A `u32` representing the gid of the group if successful
/// * A JSPError::NoGroupForName if unsuccessful
pub fn get_uid_for_group(group: &str) -> Result<u32, JSPError> {
    let os_group: &std::ffi::OsStr = group.as_ref();
    match get_group_by_name(os_group) {
        Some(g) => Ok(g.gid()),
        None => Err(JSPError::NoGroupForName(group.to_string())),
    }
}

/// Retrieve the current process owner's gid
pub fn get_current_gid() -> u32 {
    Gid::current().as_raw()
}

/// Retrieve the current process owner's effective gid
pub fn get_effective_gid() -> u32 {
    Gid::effective().as_raw()
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

    let owner = match owner {
        User::Named(name) => {
            name.clone()
        }
        User::Me => {
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
        User::Captured(key) => {
            log::info!("get_uid_for_owner(...) User::Captured({})", key);

            if let NodeType::RegEx{ pattern, ..} = node.identity() {
                let caps = pattern.captures(dir).ok_or(JSPError::MissingOwnerInRegex)?;
                let owner = caps.name(key).ok_or(JSPError::MissingOwnerInRegex)?.as_str();
                log::debug!("get_uid_for_owner(...) returning owner {}", owner);
                owner.to_string()
            } else {
                log::error!("get_uid_for_owner(...) Missing owner in regex");
                return Err(JSPError::MissingOwnerInRegex);
            }
        }
        User::Uid(uid) => {
            return Ok(*uid);
        }
    };
    Ok(get_user_by_name(&owner).ok_or_else(|| JSPError::InvalidUserName(owner.to_string()))?.uid())
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
/*
/// Given a relative pathbuf, convert it to an absolute pathbuf.
pub fn convert_relative_pathbuf_to_absolute(path: PathBuf) -> Result<PathBuf, JSPError> {
    let mut curdir = std::env::current_dir()?;
    if path.starts_with(".") || !path.starts_with("/") {
        let doit = path.starts_with("..");
        
        curdir = curdir.join(path);
        if doit {
            curdir = curdir.iter().fold(PathBuf::new(), |mut acc, x| {
                if x == DOUBLEDOT.as_os_str() ||  x == DOT.as_os_str() {
                    acc.pop();
                } else {
                    acc.push(x);
                }
                    acc
                } 
            );
        }
        log::info!("curdir {:?}", curdir);
        return Ok(curdir);
    }
    Ok(path)
}
*/

/// Converts from relative path to absolute, removing relative path nonsense
pub fn convert_relative_pathbuf_to_absolute<I>(path: I) -> Result<PathBuf, JSPError> 
where
    I: AsRef<Path> + std::fmt::Debug 
{
    let path = path.as_ref();
    let mut return_pathbuf = PathBuf::new();
    for (cnt,x) in path.components().enumerate() {
        
        if x == Component::CurDir {
            if cnt == 0 {
                return_pathbuf.push(std::env::current_dir()?);
            }
            //pass
        } else if x == Component::ParentDir {
            if cnt == 0 {
               let mut cdir = std::env::current_dir()?;
               cdir.pop();
               return_pathbuf = cdir;
            } else {
                return_pathbuf.pop();
            }

        } else {
            return_pathbuf.push(x)
        }
    }
    Ok(return_pathbuf)
}
/* Nothing should be relying on this code. Given jspt, we no longer serialize the 
graph to json. 

/// Write the template out to disk.
pub fn write_template(output: &mut PathBuf, graph: &JGraph) {

    // if we are writing out the template, we use the internal definition
    //let graph = graph::testdata::build_graph();

    // test to see if buffer is a directory. if it is apply the standard name
    if output.is_dir() {
        output.push(constants::JSP_NAME);
    }
    let j = serde_json::to_string_pretty(&graph).unwrap();
    let file = match File::create(output) {
        Ok(out) => {
            log::debug!("attempting to write to {:?}", out);
            out},
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    let mut f = BufWriter::new(file);
    f.write_all(j.as_bytes()).expect("Unable to write data");
}
*/

/// Given an output path and a reference to a JGraph, write 
/// the graph out to disk.
pub fn write_template_as_dotfile(output: &PathBuf, graph: &JGraph) {
    let mut file = match File::create(output) {
        Ok(out) => {
            log::debug!("attempting to write to {:?}", out);
            out},
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    match file.write_all(
        format!(
            "{:#?}"
            ,petgraph::dot::Dot::with_config(
                &graph,
                &[petgraph::dot::Config::EdgeNoLabel]
            )
        ).as_bytes()
    ) {
        Err(e) => {
            eprintln!("{}",e);
            std::process::exit(1);
        }
        Ok(_) => ()
    };
}

/// Set the stickybit on the directory from the provided path.
pub fn set_stickybit(path: &Path) -> Result<(), JSPError> {
    log::debug!("diskutils::set_sticktbit({:?})", path);
    use std::os::unix::fs::PermissionsExt;
    // get filehandle
    let stickybit = 0o1000;
    let fh = std::fs::File::open(path)?;
    let meta = fh.metadata()?;
    let mut mode = meta.mode();
    // if stickybit not set
    if mode & stickybit == 0 {
        mode |= stickybit;
        let mut permissions = meta.permissions();
        permissions.set_mode(mode);
        fh.set_permissions(permissions)?;
    }
    Ok(())
}