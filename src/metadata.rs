use crate::{User};
use serde::{Serialize, Deserialize};
use std::cmp::PartialEq;

pub type PermsType = String;

/// Used to search for Specific Metadata in the Graph
#[derive(Debug, Clone, Copy)]
pub enum MetadataTerm {
    Owner,
    Perms,
    Varname,
    Autocreate,
}

impl PartialEq<MetadataTerm> for MetadataTerm {
    fn eq(&self, other: &MetadataTerm) -> bool {
        self == other
    }
}

// Metata == MetadataTerm comparisons
impl PartialEq<Metadata> for MetadataTerm {
    fn eq(&self, other: &Metadata) -> bool {
        match self {
            &MetadataTerm::Owner => {
                other.has_owner()
            }
            &MetadataTerm::Perms => other.has_perms(),
            &MetadataTerm::Varname => other.has_varname(),
            &MetadataTerm::Autocreate => other.autocreate(),
        }
    }
}

impl Eq for MetadataTerm {}

/// Metadata structure
//TODO: Move volume to Metadata
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    perms: Option<PermsType>, //todo: change rep
    #[serde(skip_serializing_if = "Option::is_none")]
    varname: Option<String>,
    autocreate: bool
}

impl std::default::Default for Metadata {
    fn default() -> Metadata {
        Self {  
            owner: None,
            perms: None,
            varname: None,
            autocreate: false
        }
    }
}
impl Metadata {

    /// New up a Metadata instance
    pub fn new() -> Self {
        Metadata::default()
    }

    /// Alternate constructor
    pub fn from_components(owner: Option<User>, perms: Option<PermsType>, varname: Option<String>, autocreate: bool) -> Self 
    {
        Self {
            owner, 
            perms,
            varname,
            autocreate
        }
    }

    /// Check to see if the metadata instance has owner
    pub fn has_owner(&self) -> bool {
        self.owner.is_some()
    }

    /// Set the owner for Metadata
    pub fn set_owner(&mut self, user: Option<User>) -> &Self {
        self.owner = user;
        self
    }

    /// Get the owner
    pub fn owner(&self) -> &Option<User> {
        &self.owner
    }

    /// Get the owner
    pub fn owner_ref(&self) -> Option<&User> {
        match self.owner {
            Some(ref owner) => Some(&owner),
            _ => None,
        }
    }

    /// Get a mutable owner
    pub fn owner_mut(&mut self) -> &mut Option<User> {
        &mut self.owner
    }

    /// Check to see of the Metadata instance has perms
    pub fn has_perms(&self) -> bool {
        self.perms.is_some()
    }

    /// Set the perms for Metadata
    pub fn set_perms(&mut self, perms: Option<PermsType>) -> &mut Self  {
        self.perms = perms;
        self
    }

    /// Get the perms
    pub fn perms(&self) -> &Option<PermsType> {
       &self.perms
    }

    /// Get the perms
    pub fn perms_ref(&self) -> Option<&PermsType> {
       match self.perms {
           Some(ref perms) => Some(&perms),
           _ => None,
       }
    }

    /// Get a mutable perms
    pub fn perms_mut(&mut self) -> &mut Option<PermsType> {
        &mut self.perms
    }

    pub fn has_varname(&self) -> bool {
        self.varname().is_some()
    }

    /// Set the varname for Metadata
    pub fn set_varname(&mut self, varname: Option<String>) -> &mut Self {
        log::info!("Metadata.set_varname({:?})", varname);
        self.varname = varname;
        self
    }

    /// Get the varname
    pub fn varname(&self) -> &Option<String> {
       &self.varname
    }

    /// Get the varname
    pub fn varname_ref(&self) -> Option<&str> {
       match self.varname {
           Some(ref name) => Some(&name),
           _ => None
       }
    }

    /// Get a mutable varname
    pub fn varname_mut(&mut self) -> &mut Option<String> {
        &mut self.varname
    }

    /// Get autocreate
    pub fn autocreate(&self) -> bool {
        self.autocreate
    }

    /// set the autocreate state
    pub fn set_autocreate(&mut self, autocreate: bool) -> &mut Self {
        self.autocreate = autocreate;
        self
    }

    /// given a mutable reference to self, create a 
    /// concrete copy
    pub fn reify(&mut self) -> Self {
        self.clone()
    }
}