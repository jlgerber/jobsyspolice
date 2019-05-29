use crate::{ EntryType, NodeType, Regexp };
use serde::{ Deserialize, Serialize, self };
use std::str::FromStr;
/// The Node caries information about a specific
/// directory or file within the candidate jobsystem
/// graph. This information is used to validate
/// candidate paths in order to determine wheither or not
/// they are valid.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Node {
    identity: NodeType,
    entry_type: EntryType,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<String>,
}

impl Node {
    /// New up a Node, given a NodeType instance and an EntryType
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
    pub fn new(identity: NodeType, entry_type: EntryType, owner: Option<String>) -> Self {
        Self { identity, entry_type, owner }
    }

    /// Specialized constructor function which returns a Root node.
    pub fn new_root() -> Self {
        Self {
            identity: NodeType::Root,
            entry_type: EntryType::Root,
            owner: None,
        }
    }

    pub fn new_regexp<I: Into<String>>(name: I, re: &str, owner: Option<String>) -> Node {
        Node::new(
            NodeType::RegEx {
                name: name.into(),
                pattern: Regexp::new(re).unwrap(),
                exclude: None,
            },
            EntryType::Directory,
            owner,
        )
    }

    /// For those times when you need to define an exclusion regex in addition to a match regex.
    /// We provide this call, as the library we use does not support forward matches.
    pub fn new_regexp_adv<I: Into<String>>(name: I, re: &str, exclude_re: &str, owner: Option<String>) -> Node {
        Node::new(
            NodeType::RegEx {
                name: name.into(),
                pattern: Regexp::new(re).unwrap(),
                exclude: Some(Regexp::new(exclude_re).unwrap()),
            },
            EntryType::Directory,
            owner,
        )
    }

    /// Return a simple name for the node
    pub fn display_name(&self) -> String {
        let mut name = String::new();
        match &self.identity {
            NodeType::Simple(n) => { name.push_str(n.as_str()); },
            NodeType::RegEx{name:n, pattern: r, exclude: None} => { name.push_str(format!("{} regex: '{}'", n.as_str(), r.as_str()).as_str());},
            NodeType::RegEx{name:n, pattern: r, exclude: Some(excl)} => { name.push_str(format!("{} regex: '{}' exclude: '{}'", n.as_str(), r.as_str(), excl.as_str()).as_str());},
            NodeType::Root => name.push_str("Root()"),
        }
        if let Some(ref n) = self.owner {
            name.push_str(format!(" [{}]", n).as_str());
        }
        name
    }

    /// Set the owner to someone after instantiation.
    /// `set_owner` consumes self
    ///
    /// # Examples
    /// There are two ways to use `set_owner`. The first is by
    /// chaining
    /// ```
    /// let node = Node::new().set_owner("jobsys");
    /// ```
    /// The second way is to reassign the return value
    ///
    /// ```
    /// let node = Node::new();
    /// let node = node.set_owner("ddinst");
    /// ```
    pub fn set_owner<I>(mut self, owner: I ) -> Node where I: Into<String> {
        self.owner = Some(owner.into());
        self
    }

    /// Set the entry_type to EntryType::Volume
    /// set_volume consumes self and must be used thusly
    ///
    /// # Examples
    /// Like `set_owner`, there are two ways to use `set_volume`.
    /// You may chain calls:
    /// ```
    /// let node = Node::new().set_volume();
    /// ```
    /// Or, you may reassign the return value:
    /// ```
    /// let node = Node::new(...);
    /// let node = node.set_volume();
    /// ```
    pub fn set_volume(mut self) -> Node  {
        self.entry_type = EntryType::Volume;
        self
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
            NodeType::RegEx { name: _, pattern, exclude: None } => pattern.is_match(other.to_str().unwrap()),
            NodeType::RegEx { name: _, pattern, exclude: Some(exc) } => !exc.is_match(other.to_str().unwrap()) && pattern.is_match(other.to_str().unwrap()),

        }
    }
}

