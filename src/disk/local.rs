
use crate::{ diskutils, JGraph, is_valid, JSPError, EntryType, User, constants };
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

// requires coreutils be installed. mac only right now. sudo port install coreutils


impl<'a> Disk for DiskService<'a> {

    fn mk(&self, path: &Path ) -> Result<(), JSPError> {
        log::debug!("local::Disk.mk called");
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
                    log::info!("EntryType::Directory");

                    let tmp = node.owner().clone();
                    log::trace!("node: {:?} type:{:?}", &node, &node.entry_type());
                    owner = tmp.unwrap_or(owner);

                    if !create_path.exists() {
                        fs::create_dir(&create_path)?;
                        // perms
                        if let Some(perms) = node.perms() {
                            gperms = perms
                        }

                        diskutils::set_path_perms(&create_path, &gperms)?;
                        diskutils::chown_owner(create_path.clone(), &owner)?;
                    }
                }

                &EntryType::Volume => {
                    log::info!("EntryType::Volume");

                    let tmp = node.owner().clone();
                    log::trace!("node: {:?} type:{:?}", &node, &node.entry_type());
                    owner = tmp.unwrap_or(owner);

                    if !create_path.exists() {
                        fs::create_dir(&create_path)?;
                        if let Some(perms) = node.perms() {
                            gperms = perms
                        }
                        diskutils::set_path_perms(&create_path, &gperms)?;
                        diskutils::chown_owner(create_path.clone(), &owner)?;
                    }

                }

                &EntryType::Untracked => {
                    log::info!("EntryType::Untracked");
                    if !create_path.exists() {
                        fs::create_dir(&create_path)?;
                        log::debug!("Untracked type");
                        diskutils::set_path_perms(&create_path, &gperms)?;
                        diskutils::chown_owner(create_path.clone(), &owner)?;
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
