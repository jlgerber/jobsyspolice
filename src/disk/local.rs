
use crate::{ diskutils, JGraph, is_valid, JSPError, EntryType, Node, User, constants };
use super::{ Disk, Path };
use std::{ path::PathBuf, fs };
use log;

/// local::DiskService is, as it sounds, an implementation of Disk that
/// works for local filesystems.
#[derive(Debug)]
pub struct DiskService<'a> {
    graph: &'a JGraph,
    owner: String,
    perms: String
}

impl<'a> DiskService<'a> {
    /// new up a VolumerMaker
    pub fn new(graph: &'a JGraph, owner: String, perms: String) -> Self {
        Self {
            graph,
            owner,
            perms,
        }
    }
}

fn _create_path(create_path: &Path, gperms: &str, owner: &User, node: &Node ) -> Result<(), JSPError> {
    log::info!("_create_path(\n\tcreate_path: {:?},\n\tgperms: {},\n\towner: {},\n\tnode: {}\n)", create_path, gperms, owner, node);
    match diskutils::set_path_perms(create_path, gperms) {
        Ok(_) => (),
        Err(e) => {
            log::error!("{}", e.to_string());
            log::error!("updating path permissions failed. Attempting to roll back creation of '{:?}'",
                        create_path);

            fs::remove_dir(&create_path)?;
            return Err(e);
        }
    };
    log::trace!("calling diksutils::chown_owner");
    match diskutils::chown_owner(PathBuf::from(create_path), owner, node){
        Ok(_) => (),
        Err(e) => {
            log::error!("{}", e);
            log::error!("Changing ownership of directory failed. Attempting to roll back creation of '{:?}'",
                        create_path);

            fs::remove_dir(&create_path)?;
            return Err(e);
        },
    };
    Ok(())
}
// requires coreutils be installed. mac only right now. sudo port install coreutils
impl<'a> Disk for DiskService<'a> {

    fn mk(&self, path: &Path ) -> Result<(), JSPError> {
        log::info!("local::Disk.mk(path: {:?})", path);
        //let path = path.as_ref();
        let nodepath = is_valid(path, self.graph)?;
        let mut gperms: &str = &self.perms;

        let mut owner = User::from(constants::DEFAULT_USER);

        // step 2: iterate: create, assign ownership, set perms
        let mut create_path = PathBuf::new();
        for (idx, item) in path.iter().enumerate() {
            log::trace!("path iter pass {}", idx);
            create_path.push(item);
            if idx == 0 {continue;}
            // idx 0 is / so we have to subtract one
            let node = &nodepath[idx - 1];
            match node.entry_type() {
                &EntryType::Directory => {
                    log::debug!("local::DiskService EntryType::Directory");

                    let tmp = node.owner().clone();
                    log::trace!("node: {} type:{:?}", &node, &node.entry_type());
                    owner = tmp.unwrap_or(owner);

                    if !create_path.exists() {
                        fs::create_dir(&create_path)?;
                        // perms
                        if let Some(perms) = node.perms() {
                            gperms = perms
                        }
                        _create_path(&create_path, &gperms, &owner, &node )?;
                    }
                }

                &EntryType::Volume => {
                    log::debug!("local::DiskService EntryType::Volume");

                    let tmp = node.owner().clone();
                    log::trace!("node: {} type:{:?}", &node, &node.entry_type());
                    owner = tmp.unwrap_or(owner);

                    if !create_path.exists() {
                        fs::create_dir(&create_path)?;
                        if let Some(perms) = node.perms() {
                            gperms = perms
                        }
                        _create_path(&create_path, &gperms, &owner, &node )?;
                    }

                }

                &EntryType::Untracked => {
                    log::debug!("local::DiskService EntryType::Untracked");
                    if !create_path.exists() {
                        fs::create_dir(&create_path)?;
                        log::debug!("Untracked type");
                        _create_path(&create_path, &gperms, &owner, &node )?;
                    }
                }

                &EntryType::Root => panic!("entry type root not supported"),
            }
        }
        // step 3: profit
        Ok(())
    }

    fn default_owner(&self) -> &str {
        &self.owner
    }

    fn default_perms(&self) -> &str {
        &self.perms
    }
}
