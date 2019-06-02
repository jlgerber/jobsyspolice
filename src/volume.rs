use std::path::Path;
use crate::{JGraph, JSPError };

pub trait MakeVolume {
    type ErrType;
    type OkType;

    fn mk(path: impl AsRef<Path> ) -> Result<Self::OkType, Self::ErrType>;
    fn default_owner(&self) -> &str;
    fn default_perms(&self) -> u32;
}

pub mod local {
    use super::{JGraph, JSPError, MakeVolume, Path};

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

        fn mk(path: impl AsRef<Path> ) -> Result<Self::OkType, Self::ErrType> {
            // step 1: validate
            // step 2: iterate: create, assign ownership, set perms
            // step 3: profit

            Err(JSPError::Placeholder)
        }

        fn default_owner(&self) -> &str {
            &self.owner
        }

        fn default_perms(&self) -> u32 {
            self.perms
        }
    }
}