impl std::default::Default for Node {
    fn default() -> Node {
        Node::new(NodeType::Simple(s!("NONE")), EntryType::Directory, None)
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Node, ()> {
        Ok(Node::new(NodeType::Simple(s!(s)), EntryType::Directory, None))
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
            entry_type: EntryType::Root,
            owner: None
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
            NodeType::Simple(s!("foobar")),
            EntryType::Directory,
            None
        );

        let osstr = OsStr::new("foobar");
        assert_eq!(simple, *osstr);
    }

    #[test]
    fn osstr_cmp_with_regexp_nodetype() {
        let re = Node::new(
            NodeType::RegEx {
                name: s!("sequence"),
                pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
                exclude: None,
            },
            EntryType::Directory,
            None
        );
        let osstr = OsStr::new("AD1A");
        assert_eq!(re, *osstr);
    }

    #[test]
    fn osstr_cmp_with_exlude_nodetype() {
        let re = Node::new(
            NodeType::RegEx {
                name: s!("sequence"),
                pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
                exclude: Some(Regexp::new(r"^(SHARED|etc)$").unwrap()),
            },
            EntryType::Directory,
            None
        );
        let osstr = OsStr::new("SHARE");
        assert_eq!(re, *osstr);
    }

    #[test]
    fn osstr_cmp_with_exlude_nodetype_notequal() {
        let re = Node::new(
            NodeType::RegEx {
                name: s!("sequence"),
                pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
                exclude: Some(Regexp::new(r"^(SHARED|etc)$").unwrap()),
            },
            EntryType::Directory,
            None
        );
        let osstr = OsStr::new("SHARED");
        assert_ne!(re, *osstr);
    }

    #[test]
    fn osstr_cmp_with_regexp_nodetype_not_equal() {
        let re = Node::new(
            NodeType::RegEx {
                name: s!("sequence"),
                pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
                exclude: None,
            },
            EntryType::Directory,
            None
        );
        // the 1 on the front should make the pattern match fail
        let osstr = OsStr::new("1AD1A");
        assert_ne!(re, *osstr);
    }

    #[test]
    fn simple_name_for_root() {
        let re = Node::new(
            NodeType::Root,
            EntryType::Root,
            None
        );
        assert_eq!(re.display_name(), s!("Root()"));
    }

    #[test]
    fn simple_name_for_dir_regex() {
        let re = Node::new(
            NodeType::RegEx {
                name: s!("sequence"),
                pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
                exclude: None,
            },
            EntryType::Directory,
            None
        );
        assert_eq!(re.display_name(), s!("sequence regex: '^[A-Z]+[A-Z 0-9]*$'"));
    }

    #[test]
    fn simple_name_for_vol_regex() {
        let re = Node::new(
            NodeType::RegEx {
                name: s!("sequence"),
                pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
                exclude: None,
            },
            EntryType::Volume,
            None
        );
        assert_eq!(re.display_name(), s!("sequence regex: '^[A-Z]+[A-Z 0-9]*$'"));
    }

    #[test]
    fn simple_name_for_vol_regex_with_exclude() {
        let re = Node::new(
            NodeType::RegEx {
                name: s!("sequence"),
                pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
                exclude: Some(Regexp::new(r"^(SHARED|etc)$").unwrap()),
            },
            EntryType::Volume,
            None
        );
        assert_eq!(re.display_name(), s!("sequence regex: '^[A-Z]+[A-Z 0-9]*$' exclude: '^(SHARED|etc )$'"));
    }

    #[test]
    fn simple_name_for_dir_simple() {
        let re = Node::new(
            NodeType::Simple(s!("DEV01")),
            EntryType::Directory,
            None
        );
        assert_eq!(re.display_name(), s!("DEV01"));
    }

    #[test]
    fn simple_name_for_vol_simple() {
        let re = Node::new(
            NodeType::Simple(s!("DEV01")),
            EntryType::Volume,
            None
        );
        assert_eq!(re.display_name(), s!("DEV01"));
    }

}