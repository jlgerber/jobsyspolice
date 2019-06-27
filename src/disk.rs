use std::path::{Path};
use crate::{ JGraph, JSPError };

/// Disk trait intended to be implemented for a given storage setup.
/// For instance, Netapp has a specific call to make a volume that involves
/// a RESTful call (with ontap 6.4). This is unique, obviously, to Netapp.
pub trait Disk {
    /// Make the directory or voluem.
    fn mk(&self, path: &Path, ignore_volume: bool ) -> Result<(), JSPError>;

    /// Retrieve the default owner if none is supplied
    fn default_owner(&self) -> &str;

    /// Retrieve the default permissions as a &str if non is supplied
    fn default_perms(&self) -> &str;
}

pub mod local;

/// The type of the DiskService. There should be one variant per implementation
#[derive(Debug, PartialEq, Eq)]
pub enum DiskType {
    Local,
}

/// Retrieve the disk service given a DiskType
pub fn get_disk_service<'a>(disk_type: DiskType, graph: &'a JGraph) ->  Box<dyn Disk + 'a> {
    match disk_type {
        DiskType::Local => Box::new(local::DiskService::new(
            &graph,
            String::from("jobsys"),
            String::from("751")
        ))
    }
}