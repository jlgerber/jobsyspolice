use crate::Regexp;

/// A node in the jobsystem graph may be one of several
/// types, represented by the NodeType enum.
///
/// - `NodeType::Root` is a special variant that represents the
/// root of the graph. It has no analog on disk, and exists
/// for book keeping purposes.
/// - `NodeType::Simple` wraps a String and is used to represent
/// explicit directory and file names, such as `dd`, `etc`, and
/// `SHARED`.
/// - `NodeType::Regexp` wraps a Regexp type which reprents a range
/// of potentially valid names for a directory or file, dictated
/// by the regex stored in the type.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum NodeType {
    Root,
    Simple(String),
    Regexp { name: String, pattern: Regexp },
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum ValidType {
    Simple,
    Regexp,
}
