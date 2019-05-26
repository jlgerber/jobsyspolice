use crate::EntryType;
use crate::Valid;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    name: Valid,
    entry_type: EntryType,
}

impl Node {
    pub fn new(name: Valid, entry_type: EntryType) -> Self {
        Self { name, entry_type }
    }
    pub fn new_root() -> Self {
        Self {
            name: Valid::Root,
            entry_type: EntryType::Root,
        }
    }
}
impl PartialEq<std::ffi::OsStr> for Node {
    fn eq(&self, other: &std::ffi::OsStr) -> bool {
        match &self.name {
            Valid::Root => false,
            Valid::Name(strval) => strval.as_str() == other,
            Valid::Regexp { name: _, pattern } => pattern.is_match(other.to_str().unwrap()),
        }
    }
}

impl std::default::Default for Node {
    fn default() -> Node {
        Node::new(Valid::Name("NONE".to_string()), EntryType::Directory)
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Node, ()> {
        Ok(Node::new(Valid::Name(s.to_string()), EntryType::Directory))
    }
}
