
use crate::Valid;
use crate::NodeType;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
  name: Valid,
  node_type: NodeType,
}

impl Node {
    pub fn new(name:Valid, node_type: NodeType) -> Self {
        Self {
            name,
            node_type,
        }
    }

}
impl PartialEq<std::ffi::OsStr> for Node {
fn eq(&self, other: &std::ffi::OsStr) -> bool {
        match &self.name {
            Valid::Name(strval) => {strval.as_str() == other},
            Valid::Regexp{name:_, pattern} => { pattern.is_match(other.to_str().unwrap()) }
        }
    }
}

impl std::default::Default for Node {
    fn default() -> Node {
        Node::new(Valid::Name("NONE".to_string()), NodeType::Directory)
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Node, ()> {
        Ok(Node::new( Valid::Name(s.to_string()), NodeType::Directory))
    }
}
