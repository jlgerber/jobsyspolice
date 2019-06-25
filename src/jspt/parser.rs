

use nom::{
    IResult,
    branch::alt,
    combinator::{all_consuming },
};

use crate::jspt::{ components::ParseResult};


pub mod header;
pub use header::{parse_section_header};

pub mod regex;
pub use self::regex::{parse_regex};

pub mod node;
pub use node::{ parse_node};

pub mod edge;
pub use edge::{parse_edges};

pub mod comment;
pub use comment::parse_comment;

pub mod empty;
pub use empty::parse_empty;

pub mod metadata;
pub use metadata::{parse_metadata, parse_components};

// Parse the input &str and apply a succession of parsers corresponding with, 
// the states of the parser, returning a ParseResult from the first successful 
// one, or an Nom Errror if unsuccessful.
fn parse_str(input: &str) -> IResult<&str, ParseResult> {
    all_consuming(
        alt((
            parse_edges,
            parse_comment,
            parse_section_header,
            parse_node,
            parse_regex,
            parse_empty,
        ))
    )(input)
}

/// Parser which parses the start state.
pub fn start_parser(input: &str) -> IResult<&str, ParseResult> {
    all_consuming(
        alt((
            parse_comment,
            parse_section_header,
            parse_empty,
        ))
    )(input)
}

/// Parser which parses a Regex in the regex state.
pub fn regex_parser(input: &str) -> IResult<&str, ParseResult> {
    all_consuming(
        alt((
            parse_comment,
            parse_section_header,
            parse_regex,
            parse_empty,
        ))
    )(input)
}

/// Parser which parses a node in the node state.
pub fn node_parser(input: &str) -> IResult<&str, ParseResult> {
    all_consuming(
        alt((
            parse_comment,
            parse_section_header,
            parse_node,
            parse_empty,
        ))
    )(input)
}

/// Parser which parses an edge in the edge state.
pub fn edge_parser(input: &str) -> IResult<&str, ParseResult> {
    all_consuming(
        alt((
            parse_edges,
            parse_comment,
            parse_empty,
        ))
    )(input)
}

/// Given an input &Str, apply parse_str and then match against the results, 
/// printing the Debug form of the inner result if Ok, otherwise returning an
/// error string. 
pub fn parse(input: &str) -> Result<(), String> {
    match parse_str(input) {
        Ok(("", ParseResult::Comment(comment))) => println!("Comment {:?}", comment),
        Ok(("", ParseResult::Header(header)))   => println!("Header  {:?}", header),
        Ok(("", ParseResult::Regex(r)))         => println!("Regex   {:?}", r),
        Ok(("", ParseResult::Node(n)))          => println!("Node    {:?}", n),
        Ok(("", ParseResult::Edges(e)))         => println!("Edges   {:?}", e),
        Ok(("", ParseResult::Empty))            => println!(""),

        Err(e) => return Err(format!("Error {:?}", e)),
        _ => println!("unexpected result"),
    }
    Ok(())
}