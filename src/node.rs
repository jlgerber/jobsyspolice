use crate::{ EntryType, Navalias, NodeType, User, Metadata};
use serde::{ Deserialize, Serialize, self };

#[allow(unused_imports)]
use log;
use std::{fmt::{ Display, Formatter, self} };

/// The Node caries information about a specific
/// directory or file within the candidate jobsystem
/// graph. This information is used to validate
/// candidate paths in order to determine wheither or not
/// they are valid.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Node {
    identity: NodeType,
    entry_type: EntryType,
    metadata: Metadata
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Node{{identity: {}, entry_type: {}, metadata: {:?} }}",
            self.identity,
            self.entry_type,
            self.metadata,
        )
    }
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
    pub fn new(
        identity: NodeType, 
        entry_type: EntryType,
        owner: Option<User>, 
        perms: Option<String>, 
        varname: Option<String>, 
        autocreate: bool, 
        navalias: Option<Navalias>
    ) -> Self {
        Self { 
            identity, 
            entry_type,
            metadata:  Metadata::from_components(owner, perms, varname, autocreate, navalias)
        }
    }

    pub fn new_simple(identity: NodeType, entry_type: EntryType, metadata: Metadata) -> Self {
        Self { 
            identity, 
            entry_type,
            metadata,
        }
    }

    /// Specialized constructor function which returns a Root node.
    pub fn new_root() -> Self {
        Self {
            identity: NodeType::Root,
            entry_type: EntryType::Root,
            metadata: Metadata::new(),
        }
    }
    /// Specialized constructor function which returns an Untracked node.
    pub fn new_untracked() -> Self {
        Self {
            identity: NodeType::Untracked,
            entry_type: EntryType::Untracked,
            metadata: Metadata::new()
        }
    }

    pub fn entry_type(&self) -> &EntryType {
        &self.entry_type
    }

    /// Retrieve a reference to metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// return a mutable reference to metadata
    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    pub fn identity(&self) -> &NodeType {
        &self.identity
    }

    /// Return a simplified name for the node.
    // TODO: add a simplename: Option<RefCell<String>> to Node to cache the simple name
    pub fn display_name(&self) -> String {
        let mut name = String::new();
        match &self.identity {
            NodeType::Simple(n) => { name.push_str(n.as_str()); },
            NodeType::RegEx{name:n, pattern: r, exclude: None} => { name.push_str(format!("{} regex: '{}'", n.as_str(), r.as_str()).as_str());},
            NodeType::RegEx{name:n, pattern: r, exclude: Some(excl)} => { name.push_str(format!("{} regex: '{}' exclude: '{}'", n.as_str(), r.as_str(), excl.as_str()).as_str());},
            NodeType::Root => name.push_str("Root()"),
            NodeType::Untracked => name.push_str("Untracked()"),
        }

        let mut meta = Vec::new();

        if let Some(ref n) = self.metadata().owner() {
            meta.push(format!("owner:{}",n));
            //name.push_str(format!(" [{}]", n).as_str());
        }

        if let Some(ref n) = self.metadata().perms() {
            meta.push(format!("perms:{}", n));
            //name.push_str(format!(" [{}]", n).as_str());
        }
        
        if self.metadata().autocreate() {
            meta.push(String::from("autocreate"));
            //name.push_str(format!(" [{}]", n).as_str());
        }


        if let Some(ref n) = self.metadata().navalias() {
            match n {
                Navalias::Simple(name) => meta.push(format!("navalias:{}", name)),
                Navalias::Complex{name, value} => meta.push(format!("navalias:{}={}", name, value)),
            }
        }

        if meta.len() > 0 {
            name.push_str(format!(" [{}]", meta.join(", ")).as_str());
        }

        name
    }

    /*
    /// Set the owner to someone after instantiation.
    /// `set_owner` consumes self
    ///
    /// # Examples
    /// There are two ways to use `set_owner`. The first is by
    /// chaining
    /// ```
    /// use jsp::{Node, jspnode, NodeType, EntryType };
    /// let node = jspnode!("FOO").set_owner("jobsys");
    /// ```
    /// The second way is to reassign the return value
    ///
    /// ```
    /// use jsp::{Node, jspnode, NodeType, EntryType };
    /// let node = jspnode!("FOO");
    /// let node = node.set_owner("ddinst");
    /// ```
    /// 
    pub fn set_owner<I>(mut self, owner: I ) -> Node where I: Into<User> {
        log::trace!("set_owner before {:?}", self.metadata().owner());
        self.metadata.set_owner(Some(owner.into()));
        log::trace!("set owwer after {:?}", self.metadata().owner());
        self
    }
*/
    /// Set the entry_type to EntryType::Volume
    /// set_volume consumes self and must be used thusly
    ///
    /// # Examples
    /// Like `set_owner`, there are two ways to use `set_volume`.
    /// You may chain calls:
    /// ```rust
    /// use jsp::{Node, jspnode, NodeType, EntryType };
    /// let node = jspnode!("FOO").set_volume();
    /// ```
    /// Or, you may reassign the return value:
    /// ```rust
    /// use jsp::{ Node, jspnode, NodeType, EntryType };
    /// let node = jspnode!("FOO");
    /// let node = node.set_volume();
    /// ```
    pub fn set_volume(mut self) -> Node  {
        self.entry_type = EntryType::Volume;
        self
    }
    
    pub fn set_metadata(mut self, metadata: Metadata) -> Node {
        self.metadata = metadata;
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
            NodeType::Untracked => true,
            NodeType::Simple(strval) => strval.as_str() == other,
            NodeType::RegEx { pattern, exclude: None, .. } => pattern.is_match(other.to_str().unwrap()),
            NodeType::RegEx {  pattern, exclude: Some(exc), .. } => 
                !exc.is_match(other.to_str().unwrap()) && pattern.is_match(other.to_str().unwrap()),
        }
    }
}

