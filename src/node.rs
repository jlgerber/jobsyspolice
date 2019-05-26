use crate::EntryType;
use crate::NodeType;
use std::str::FromStr;

/// The Node caries information about a specific
/// directory or file within the candidate jobsystem
/// graph. This information is used to validate
/// candidate paths in order to determine wheither or not
/// they are valid.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    identity: NodeType,
    entry_type: EntryType,
}

impl Node {
    /// New up a Node, given a NodeType instanc and an EntryType
    /// instance
    ///
    /// # Parameters
    ///
    /// * `identity` - The identity of the node. This may be Root,
    ///    a simple name, or a regular expression.
    /// * `entry_type` - The type of entity that the Node represents,
    ///    including `Directory,``Volume`, and hte special `Root` type
    ///    which may only appear at the oth index of the graph.
    ///
    /// # Returns
    ///   A new instance of Node
    pub fn new(identity: NodeType, entry_type: EntryType) -> Self {
        Self { identity, entry_type }
    }

    /// Specialized constructor function which returns a Root node.
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
            NodeType::Simple(strval) => strval.as_str() == other,
            NodeType::Regexp { name: _, pattern } => pattern.is_match(other.to_str().unwrap()),
        }
    }
}

impl std::default::Default for Node {
    fn default() -> Node {
        Node::new(NodeType::Simple("NONE".to_string()), EntryType::Directory)
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Node, ()> {
        Ok(Node::new(NodeType::Simple(s.to_string()), EntryType::Directory))
    }
}
