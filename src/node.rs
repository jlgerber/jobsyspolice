use crate::EntryType;
use crate::NodeType;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    identity: NodeType,
    entry_type: EntryType,
}

impl Node {
    pub fn new(identity: NodeType, entry_type: EntryType) -> Self {
        Self { identity, entry_type }
    }
    pub fn new_root() -> Self {
        Self {
            identity: NodeType::Root,
            entry_type: EntryType::Root,
        }
    }
}
impl PartialEq<std::ffi::OsStr> for Node {
    fn eq(&self, other: &std::ffi::OsStr) -> bool {
        match &self.identity {
            NodeType::Root => false,
            NodeType::Name(strval) => strval.as_str() == other,
            NodeType::Regexp { name: _, pattern } => pattern.is_match(other.to_str().unwrap()),
        }
    }
}

impl std::default::Default for Node {
    fn default() -> Node {
        Node::new(NodeType::Name("NONE".to_string()), EntryType::Directory)
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Node, ()> {
        Ok(Node::new(NodeType::Name(s.to_string()), EntryType::Directory))
    }
}
