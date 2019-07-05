//! minimatch 
//! 
//! Problem: we want to be able to locate the jobsystem template 
//! but we need to evaluate the call that leverages the templete in 
//! order to do so. 
//! EG
//! We store the template relative to the show. But how do we know
//! where the show is in the request? Normally, that is the job of
//! ... the template. So, we could have two templates... a mini 
//! template that just parses enough and a full template. However, that 
//! might cause problems or at least require a redesign to support
//! Making a request with more SearchItems that the can resolve. Hmmm
//! maybe there is another way? Write a mini request parser that just
//! identifies the location of the template by pattern matching against
//! 
//! - levelspec (foo.anything) x
//! - a full path /dd/shows/<show>/anything
//! 
//! Lets write a simple parser to handle this
use nom::{
    IResult,
    branch::alt,
    sequence::{terminated, delimited},
    bytes::complete::{tag},
    combinator::{ map, rest},
};
use crate::{constants, jspt::helpers::*, JSPError};


// The results from parsing the show out of the front of 
// the user input. 
// - Show(String) is for explicitly defined show
// - EnvVar is for retrieving the show name from the environment
#[derive(Debug, PartialEq, Eq, Clone)]
enum MinimatchResult {
    Show(String),
    EnvVar,
}

pub fn parse_show_from_arg(input: &str) -> Result<String, JSPError> {
    match parse_show(input) {
        Ok((_,result)) => {
            match result {
                MinimatchResult::Show(s) => Ok(s),
                MinimatchResult::EnvVar => {
                    let result = std::env::var(constants::JSP_SHOW_ENVVAR)?;
                    Ok(result)
                }
            }
        },
        Err(_) => Err(JSPError::TemplateError("Unable parse show from input".to_string() ))
    }
}



#[cfg(test)]
mod parse_show_tests {
    use super::*;

    #[test]
    fn can_get_show() {
        let show = parse_show_from_arg("/dd/shows/FOOBAR");
        assert_eq!(show, Ok( "FOOBAR".to_string() ));
    }

     #[test]
    fn can_get_show_from_levelspec() {
        let show = parse_show_from_arg("foobar.rd.bla");
        assert_eq!(show, Ok("FOOBAR".to_string() ));
    }

    #[test]
    fn can_get_show_from_levelspec_show() {
        let show = parse_show_from_arg("foobar");
        assert_eq!(show, Ok("FOOBAR".to_string() ));
    }


    #[test]
    fn can_get_show_from_rel_levelspec() {
        std::env::set_var("DD_SHOW", "BLARG");
        let show = parse_show_from_arg(".rd.bla");
        assert_eq!(show, Ok("BLARG".to_string() ) );
    }
}


fn parse_show(input: &str) -> IResult<&str, MinimatchResult> {
    
    alt((
        get_show_from_fullpath,
        get_show_from_rel_levelspec,
        get_show_from_levelspec,
    ))
    (input)
}


#[cfg(test)]
mod parse_show_tests_private {
    use super::*;

    #[test]
    fn can_get_show() {
        let show = parse_show("/dd/shows/FOOBAR");
        assert_eq!(show, Ok(("",MinimatchResult::Show("FOOBAR".to_string())  )));
    }

     #[test]
    fn can_get_show_from_levelspec() {
        let show = parse_show("foobar.rd.bla");
        assert_eq!(show, Ok(("",MinimatchResult::Show("FOOBAR".to_string())  )));
    }

    #[test]
    fn can_get_show_from_levelspec_show() {
        let show = parse_show("foobar");
        assert_eq!(show, Ok(("",MinimatchResult::Show("FOOBAR".to_string()))));
    }


    #[test]
    fn can_get_show_from_rel_levelspec() {
        let show = parse_show(".rd.bla");
        assert_eq!(show, Ok(("",MinimatchResult::EnvVar )));
    }
}


fn valid_showpath(input: &str) -> IResult<&str, &str> {
    delimited(
        tag("/"),
        variable,
        tag("/shows/")
    )(input)  
}

/// given an input string, produce a PathBuf
fn get_show_from_fullpath(input: &str) -> IResult<&str, MinimatchResult> {
    map(
    delimited(
        valid_showpath,
        variable,
        rest
    ),
    |item| {
        MinimatchResult::Show(item.to_string())
    }
    
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_show() {
        let show = get_show_from_fullpath("/dd/shows/FOOBAR");
        assert_eq!(show, Ok(("",MinimatchResult::Show("FOOBAR".to_string())  )));
    }
}

fn get_show_from_levelspec(input: &str) -> IResult<&str, MinimatchResult> {
    map(
        alt((
            terminated(variable, terminated(tag("."), rest)),
            variable
        )),
        |item| {
            MinimatchResult::Show(item.to_uppercase())
        }
    )(input)
}

#[cfg(test)]
mod minimatch_show_tests {
    use super::*;

    #[test]
    fn can_get_show() {
        let show = get_show_from_levelspec("foobar.rd.bla");
        assert_eq!(show, Ok(("",MinimatchResult::Show("FOOBAR".to_string())  )));
    }

    #[test]
    fn can_get_show_from_levelspec_show() {
        let show = get_show_from_levelspec("foobar");
        assert_eq!(show, Ok(("",MinimatchResult::Show("FOOBAR".to_string())  )));
    }
}


fn get_show_from_rel_levelspec(input: &str) -> IResult<&str, MinimatchResult> {
    map(
        terminated(tag("."), rest),
        |_| {
            MinimatchResult::EnvVar
        }
    )(input)
}


#[cfg(test)]
mod minimatch_rel_levelspec_tests {
    use super::*;

    #[test]
    fn can_get_show() {
        let show = get_show_from_rel_levelspec(".rd.bla");
        assert_eq!(show, Ok(("",MinimatchResult::EnvVar )));
    }

}

