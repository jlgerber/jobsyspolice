
use crate::{ diskutils, JGraph, validate_path, JSPError, EntryType, User, constants };
use super::{ Disk, Path };
use std::{ path::PathBuf };
use log;

#[cfg(target_os="macos")]
use std::os::macos::fs::MetadataExt;

#[cfg(target_os="linux")]
use std::os::linux::fs::MetadataExt;

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
    fn mk(&self, path: &Path, sticky:bool, ignore_volume: bool) -> Result<(), JSPError> {
        log::info!("local::Disk.mk(path: {:?}, ignore_volume: {})", path, ignore_volume);

        let nodepath = validate_path(path, self.graph)?;
        // we need to stash information when we reach the last node
        // that is in the template for a given path. So we store the
        // length of the nodepath, which we use to match against the
        // loop count later, when we are looping over the supplied path.
        let last_managed_node = nodepath.len() - 1; 
        log::trace!("last managed node {}", last_managed_node);
        let mut perms_str: &str = &self.perms;
        let mut perms_u32 = u32::from_str_radix(&perms_str,8).expect("couldnt convert perms_str to perms");
        let mut gid = diskutils::get_uid_for_group(constants::DEFAULT_GROUP)?;
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
                perms_str = perms;
                perms_u32 = u32::from_str_radix(&perms_str,8).expect("couldnt convert perms_str to perms");
            }
            
            // retrieve the group info from metadata. If it does not exist, get it
            // from the path. We may want to move this down into the match 
            // statement below to differentiate between untracked nodes and the rest of
            // the nodes. 
            match node.metadata().group() {
                Some(ref grp) => {
                    // get the group id
                    gid = diskutils::get_uid_for_group(grp)?;
                    log::debug!("local::DiskService.mk(...) retrieved gid {} for group {}", gid, grp);
                },
                None => {
                    if create_path.exists() {
                        gid = create_path.metadata()?.st_gid();
                    }
                },
            }           
        
            match *node.entry_type() {
                EntryType::Directory | EntryType::Volume => {
                    log::debug!("local::DiskService.mk(...) EntryType::Directory or Volume");

                    // If we only want to update the gid when we are 
                    // in a tracked node, we would do it here instead of outside
                    // get the group id
                    // if create_path.exists() {
                    //     gid = create_path.metadata()?.st_gid();
                    // }
                    // lets get group
                    
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
                        //let gid = diskutils::get_current_gid();
                        diskutils::create_dir(&create_path, uid, gid, perms_u32)?

                    } 
                    // now cache uid.
                    if idx == last_managed_node {
                        log::trace!("local::DiskService.mk(...) last_managed_node");
                        // stash the uid from the recently created path as a User::Uid()
                        // this will be used by Untracked to assign ownership.
                        owner = diskutils::get_owner_for_path(&create_path)?;
                        log::trace!("local::DiskService.mk(...) last_managed_node owner : {:?} for path {:?}",
                                    owner, &create_path);
                        // now we set the stickybit
                        if sticky {
                            diskutils::set_stickybit(create_path.as_path())?;
                        }
                    }
                }

                EntryType::Untracked => {
                    log::trace!("local::DiskService.mk(...) EntryType::Untracked");
                    if !create_path.exists() {
                        log::debug!("local::DiskService.mk(...) {:?} does not exist. attempting to create", &create_path);
                        if let User::Uid(id) = owner {
                            //let gid = diskutils::get_current_gid();
                            diskutils::create_dir(&create_path, id, gid, perms_u32)?;
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

