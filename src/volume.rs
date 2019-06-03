use std::path::{Path};

/// MakeVolume trait intended to be implemented for a given storage setup.
/// For instance, Netapp has a specific call to make a volume that involves
/// a RESTful call (with ontap 6.4). This is unique, obviously, to Netapp.
pub trait MakeVolume {
    type ErrType;
    type OkType;
    /// Make the directory or volume.
    fn mk(&self, path: impl AsRef<Path> ) -> Result<Self::OkType, Self::ErrType>;

    /// Retrieve the default owner if none is supplied
    fn default_owner(&self) -> &str;

    /// Retrieve the default permissions as a &str if non is supplied
    fn default_perms(&self) -> &str;
}

pub mod local {
    use crate::{ diskutils, JGraph, is_valid, JSPError, EntryType, User, constants };
    use super::{ MakeVolume, Path };
    use std::{ path::PathBuf, fs };

    /// local::VolumeMaker is, as it sounds, an implementation of MakeVolume that
    /// works for local filesystems.
    #[derive(Debug)]
    pub struct VolumeMaker<'a> {
        graph: &'a JGraph,
        owner: String,
        perms: String
    }

    impl<'a> VolumeMaker<'a> {
        /// new up a VolumerMaker
        pub fn new(graph: &'a JGraph, owner: String, perms: String) -> Self {
            Self {
                graph,
                owner,
                perms,
            }
        }
    }

    impl<'a> MakeVolume for VolumeMaker<'a> {
        type OkType = ();
        type ErrType = JSPError;

        fn mk(&self, path: impl AsRef<Path> ) -> Result<Self::OkType, Self::ErrType> {
            log::debug!("local::MakeVolume.mk called");
            let path = path.as_ref();
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
                        if !create_path.exists() {
                            fs::create_dir(&create_path)?;
                            // perms
                            if let Some(perms) = node.perms() {
                                gperms = perms
                            }
                            let tmp = node.owner().clone();
                            log::trace!("node: {:?} type:{:?}", &node, &node.entry_type());
                            owner = tmp.unwrap_or(owner);
                            diskutils::set_path_owner(create_path.clone(), &owner)?;
                            diskutils::set_path_perms(&create_path, &gperms)?;
                        }

                    }

                    &EntryType::Volume => {
                        if !create_path.exists() {
                            fs::create_dir(&create_path)?;
                            if let Some(perms) = node.perms() {
                                gperms = perms
                            }

                            let tmp = node.owner().clone();
                            //log::debug!("node: {:?} type:{:?}", &node, &node.entry_type());
                            owner = tmp.unwrap_or(owner);
                            diskutils::set_path_owner(create_path.clone(), &owner)?;
                            diskutils::set_path_perms(&create_path, &gperms)?;
                        }

                    }

                    &EntryType::Untracked => {
                        if !create_path.exists() {
                            fs::create_dir(&create_path)?;
                            log::debug!("Untracked type");
                            diskutils::set_path_owner(create_path.clone(), &owner)?;
                            diskutils::set_path_perms(&create_path, &gperms)?;
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
}