impl std::default::Default for Node {
    fn default() -> Node {
        Node::new(NodeType::Simple(s!("NONE")), EntryType::Directory, None, None, None, false, None)
    }
}


#[cfg(test)]
mod node_tests {
    use super::*;
    use std::ffi::OsStr;
    use crate::Regexp;

    #[test]
    fn new_root_creates_root_node() {
        let root = Node::new_root();
        let expected = Node {
            identity: NodeType::Root,
            entry_type: EntryType::Root,
            metadata: Metadata::new()
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
    fn osstr_cmp_with_simple_nodetype_untracked() {
        let simple = Node::new_untracked();
        let osstr = OsStr::new("foobar");
        assert_eq!(simple, *osstr);
    }

    #[test]
    fn osstr_cmp_with_simple_nodetype() {
        let simple = Node::new(
            NodeType::Simple(s!("foobar")),
            EntryType::Directory,
            None, None, None, false, None
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
            None, None, None, false, None
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
            None, None, None, false, None
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
            None, None, None, false, None
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
            None, None, None, false, None
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
            None, None, None, false, None
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
            None, None, None, false, None
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
            None, None, None, false, None
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
            None, None, None, false, None
        );
        assert_eq!(re.display_name(), s!("sequence regex: '^[A-Z]+[A-Z 0-9]*$' exclude: '^(SHARED|etc)$'"));
    }

    #[test]
    fn simple_name_for_dir_simple() {
        let re = Node::new(
            NodeType::Simple(s!("DEV01")),
            EntryType::Directory,
            None, None, None, false, None
        );
        assert_eq!(re.display_name(), s!("DEV01"));
    }

    #[test]
    fn simple_name_for_vol_simple() {
        let re = Node::new(
            NodeType::Simple(s!("DEV01")),
            EntryType::Volume,
            None, None, None, false, None 
        );
        assert_eq!(re.display_name(), s!("DEV01"));
    }
}

#[cfg(test)]
mod jspnode_tests {
    use super::*;
    use crate::jspnode;

    #[test]
    fn can_define_show_with_perms_metadata() {
        let re = jspnode!("DEV01", "perms" => "777");
        assert_eq!(re.display_name(), s!("DEV01 [perms:777]"));
    }


    #[test]
    fn can_define_show_with_owner_metadata() {
        let re = jspnode!("DEV01", "owner" => "jgerber");
        assert_eq!(re.display_name(), s!("DEV01 [owner:jgerber]"));
    }

    #[test]
    fn can_create_show_with_autocreate() {
        let re = jspnode!("DEV01", "autocreate" => "true");
        //assert!(re.metadata().autocreate());
        assert_eq!(re.display_name(), s!("DEV01 [autocreate]"));
    }

    #[test]
    fn can_create_show_with_owner_ande_autocreate_metadata() {
        let re = jspnode!("DEV01", "autocreate" => "true", "owner" => "jgerber");
        //assert!(re.metadata().autocreate());
        assert_eq!(re.display_name(), s!("DEV01 [owner:jgerber, autocreate]"));
    }

    #[test]
    fn can_create_show_with_owner_autocreate_and_simple_navalias() {
        let re = jspnode!("DEV01", "autocreate" => "true", "owner" => "jgerber", "navalias" => "cs");
        //assert!(re.metadata().autocreate());
        assert_eq!(re.display_name(), s!("DEV01 [owner:jgerber, autocreate, navalias:cs]"));
    }

    #[test]
    fn can_create_show_with_owner_autocreate_and_complex_navalias() {
        let re = jspnode!("DEV01", "autocreate" => "true", "owner" => "jgerber", "navalias" => "cs=work.$USER");
        assert_eq!(re.display_name(), s!("DEV01 [owner:jgerber, autocreate, navalias:cs=work.$USER]"));
    }

}