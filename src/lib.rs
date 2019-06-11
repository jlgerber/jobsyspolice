#[macro_use]
pub mod macros {
    macro_rules! s {
        ($val: expr) => {
            $val.to_string();
        }
    }
}

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
pub use graph::{is_valid, JGraph};

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
pub use find::{find, find_path};

pub mod searchterm;
pub use searchterm::{Search, SearchTerm};