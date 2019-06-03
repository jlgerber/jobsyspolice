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

pub mod volume;
pub use volume::{MakeVolume, local};

pub mod user;
pub use user::User;

pub mod constants;
