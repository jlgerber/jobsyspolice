use crate::Regexp;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum NodeType {
    Root,
    Name(String),
    Regexp { name: String, pattern: Regexp },
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum ValidType {
    Simple,
    Regexp,
}
