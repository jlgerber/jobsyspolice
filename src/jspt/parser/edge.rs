use nom::{
    IResult,
    sequence::{tuple,preceded, delimited},
    bytes::complete::{tag},
    combinator::{ map},
    multi::many1,
    character::complete::{ space0, alphanumeric1,},
};

use crate::jspt::{helpers::*, Edge, ParseResult};


/// Parse input &str into a vector of edges. 
/// 
/// ```foo -> bar-> bla```
/// 
/// produces
/// ```vec![ Edge::new(foo,bar), Edge::new(bar, bla) ];```
pub fn parse_edges(input: &str) -> IResult<&str, ParseResult> {
    map(
        tuple((
            delimited(space0, variable, space0),
            many1(
                preceded(
                    tag("->"),
                    delimited(space0, variable, space0)
                )
            ),
        )),
        |item| {
            let (first, rest) = item ;
            let mut rval = Vec::with_capacity(rest.len());
            let mut node1 = first;
            for node2 in rest {
                rval.push(
                    Edge::new(node1, node2)
                );
                node1 = node2;
            }
            ParseResult::Edges(rval)
        }
    )(input)
}

#[cfg(test)]
mod parse_edges {
    use super::*;
 
    #[test]
    fn can_parse_edge() {
        let result = parse_edges(" foo->bar");
        assert_eq!(result, Ok(("", ParseResult::Edges(vec![Edge::new("foo", "bar")]))));
    }

    #[test]
    fn can_parse_spaces_in_header_with_space_ending() {
        let result = parse_edges(r#"foo -> bar  "#);
        assert_eq!(result, Ok(("",ParseResult::Edges(vec![Edge::new("foo", "bar")]))));
    }


    #[test]
    fn can_parse_edges_2() {
        let result = parse_edges(" foo->bar -> bla ");
        assert_eq!(
            result, 
            Ok(("",
                ParseResult::Edges(vec![
                    Edge::new("foo", "bar"),
                    Edge::new("bar", "bla"),
                ])
        )));
    }


    #[test]
    fn can_parse_edges_3() {
        let result = parse_edges(" foo->bar -> bla -> flarg  ");
        assert_eq!(
            result, 
            Ok(("",
                ParseResult::Edges(vec![
                    Edge::new("foo", "bar"),
                    Edge::new("bar", "bla"),
                    Edge::new("bla", "flarg"),
                ])
        )));
    }

    #[test]
    fn can_parse_edges_4() {
        let result = parse_edges(" foo->bar -> bla -> flarg  -> picklerick ");
        assert_eq!(
            result, 
            Ok(("",
                ParseResult::Edges(vec![
                    Edge::new("foo", "bar"),
                    Edge::new("bar", "bla"),
                    Edge::new("bla", "flarg"),
                    Edge::new("flarg", "picklerick"),
                ])
        )));
    }
    
}