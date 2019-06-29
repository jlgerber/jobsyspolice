
use crate::{ diskutils, JGraph, validate_path, JSPError, EntryType, User, constants };
use super::{ Disk, Path };
use std::{ path::PathBuf };
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

// TODO: requires coreutils be installed. mac only right now. sudo port install coreutils
impl<'a> Disk for DiskService<'a> {

    fn mk(&self, path: &Path, ignore_volume: bool) -> Result<(), JSPError> {
        log::info!("local::Disk.mk(path: {:?}, ignore_volume: {})", path, ignore_volume);

        let nodepath = validate_path(path, self.graph)?;
        // we need to stash information when we reach the last node
        // that is in the template for a given path. So we store the
        // length of the nodepath, which we use to match against the
        // loop count later, when we are looping over the supplied path.
        let last_managed_node = nodepath.len() - 1; 
        log::trace!("last managed node {}", last_managed_node);
        let mut gperms: &str = &self.perms;
        let mut uperms = u32::from_str_radix(&gperms,8).expect("couldnt convert gperms to perms");
        let mut owner = User::from(constants::DEFAULT_USER);

        // step 2: iterate: create, assign ownership, set perms
        let mut create_path = PathBuf::new();

        for (idx, item) in path.iter().enumerate() {
            log::debug!("path iter pass {}, item: {:?}", idx, &item);
            create_path.push(item);
            // seems like I can now remove the next two lines
            //if idx == 0 {continue;}
            // idx 0 is / so we have to subtract one
            let node = &nodepath[idx];

            // update permissions if they have changed
            if let Some(perms) = node.metadata().perms() {
                gperms = perms;
                uperms = u32::from_str_radix(&gperms,8).expect("couldnt convert gperms to perms");
            }

            match *node.entry_type() {
                EntryType::Directory | EntryType::Volume => {
                    log::debug!("local::DiskService.mk(...) EntryType::Directory or Volume");

                    // we need the owner to look up the uid
                    let tmp_owner = node.metadata().owner().clone();
                    log::trace!("node: {} type:{:?}", &node, &node.entry_type());
                    owner = tmp_owner.unwrap_or(owner);

                    log::trace!("local::DiskService.mk(...) retrieving uid via diskutils::get_uid_for_owner");
                    let uid = diskutils::get_uid_for_owner(
                        &owner,
                        &node,
                        item.to_str().expect("unable to convert osstr to str")
                    )?;

                    log::trace!("local::DiskService.mk(...) testing if create_path.exists {:?} {}", &create_path, create_path.exists()) ;
                    if !create_path.exists() {
                        log::trace!("local::DiskService.mk(...) calling diksutils::create_dir()");
                        diskutils::create_dir(&create_path, uid, uperms)?

                    } 
                    // now cache uid.
                    if idx == last_managed_node {
                        log::trace!("local::DiskService.mk(...) last_managed_node");
                        // stash the uid from the recently created path as a User::Uid()
                        // this will be used by Untracked to assign ownership.
                        owner = diskutils::get_owner_for_path(&create_path)?;
                        log::trace!("local::DiskService.mk(...) last_managed_node owner : {:?} for path {:?}",
                                    owner, &create_path);
                    }
                }

                EntryType::Untracked => {
                    log::trace!("local::DiskService.mk(...) EntryType::Untracked");
                    if !create_path.exists() {
                        log::debug!("local::DiskService.mk(...) {:?} does not exist. attempting to create", &create_path);
                        if let User::Uid(id) = owner {
                            diskutils::create_dir(&create_path, id, uperms)?;
                        } else {
                            log::error!( "local::DiskService.mk(...) unexpected. Unable to get Uid from owner {:?} in EntryType::Untracked",
                                    owner);
                            return Err(JSPError::UidRetrievalError(
                                format!(
                                    "local::DiskService.mk(...) unexpected. Unable to get Uid from owner {:?} in EntryType::Untracked",
                                    owner
                                )
                            ));
                        }
                    } else {
                        log::debug!("local::DiskService.mk(...) path {:?} exists. skipping mkdir", create_path);
                    }
                }

                EntryType::Root => {
                    log::trace!("local::DiskService.mk(...) EntryType::Root");
                },
            }
        }
        Ok(())
    }

    fn default_owner(&self) -> &str {
        &self.owner
    }

    fn default_perms(&self) -> &str {
        &self.perms
    }
}

