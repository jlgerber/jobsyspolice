//! Define a trait for interfacing with disk, and implement said trait for local
//! and GX systems. 
use std::path::{Path};
use crate::{ JGraph, JSPError };

/// Disk trait intended to be implemented for a given storage setup.
/// For instance, Netapp has a specific call to make a volume that involves
/// a RESTful call (with ontap 6.4). This is unique, obviously, to Netapp.
pub trait Disk {

    /// Make the directory or volume.
    /// 
    /// # Parameters
    /// * `path`: Reference to Path that we wish to make
    /// * `sticky`: Bool, if true, we add teh sticky to the group
    /// * `ignore_volume`: Bool, if true, we treat volumes like normal directories
    /// 
    /// # Returns
    /// * Ok wrapped unit, if successful
    /// * JSPError if unsuccessful
    fn mk(&self, path: &Path, sticky: bool, ignore_volume: bool ) -> Result<(), JSPError>;

    /// Retrieve the default owner if none is supplied. 
    fn default_owner(&self) -> &str;

    /// Retrieve the default permissions as a &str if non is supplied
    fn default_perms(&self) -> &str;
}

pub mod local;
pub mod gx;

/// The type of disk system. This dictates the strategy for file/directory
/// creation, as well as volume creation.
#[derive(Debug, PartialEq, Eq)]
pub enum DiskType {
    /// Local DiskType does not differentiate between volumes and directories.
    /// Furthermore, it assumes that rootsquash is not active, as it relies on
    /// executng as a privilaged user in order to change ownership of directories. 
    Local,
    /// GX assumes that rootsquash is active, but that the abilty to give away
    /// ownership for files/directories that one owns has been enabled in the 
    /// OnTap preferences. DiskType::Gx also relies upon the jspmk command being
    /// setuid enabled, but its strategy involves setting the process owner to the 
    /// owner of the parent directory for each file/directory it makes.
    Gx,
}

/// Retrieve an instance of a DiksService given a DiskType
pub fn get_disk_service<'a>(disk_type: &'a DiskType, graph: &'a JGraph) ->  Box<dyn Disk + 'a> {
    match *disk_type {
        DiskType::Local => Box::new(local::DiskService::new(
            &graph,
            String::from("jobsys"),
            String::from("751")
        )),
        DiskType::Gx => Box::new(gx::DiskService::new(
            &graph,
            String::from("jobsys"),
            String::from("751")
        ))
    }
}