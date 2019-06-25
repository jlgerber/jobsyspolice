use nom::{
    IResult,
    sequence::{tuple,preceded, delimited},
    bytes::complete::{tag},
    combinator::{ map, },
    character::complete::{ space0, multispace0, alphanumeric1,},
};

use crate::jspt::{Header, ParseResult};


/// Parse the section header, consisting of an identifier surounded by square
/// brackets. 
/// EG
/// [regex]
pub fn parse_section_header(input: &str) -> IResult<&str, ParseResult> {
    map ( 
            tuple((
                preceded(space0,tag("[")),
                preceded(space0, alphanumeric1), 
                delimited( space0, tag("]"), multispace0) 
            )),
        | item| {
            let (_,header,_) = item ;
            match header {
                "regex" | "regexp" | "re" => ParseResult::Header(Header::Regex),
                "nodes" | "node" => ParseResult::Header(Header::Node),
                "graph"| "edge" | "edges" => ParseResult::Header(Header::Edge),
                _ => ParseResult::Header(Header::Unknown(header.to_string())),
            }
        } 
    ) 
    (input)
}

#[cfg(test)]
mod section_header {
    use super::*;
 
    #[test]
    fn can_parse_spaces_in_header() {
        let result = parse_section_header(" [ regex ]    ");
        assert_eq!(result, Ok(("",ParseResult::Header(Header::Regex))));
    }

    #[test]
    fn can_parse_spaces_in_header_with_carriage_return_ending() {
        let result = parse_section_header(r#" [ regex ]    
        "#);
        assert_eq!(result, Ok(("",ParseResult::Header(Header::Regex))));
    }

    #[test]
    fn can_parse_spaces_in_header_2() {
        let result = parse_section_header("[ regex ]");
        assert_eq!(result, Ok(("",ParseResult::Header(Header::Regex))));
    }

    #[test]
    fn can_parse_no_space_header() {
        let result = parse_section_header("[regex]");
        assert_eq!(result, Ok(("",ParseResult::Header(Header::Regex))));
    }


    #[test]
    fn can_parse_no_space_nodes() {
        let result = parse_section_header("[nodes]");
        assert_eq!(result, Ok(("",ParseResult::Header(Header::Node))));
    }

    #[test]
    fn can_parse_no_space_node() {
        let result = parse_section_header("[node]");
        assert_eq!(result, Ok(("",ParseResult::Header(Header::Node))));
    }

    #[test]
    fn can_parse_no_space_graph() {
        let result = parse_section_header("[graph]");
        assert_eq!(result, Ok(("",ParseResult::Header(Header::Edge))));
    }

    #[test]
    fn can_parse_no_space_unknown() {
        let result = parse_section_header("[grapha]");
        assert_eq!(result, Ok(("",ParseResult::Header(Header::Unknown("grapha".to_string())))));
    }
}
