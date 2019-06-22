use crate::Regexp;
use std::fmt::{ Display, Formatter, self };
use serde::{ Deserialize, Serialize };

/// A node in the jobsystem graph may be one of several
/// types, represented by the NodeType enum.
///
/// - `NodeType::Root` is a special variant that represents the
/// root of the graph. It has no analog on disk, and exists
/// for book keeping purposes.
/// - `NodeType::Simple` wraps a String and is used to represent
/// explicit directory and file names, such as `dd`, `etc`, and
/// `SHARED`.
/// - `NodeType::RegEx` wraps a Regexp type which reprents a range
/// of potentially valid names for a directory or file, dictated
/// by the regex stored in the type.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Root,
    Untracked, // used to handle directories outside of the graph
    Simple(String),
    RegEx {
        name: String,
        pattern: Regexp,
        #[serde(skip_serializing_if = "Option::is_none")]
        exclude: Option<Regexp>
    },
}
impl NodeType {
    pub fn new_regex(name: String, pattern: Regexp, exclude: Option<Regexp>) -> NodeType {
        NodeType::RegEx{name, pattern, exclude}
    }
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            NodeType::RegEx{name, pattern, exclude: None} =>  write!(f, "RegEx{{'{}', '{}'}}", name, pattern.as_str()),
            NodeType::RegEx{name, pattern, exclude: Some(neg_pattern) } =>  write!(f, "RegEx{{'{}', '{}', '{}'}}", name, pattern.as_str(), neg_pattern.as_str()),
            NodeType::Simple(name) =>  write!(f, "Simple('{}')",name),
            NodeType::Root =>  write!(f, "Root"),
            NodeType::Untracked => write!(f, "Untracked"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum ValidType {
    Simple,
    RegEx,
}
