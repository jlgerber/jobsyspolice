use nom::{
    IResult,
    branch::alt,
    sequence::{tuple,preceded, delimited},
    combinator::{ map, },
    character::complete::{char, space0, multispace0,},// alphanumeric1,},
};

use crate::jspt::helpers::*;

use crate::jspt::{JsptRegex, ParseResult};

/// Parse regex from an input &str. Could be either simple or complex
pub fn parse_regex(input: &str) -> IResult<&str,  ParseResult> {
    alt((
        parse_regex_complex,
        parse_regex_simple,
    ))(input)
}

#[cfg(test)]
mod parse_regex {
    use super::*;
    //use nom::error::ErrorKind;

    #[test]
    fn can_parse_regex_complex() {
        let result = parse_regex(r#" foobar = "[a-zA-Z]" "(hello|world)" "#);
        assert_eq!(result, 
            Ok( 
                (
                    "", 
                    ParseResult::Regex(JsptRegex::Complex{
                        name:"foobar".to_string(), 
                        positive: "[a-zA-Z]".to_string(), 
                        negative:"(hello|world)".to_string() 
                    }) 
                ) 
            ) 
        );
    }

    #[test]
    fn can_parse_regex_simple() {
        let result = parse_regex(r#" foobar = "[a-zA-Z]" "#);
        assert_eq!(result, Ok( ("", 
        ParseResult::Regex(JsptRegex::Simple{name:"foobar".to_string(), value: "[a-zA-Z]".to_string()}) ) ) );
    }
}

// parse simple regex
// EG
// num_under =   "[0-9_]+"
fn parse_regex_simple(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
            tuple((
                preceded(space0, variable),
                preceded(space0, char('=')), 
                delimited( space0, quoted_regex_str, multispace0) 
            )),
        | item| {
            let (variable,_,re) = item ;
             ParseResult::Regex(JsptRegex::Simple{name: variable.to_string(), value: re.to_string()})
        } 
    ) 
    (input)
}

#[cfg(test)]
mod parse_regex_simple {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn can_parse_regex_simple() {
        let result = parse_regex_simple(r#" foobar = "[a-zA-Z]" "#);
        assert_eq!(result, Ok( ("", 
        ParseResult::Regex(JsptRegex::Simple{name:"foobar".to_string(), value: "[a-zA-Z]".to_string()} )
        ) ) );
    }

    #[test]
    fn can_parse_regex_simple_with_return() {
        let result = parse_regex_simple(r#" foobar = "[a-zA-Z]" 
        "#);
        assert_eq!(result, Ok( ("", 
        ParseResult::Regex(JsptRegex::Simple{name:"foobar".to_string(), value: "[a-zA-Z]".to_string()})
         ) ) );
    }

    #[test]
    fn can_parse_regex_simple_with_carriage_return() {
        let result = parse_regex_simple(r#" foobar = "[a-zA-Z]" 
        "#);
        assert_eq!(result, Ok( ("", 
        ParseResult::Regex(JsptRegex::Simple{name:"foobar".to_string(), value: "[a-zA-Z]".to_string()})
         ) ) );
    }

    #[test]
    fn fails_regex_simple_missing_quote() {
        let result = parse_regex_simple(r#" foobar = "[a-zA-Z] "#);
        assert_eq!(result, Err(nom::Err::Error((" ", ErrorKind::Tag)))) ;
    }

    #[test]
    fn fails_regex_simple_space() {
        let result = parse_regex_simple(r#" foobar = "[a-zA-Z] " "#);
        assert_eq!(result, Err(nom::Err::Error((" \" ", ErrorKind::Tag)))) ;
    }
}

// Parsex complex regex from input &str, which has positive and negative matches
// EG
// shot =   "[0-9_a-zA-Z]+" "(etc|SHARED|lib)"
fn parse_regex_complex(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
            tuple((
                preceded(space0, variable),
                preceded(space0, char('=')), 
                preceded( space0, quoted_regex_str) ,
                delimited( space0, quoted_regex_str, multispace0) 
            )),
        | item| {
            let (variable,_,pos,neg) = item;

             ParseResult::Regex(JsptRegex::Complex{
                 name: variable.to_string(), 
                 positive: pos.to_string(), 
                 negative: neg.to_string()
            })
        } 
    ) 
    (input)
}


#[cfg(test)]
mod parse_regex_complex {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn can_parse_regex_complex() {
        let result = parse_regex_complex(r#" foobar = "[a-zA-Z]" "(hello|world)" "#);
        assert_eq!(result, 
            Ok( 
                (
                    "", 
                    ParseResult::Regex(JsptRegex::Complex{
                        name:"foobar".to_string(), 
                        positive: "[a-zA-Z]".to_string(), 
                        negative:"(hello|world)".to_string() 
                    } )
                ) 
            ) 
        );
    }

    #[test]
    fn can_parse_regex_complex_with_return() {
        let result = parse_regex_complex(r#" foobar = "[a-zA-Z]" "(hello|world)"
        "#);
        
        assert_eq!(result, 
            Ok( 
                (
                    "", 
                    ParseResult::Regex(JsptRegex::Complex{
                        name:"foobar".to_string(), 
                        positive: "[a-zA-Z]".to_string(), 
                        negative:"(hello|world)".to_string() 
                    } )
                ) 
            ) 
        );    
    }


    #[test]
    fn fails_parse_regex_complex_missing_quote() {
        let result = parse_regex_complex(r#" foobar = "[a-zA-Z]" "(hello|world) "#);
        assert_eq!(result, Err(nom::Err::Error((" ", ErrorKind::Tag)))) ;
    }

    #[test]
    fn fails_parse_regex_complex_space() {
        let result = parse_regex_complex(r#" foobar = "[a-zA-Z] " "(hello|world)" "#);
        assert_eq!(result, Err(nom::Err::Error((" \" \"(hello|world)\" ", ErrorKind::Tag)))) ;
    }
}
