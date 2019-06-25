use crate::jspt::ParseResult;
use nom::{
    branch::alt,
    bytes::complete::{tag},
    combinator::{rest,map},
    character::complete::{space0},
    IResult,
    sequence::{preceded},
};


/// Parser function which parses a comment given a string. A comment
/// is defined as zero or more spaces, followed by a '#', followed by 
/// anything.
/// 
/// # Parameters
/// 
/// * `input` - a &str 
/// 
/// # Returns
/// an IResult having an Ok value of (&str, ParseResult), and an Err
/// value of (&str, ErrorKind)
pub fn parse_comment(input: &str) -> IResult<&str, ParseResult> {
    map(
        preceded(
            preceded(
                space0, 
                alt((
                    tag("#"),
                    tag("//")
                ))
            ),
            rest
        ), 
        |item: &str| {
            ParseResult::Comment(item.to_string())
        }
    )
    (input)
}


#[cfg(test)]
mod comment {
    use super::*;
 
    #[test]
    fn can_parse_comment_pound() {
        let c = parse_comment(" # this is a comment");
        assert_eq!(c, Ok(("", ParseResult::Comment(" this is a comment".to_string()))));
    }

    #[test]
    fn can_parse_comment_double_slash() {
        let c = parse_comment(" // this is a comment");
        assert_eq!(c, Ok(("", ParseResult::Comment(" this is a comment".to_string()))));
    }
 
    #[test]
    fn can_parse_comment_2() {
        let c = parse_comment(" # this is a comment    ");
        assert_eq!(c, Ok(("", ParseResult::Comment( " this is a comment    ".to_string()))));
    }
}
