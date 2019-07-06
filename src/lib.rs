#[macro_use]
pub mod macros {
    macro_rules! s {
        ($val: expr) => {
            $val.to_string();
        }
    }
}

pub mod navalias;
pub use navalias::Navalias;

pub mod errors;
pub use errors::JSPError;

pub mod regexp;
pub use regexp::*;

pub mod nodetype;
pub use nodetype::{NodeType, ValidType};

pub mod entrytype;
pub use entrytype::EntryType;

pub mod node;
pub use node::{Node };

pub mod nodepath;
pub use nodepath::NodePath;

pub mod graph;
pub use graph::{validate_path, JGraph, get_graph, get_graph_from_fn};

pub mod returnvalue;
pub use returnvalue::{ ReturnValue, NIndex };

pub mod diskutils;
//pub use diskutils::DiskUtils;

pub mod disk;
pub use disk::{Disk, local, get_disk_service, DiskType};

pub mod user;
pub use user::{User, get_default_user};

pub mod constants;

pub mod find;
pub use find::{find, find_path, find_path_from_terms, find_rel};

pub mod searchterm;
pub use searchterm::{Search, SearchTerm};

pub mod metadata;
pub use metadata::{Metadata,MetadataTerm};

pub mod cachedenvvars;
pub use cachedenvvars::CachedEnvVars;

pub mod shell;
pub use shell::{bash, tcsh, ShellEnvManager, SupportedShell};

pub mod cli;
pub use cli::{gen_terms_from_strings};

pub mod jspt;

pub mod report;

pub mod validpath;
pub use validpath::ValidPath;

pub mod minimatch;
pub use minimatch::parse_show_from_arg;


#[macro_export]
macro_rules!  jspnode {
    // jspnode("foo")
    ($name:expr) => (
        Node::new(
            NodeType::Simple(String::from($name)),
            EntryType::Directory,
            None,
            None,
            None,
            false,
            None,
        )
    );
    // jspnode!("foo", "owner" => "bob")
    ($name:expr, $($key:expr => $val:expr),+) => ({
        let mut n = Node::new(
            NodeType::Simple(String::from($name)),
            EntryType::Directory,
            None,
            None,
            None,
            false,
            None,
        );
        $(
            match $key {
                "owner" => {n.metadata_mut().set_owner(Some(crate::User::from($val)));}
                "perms" | "permissions" => {
                    let conv = $val.parse::<u32>();
                    if conv.is_ok(){
                        n.metadata_mut().set_perms(Some($val.to_owned()));
                    }
                }
                "varname" => {n.metadata_mut().set_varname(Some(String::from($val)));}
                "autocreate" => {n.metadata_mut().set_autocreate($val.parse().unwrap_or_else(|_v| false));}
                "navalias" => {
                    if let Some(idx) = $val.find("=") {
                        let (key, value) = $val.split_at(idx);
                        let value = value.trim_start_matches('=');
                        n.metadata_mut().set_navalias(Some(crate::navalias::Navalias::Complex{name:key.to_owned(), value: value.to_owned()} ));
                    } else {
                        n.metadata_mut().set_navalias(Some(crate::navalias::Navalias::Simple($val.to_owned())));
                    }
                }
                _ => ()
            }
        )+
        n
    });
    ($name:expr, $regex:expr) => (
        Node::new(
        NodeType::RegEx {
            name: $name.into(),
            pattern: Regexp::new($regex).unwrap(),
            exclude: None,
        },
        EntryType::Directory,
        None,
        None, 
        None,
        false,
        None,
        ));
    ($name:expr, $regex:expr, $($key:expr => $val:expr),+) => ({
        let mut n = Node::new(
        NodeType::RegEx {
            name: $name.into(),
            pattern: Regexp::new($regex).unwrap(),
            exclude: None,
        },
        EntryType::Directory,
        None,
        None,
        None,
        false,
        None,
        );
        $(
            match $key {
                "owner" => {n.metadata_mut().set_owner(Some(crate::User::from($val)));}
                "perms" | "permissions" => {
                    let conv = $val.parse::<u32>();
                    if conv.is_ok(){
                        n.metadata_mut().set_perms(Some($val.to_owned()));
                    }
                }
                "varname" => {n.metadata_mut().set_varname(Some(String::from($val)));}
                "autocreate" => {n.metadata_mut().set_autocreate($val.parse().unwrap_or_else(|_v| false));}
                "navalias" => {
                    if let Some(idx) = $val.find("=") {
                        let (key, value) = $val.split_at(idx);
                        let value = value.trim_start_matches('=');
                        n.metadata_mut().set_navalias(Some(crate::navalias::Navalias::Complex{name:key.to_owned(), value: value.to_owned()} ));
                    } else {
                        n.metadata_mut().set_navalias(Some(crate::navalias::Navalias::Simple($val.to_owned())));
                    }
                }
                _ => ()
            }
        )+
        n
    });
    ($name:expr, $regex:expr, $exclude:expr) => (
        Node::new(
            NodeType::RegEx {
                name: $name.into(),
                pattern: Regexp::new($regex).unwrap(),
                exclude: Some(Regexp::new($exclude).unwrap()),
            },
            EntryType::Directory,
            None,
            None,
            None,
            false,
            None,
        )
    );
    ($name:expr, $regex:expr, $exclude:expr, $($key:expr => $val:expr),+) => ({
        let mut n = Node::new(
            NodeType::RegEx {
                name: $name.into(),
                pattern: Regexp::new($regex).unwrap(),
                exclude: Some(Regexp::new($exclude).unwrap()),
            },
            EntryType::Directory,
            None,
            None,
            None, 
            false,
            None,
        );
        $(
            match $key {
                "owner" => {n.metadata_mut().set_owner(Some(crate::User::from($val)));}
                "perms" | "permissions" => {
                    let conv = $val.parse::<u32>();
                    if conv.is_ok(){
                        n.metadata_mut().set_perms(Some($val.to_owned()));
                    }
                }
                "varname" => {n.metadata_mut().set_varname(Some(String::from($val)));}
                "autocreate" => {n.metadata_mut().set_autocreate($val.parse().unwrap_or_else(|_v| false));}
                "navalias" => {
                    if let Some(idx) = $val.find("=") {
                        let (key, value) = $val.split_at(idx);
                        let value = value.trim_start_matches('=');
                        n.metadata_mut().set_navalias(Some(crate::navalias::Navalias::Complex{name:key.to_owned(), value: value.to_owned()} ));
                    } else {
                        n.metadata_mut().set_navalias(Some(crate::navalias::Navalias::Simple($val.to_owned())));
                    }
                }
                _ => ()
            }
        )+
        n
    });
}

