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

    /// Return a simple name for the node
    pub fn simple_name(&self) -> String {
        let mut name = match self.entry_type {
            EntryType::Directory => String::from("Dir( "),
            EntryType::Volume => String::from("Vol( "),
            EntryType::Root => String::from("Root()"),
        };
        match &self.identity {
            NodeType::Simple(n) => { name.push_str(n.as_str()); name.push_str(" )"); },
            NodeType::Regexp{name:n, pattern: r} => { name.push_str(format!("{} {} )", n.as_str(), r.as_str()).as_str());},
            NodeType::Root => (),
        }
        name
    }
}

impl PartialEq<std::ffi::OsStr> for Node {
    fn eq(&self, other: &std::ffi::OsStr) -> bool {
        // we cannot match a root node
        if self.entry_type == EntryType::Root {
            panic!("cannot compair osstr with EntryType::Root. Root should only be set once");
        }

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


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;
    use crate::Regexp;

    #[test]
    fn new_root_creates_root_node() {
        let root = Node::new_root();
        let expected = Node {
            identity: NodeType::Root,
            entry_type: EntryType::Root
        };
        assert_eq!(root, expected);
    }

    #[test]
    #[should_panic]
    fn osstr_cmp_with_simple_nodetype_root() {
        let simple = Node::new_root();
        let osstr = OsStr::new("foobar");
        assert_eq!(simple, *osstr);
    }

    #[test]
    fn osstr_cmp_with_simple_nodetype() {
        let simple = Node::new(
            NodeType::Simple("foobar".to_string()),
            EntryType::Directory
        );

        let osstr = OsStr::new("foobar");
        assert_eq!(simple, *osstr);
    }

    #[test]
    fn osstr_cmp_with_regexp_nodetype() {
        let re = Node::new(
            NodeType::Regexp {
                name: "sequence".to_string(),
                pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
            },
            EntryType::Directory
        );
        let osstr = OsStr::new("AD1A");
        assert_eq!(re, *osstr);
    }

    #[test]
    fn osstr_cmp_with_regexp_nodetype_not_equal() {
        let re = Node::new(
            NodeType::Regexp {
                name: "sequence".to_string(),
                pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
            },
            EntryType::Directory
        );
        // the 1 on the front should make the pattern match fail
        let osstr = OsStr::new("1AD1A");
        assert_ne!(re, *osstr);
    }

    #[test]
    fn simple_name_for_root() {
        let re = Node::new(
            NodeType::Root,
            EntryType::Root
        );
        assert_eq!(re.simple_name(), String::from("Root()"));
    }

    #[test]
    fn simple_name_for_dir_regex() {
        let re = Node::new(
            NodeType::Regexp {
                name: "sequence".to_string(),
                pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
            },
            EntryType::Directory
        );
        assert_eq!(re.simple_name(), String::from("Dir( sequence ^[A-Z]+[A-Z 0-9]*$ )"));
    }

    #[test]
    fn simple_name_for_vol_regex() {
        let re = Node::new(
            NodeType::Regexp {
                name: "sequence".to_string(),
                pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
            },
            EntryType::Volume
        );
        assert_eq!(re.simple_name(), String::from("Vol( sequence ^[A-Z]+[A-Z 0-9]*$ )"));
    }

    #[test]
    fn simple_name_for_dir_simple() {
        let re = Node::new(
            NodeType::Simple(String::from("DEV01")),
            EntryType::Directory
        );
        assert_eq!(re.simple_name(), String::from("Dir( DEV01 )"));
    }

    #[test]
    fn simple_name_for_vol_simple() {
        let re = Node::new(
            NodeType::Simple(String::from("DEV01")),
            EntryType::Volume
        );
        assert_eq!(re.simple_name(), String::from("Vol( DEV01 )"));
    }

}