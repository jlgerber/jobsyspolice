use nom::{
    IResult,
    branch::alt,
    sequence::{tuple,preceded, delimited},
    bytes::complete::{tag},
    combinator::{ map, },
    character::complete::{char, space0, multispace0, },
};
use crate::jspt::helpers::*;

use crate::jspt::{Node, ParseResult, parse_metadata};

/// Parses a Node given an input str. The parser is composed of a number
/// of alternative parsers targetting specific types of nodes. 
pub fn parse_node(input: &str) -> IResult<&str, ParseResult> {
    alt((
        parse_node_pair,
        parse_node_envvar,
        parse_node_revar,
        parse_node_regexcomplex,
        parse_node_regexsimple,
        parse_node_simple,
    ))
    (input)
}

#[cfg(test)]
mod parse_node {
    use super::*;
    //use nom::error::ErrorKind;

    #[test]
    fn can_parse_simple() {
        let result = parse_node(r#" rd "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::Simple("rd".to_string(), None)) ) ) );
    }

    #[test]
    fn can_parse_node_pair() {
        let result = parse_node(r#"rd = RD "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_pair("rd", "RD", None)) ) )) ;
    }

    #[test]
    fn can_parse_node_envvar() {
        let result = parse_node(r#"rd = $$rdexpr "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_envvar("rd", "rdexpr",None)) )) ) ;
    }

    #[test]
    fn can_parse_node_revar() {
        let result = parse_node(r#"rd = $rdexpr "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_revar("rd", "rdexpr",None)) )) ) ;
    }

    #[test]
    fn can_parse_node_regexsimple() {
        let result = parse_node(r#"rd = "(foo|bar)" "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_regexsimple("rd", "(foo|bar)", None))) ) ) ;
    }

    #[test]
    fn can_parse_node_regexcomplex() {
        let result = parse_node_regexcomplex(r#"rd = "(foo|bar)" "(bla|mange)" "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_regexcomplex("rd", "(foo|bar)", "(bla|mange)", None )) ) )) ;
    }
}

fn parse_node_simple(input: &str) -> IResult<&str, ParseResult> {
    alt((
        parse_node_simple_meta,
        parse_node_simple_nometa,
    ))(input)
}
// parse simple node witout metadata
// EG
// rd_node =   rd
fn parse_node_simple_nometa(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
        delimited( space0, variable, space0),
        | item| {
           ParseResult::Node(Node::Simple(item.to_string(), None))
        } 
    ) 
    (input)
}

// parse the simple node with metadata
// EG
//rd_node =   rd [volume, owner:jgerber ]
fn parse_node_simple_meta(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
        tuple((
            preceded( space0, variable),
            parse_metadata
        )),
        | item| {
            let (var, meta) = item;
            let meta = if meta.is_empty() {None} else {Some(meta)};
           ParseResult::Node(Node::Simple(var.to_string(), meta))
        } 
    ) 
    (input)
}


#[cfg(test)]
mod parse_node_simple {
    use super::*;
    //use nom::error::ErrorKind;
    use crate::jspt::JsptMetadata;

    #[test]
    fn can_parse_node_simple() {
        let result = parse_node_simple(r#" rd"#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::Simple("rd".to_string(), None)) ) ) ) ;
    }

    #[test]
    fn can_parse_node_simple_meta() {
        let result = parse_node_simple(r#" rd [ volume ] "#);
        let  md = JsptMetadata::new().set_volume(true);
        assert_eq!(
            result, 
            Ok((
                "", 
                ParseResult::Node(
                    Node::Simple(
                        "rd".to_string(), 
                        Some(md)
                    )) ) ) ) ;
    }

}

// parse simple node - that is:
// rd_node =   rd
fn parse_node_pair(input: &str) -> IResult<&str,  ParseResult> {
    alt((
        parse_node_pair_meta,
        parse_node_pair_nometa,
    ))(input)
}

// parse a node pari without metadata
// EG
// rd_node = rd 
fn parse_node_pair_nometa(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
            tuple((
                preceded(space0, variable),
                preceded(space0, char('=')), 
                delimited( space0, variable, multispace0) 
            )),
        | item| {
            let (var,_,val) = item ;
             ParseResult::Node(Node::new_pair(var, val, None))
        } 
    ) 
    (input)
}

// parse a node pair with metadata
// EG
// rd_node = rd [ owner: foobar ]
fn parse_node_pair_meta(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
            tuple((
                preceded(space0, variable),
                preceded(space0, char('=')), 
                preceded( space0, variable) ,
                parse_metadata,
            )),
        | item| {
            let (var,_,val, meta) = item ;
            let meta = if meta.is_empty() {None} else {Some(meta)};
             ParseResult::Node(Node::new_pair(var, val, meta))
        } 
    ) 
    (input)
}


#[cfg(test)]
mod parse_node_pair {
    use super::*;
    //use nom::error::ErrorKind;
    use crate::jspt::JsptMetadata;

    #[test]
    fn can_parse_node_pair() {
        let result = parse_node_pair(r#"rd = RD "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_pair("rd", "RD", None)) )) ) ;
    }

    #[test]
    fn can_parse_node_pair_meta() {
        let md = JsptMetadata::new().set_volume(true).set_owner(Some("jgerber"));
        let result = parse_node_pair(r#"rd = RD [volume, owner:jgerber ]"#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_pair("rd", "RD", Some(md))) )) ) ;
    }

}

// parse a Node::EnvVar from input, with or without metadata. 
// eg
// rd = $rd
fn parse_node_envvar(input: &str) -> IResult<&str,  ParseResult> {
    alt((
        parse_node_envvar_meta,
        parse_node_envvar_nometa,
    ))
    (input)
}

// parse env variable node without metadata. regex node references a named regex. 
// EG
// `rd_node =   $$DD_RND`
fn parse_node_envvar_nometa(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
            tuple((
                // drops zero or more spaces in front of a variable (upper lower case number _-)
                preceded(space0, variable),
                // drop zero or more spaces in front of '='
                preceded(space0, char('=')), 
                // drop zero or more spaces around variable preceded by $$ and drop zero or more spaces and returns
                delimited( space0, preceded(tag("$$"),variable), multispace0) 
            )),
        | item| {
            let (var,_,val) = item ;
            ParseResult::Node( Node::new_envvar(var, val, None))
        } 
    ) 
    (input)
}

// Parse a envvar variable node with metadata from a &str.
// EG
// rd = $rd [volume]
fn parse_node_envvar_meta(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
            tuple((
                // drops zero or more spaces in front of a variable (upper lower case number _-)
                preceded(space0, variable),
                // drop zero or more spaces in front of '='
                preceded(space0, char('=')), 
                // drop zero or more spaces around variable preceded by $ and drop zero or more spaces and returns
                delimited( space0, preceded(tag("$$"),variable), space0),
                parse_metadata,
            )),
        | item| {
            let (var,_,val, meta) = item ;
            let meta = if meta.is_empty() {None} else {Some(meta)};
            ParseResult::Node( Node::new_envvar(var, val, meta))
        } 
    ) 
    (input)
}


#[cfg(test)]
mod parse_node_envvar {
    use super::*;
    //use nom::error::ErrorKind;

    #[test]
    fn can_parse_node_envvar() {
        let result = parse_node_envvar(r#"rd = $$rdexpr "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_envvar("rd", "rdexpr", None)) ) )) ;
    }

    #[test]
    fn can_parse_node_pair_with_return() {
        let result = parse_node_envvar(r#" rd = $$rdexpr
        "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_envvar("rd", "rdexpr", None) )) ) );
    }
}

// parse a Node::ReVar from input, with or without metadata. 
// eg
// rd = $rd
fn parse_node_revar(input: &str) -> IResult<&str,  ParseResult> {
    alt((
        parse_node_revar_meta,
        parse_node_revar_nometa,
    ))
    (input)
}

// parse revar variable node without metadata. regex node references a named regex. 
// EG
// `rd_node =   $rd`
fn parse_node_revar_nometa(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
            tuple((
                // drops zero or more spaces in front of a variable (upper lower case number _-)
                preceded(space0, variable),
                // drop zero or more spaces in front of '='
                preceded(space0, char('=')), 
                // drop zero or more spaces around variable preceded by $ and drop zero or more spaces and returns
                delimited( space0, preceded(tag("$"),variable), multispace0) 
            )),
        | item| {
            let (var,_,val) = item ;
            ParseResult::Node( Node::new_revar(var, val, None))
        } 
    ) 
    (input)
}

// Parse a revar variable node with metadata from a &str.
// EG
// rd = $rd [volume]
fn parse_node_revar_meta(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
            tuple((
                // drops zero or more spaces in front of a variable (upper lower case number _-)
                preceded(space0, variable),
                // drop zero or more spaces in front of '='
                preceded(space0, char('=')), 
                // drop zero or more spaces around variable preceded by $ and drop zero or more spaces and returns
                delimited( space0, preceded(tag("$"),variable), space0),
                parse_metadata,
            )),
        | item| {
            let (var,_,val, meta) = item ;
            let meta = if meta.is_empty() {None} else {Some(meta)};
            ParseResult::Node( Node::new_revar(var, val, meta))
        } 
    ) 
    (input)
}

#[cfg(test)]
mod parse_node_revar {
    use super::*;
    //use nom::error::ErrorKind;

    #[test]
    fn can_parse_node_revar() {
        let result = parse_node_revar(r#"rd = $rdexpr "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_revar("rd", "rdexpr", None)) ) )) ;
    }

    #[test]
    fn can_parse_node_pair_with_return() {
        let result = parse_node_revar(r#" rd = $rdexpr
        "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_revar("rd", "rdexpr", None) )) ) );
    }
}

// parse a simple regex node from input with our without metadata.
// eg
// rd_mode = $rd [volume]
fn parse_node_regexsimple(input: &str) -> IResult<&str,  ParseResult> {
    alt((
        parse_node_regexsimple_meta,
        parse_node_regexsimple_nometa,
    ))
    (input)
}

// parse regex variable node without metadata. regex node references a named regex
// `rd_node =   $rd`
fn parse_node_regexsimple_nometa(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
            tuple((
                // drops zero or more spaces in front of a variable (upper lower case number _-)
                preceded(space0, variable),
                // drop zero or more spaces in front of '='
                preceded(space0, char('=')), 
                // drop zero or more spaces around variable preceded by $ and drop zero or more spaces and returns
                delimited( space0, quoted_regex_str, multispace0) 
            )),
        | item| {
            let (var,_,val) = item ;
             ParseResult::Node(Node::new_regexsimple(var, val, None))
        } 
    ) 
    (input)
}

// parse a simple regex node with metadata 
// eg
// rd = $rd [owner: jgerber]
fn parse_node_regexsimple_meta(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
            tuple((
                // drops zero or more spaces in front of a variable (upper lower case number _-)
                preceded(space0, variable),
                // drop zero or more spaces in front of '='
                preceded(space0, char('=')), 
                // drop zero or more spaces around variable preceded by $ and drop zero or more spaces and returns
                delimited( space0, quoted_regex_str, space0),
                parse_metadata
            )),
        | item| {
            let (var,_, val, meta) = item ;
            let meta = if meta.is_empty() {None} else {Some(meta)};
            ParseResult::Node(Node::new_regexsimple(var, val, meta))
        } 
    ) 
    (input)
}

#[cfg(test)]
mod parse_node_regexsimple {
    use super::*;
    //use nom::error::ErrorKind;

    #[test]
    fn can_parse_node_regexsimple() {
        let result = parse_node_regexsimple(r#"rd = "(foo|bar)" "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_regexsimple("rd", "(foo|bar)", None)) ) )) ;
    }

    #[test]
    fn can_parse_node_regexsimplewith_return() {
        let result = parse_node_regexsimple(r#" rd = "[a-zA-Z0-1_-]"
        "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_regexsimple("rd", "[a-zA-Z0-1_-]", None) ) )) );
    }
}

// parse a complex regex node from a &str input with or without metadata.
fn parse_node_regexcomplex(input: &str) -> IResult<&str,  ParseResult> {
    alt((
        parse_node_regexcomplex_meta,
        parse_node_regexcomplex_nometa
    ))
    (input)
}

// parse regex variable node without metadata. regex node references a named regex
// EG
// `rd_node =   $rd`
fn parse_node_regexcomplex_nometa(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
            tuple((
                // drops zero or more spaces in front of a variable (upper lower case number _-)
                preceded(space0, variable),
                // drop zero or more spaces in front of '='
                preceded(space0, char('=')), 
                // drop zero or more spaces around variable preceded by $ and drop zero or more spaces and returns
                preceded( space0, quoted_regex_str),
                delimited( space0, quoted_regex_str, multispace0) 
            )),
        | item| {
            let (var,_,pos, neg) = item ;
            ParseResult::Node( Node::new_regexcomplex(var, pos, neg, None))
        } 
    ) 
    (input)
}

// parse a complex regex node with metadata from an input &str. 
// eg
// rd_node = $rd [volume ]
fn parse_node_regexcomplex_meta(input: &str) -> IResult<&str,  ParseResult> {
    map ( 
            tuple((
                // drops zero or more spaces in front of a variable (upper lower case number _-)
                preceded(space0, variable),
                // drop zero or more spaces in front of '='
                preceded(space0, char('=')), 
                // drop zero or more spaces around variable preceded by $ and drop zero or more spaces and returns
                preceded( space0, quoted_regex_str),
                delimited( space0, quoted_regex_str, space0),
                parse_metadata
            )),
        | item| {
            let (var,_,pos, neg, meta) = item ;
            let meta = if meta.is_empty() {None} else {Some(meta)};
            ParseResult::Node( Node::new_regexcomplex(var, pos, neg, meta))
        } 
    ) 
    (input)
}

#[cfg(test)]
mod parse_node_regexcomplex {
    use super::*;
    //use nom::error::ErrorKind;

    #[test]
    fn can_parse_node_regexcomplex() {
        let result = parse_node_regexcomplex(r#"rd = "(foo|bar)" "(bla|mange)" "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_regexcomplex("rd", "(foo|bar)", "(bla|mange)", None )) ) )) ;
    }

    #[test]
    fn can_parse_node_regexsimplewith_return() {
        let result = parse_node_regexcomplex(r#" rd = "[a-zA-Z0-1_-]" "(bla|mange)"
        "#);
        assert_eq!(result, Ok( ("", ParseResult::Node(Node::new_regexcomplex("rd", "[a-zA-Z0-1_-]","(bla|mange)", None) )) ) );
    }
}

