pub mod edge;
pub use edge::Edge;
pub mod header;
pub use header::Header;
pub mod regex;
pub use regex::JsptRegex;
pub mod node;
pub use node::Node;
pub mod metadata;
pub use metadata::{JsptMetadata, MetadataComponent};

/// Like the name implies, categorize the results of the line parser. 
/// Each line parsed will either be a ParseResult or a JSPTemplateError
#[derive(Debug,PartialEq,Eq)]
pub enum ParseResult {
    /// The header, eg [node] , signals the start of a new state in
    /// the parsing state machine
    Header(Header),
    /// A named regular expression
    Regex(JsptRegex),
    /// A node 
    Node(Node),
    /// The connection between two nodes
    Edges(Vec<Edge>),
    /// A comment, preceded by the comment token ('#')
    Comment(String),
    /// An emtpy line
    Empty,
}