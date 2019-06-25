use nom::{
    IResult,
    combinator::{ map, },
    character::complete::{ multispace0},
};

use crate::jspt::ParseResult;

/// A parser that parses an input and identifies spaces only.
pub fn parse_empty(input: &str) -> IResult<&str, ParseResult> {
    map(
        multispace0, 
        |_item: &str| {
            ParseResult::Empty
        }
    )
    (input)
}

