#[macro_use]
pub mod macros {
    macro_rules! s {
        ($val: expr) => {
            $val.to_string();
        }
    }
}

pub mod regexp;
pub use regexp::*;

pub mod nodetype;
pub use nodetype::{NodeType, ValidType};

pub mod entrytype;
pub use entrytype::EntryType;

pub mod node;
pub use node::Node;

#[macro_use]
pub mod jstmacro {
    macro_rules! jstnode {
        ($name:expr) => (Node::from_str($name).unwrap());
        ($name:expr, $regex:expr) => (Node::new_regexp($name, $regex, None));
        ($name:expr, $regex:expr, $exclude:expr) => (Node::new_regexp_adv($name, $regex, $exclude, None));
    }
}

pub mod graph;
pub use graph::{is_valid, JGraph};

pub mod returnvalue;
pub use returnvalue::{ ReturnValue, NIndex };