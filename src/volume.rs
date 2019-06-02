use std::path::Path;
//use crate::{JGraph, is_valid, JSPError };

pub trait MakeVolume {
    type ErrType;
    type OkType;

    fn mk(&self, path: impl AsRef<Path> ) -> Result<Self::OkType, Self::ErrType>;
    fn default_owner(&self) -> &str;
    fn default_perms(&self) -> u32;
}

pub mod local {
    use crate::{ JGraph, is_valid, JSPError, EntryType };
    use super::{ MakeVolume, Path};
    use std::{ path::PathBuf, fs };

    #[derive(Debug)]
    pub struct VolumeMaker<'a> {
        graph: &'a JGraph,
        owner: String,
        perms: u32
    }

    impl<'a> VolumeMaker<'a> {
        /// new up a VolumerMaker
        pub fn new(graph: &'a JGraph, owner: String, perms: u32) -> Self {
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
            // step 1: validate
            let path = path.as_ref();
            let nodepath = is_valid(path, self.graph)?;
            // step 2: iterate: create, assign ownership, set perms
            let mut create_path = PathBuf::new();
            for (idx, item) in path.iter().enumerate() {
                let node = &nodepath[idx];
                create_path.push(item);
                match node.entry_type() {
                    &EntryType::Directory => {
                        if !create_path.exists() {
                            fs::create_dir(&create_path)?;
                        }
                        // perms
                        if let Some(perms) = node.perms() {
                            // err now we need to convert to perms. not happy
                            // with the current rep. will need to readdress this
                        }
                    }
                    &EntryType::Untracked => {
                        if !create_path.exists() {
                            fs::create_dir(&create_path)?;
                        }
                        // todo set default owner & perms
                    }
                    &EntryType::Volume => {
                        if !create_path.exists() {
                            fs::create_dir(&create_path)?;
                        }
                        // todo set default owner & perms
                    }
                    EntryType::Root => panic!("entry type root not supported"),
                }
            }
            // step 3: profit
            Ok(())
        }

        fn default_owner(&self) -> &str {
            &self.owner
        }

        fn default_perms(&self) -> u32 {
            self.perms
        }
    }
}