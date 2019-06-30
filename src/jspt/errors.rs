use failure::Fail;
use crate::jspt::State;
use nom;
use std::{io, path::PathBuf};
use ext_regex;

/// The Fail implementation for this crate. All functions that return 
/// results should return a JSPTemplateError for the error branch.
#[derive(Debug, Fail, PartialEq, Clone)]
pub enum JSPTemplateError {
    #[fail(display = "Environment Variable Lookup Error  :{}", _0)]
    EnvVarLookupError(String),
    /// When the StateMachine attempts a transition between two non-compatible
    /// states.
    #[fail(display = "Invalid State Transition from: {:?} to {:?}", _0, _1)]
    InvalidStateTransition(State, State),
    /// When a terminal state is asked to transition to the next state, which
    /// by definition does not exist. We hope never to encounter this error 
    /// in practice
    #[fail(display = "No valid next state for: {:?}", _0)]
    NoValidNextState(State),
    /// A development placeholder which should not find its way into production code
    #[fail(display = "Placeholder")]
    Placeholder,
    /// Used internally in StateMachine.next to signify an erroneous execution request in 
    /// this state
    #[fail(display = "DoneState")]
    DoneState,
    /// Used internally in StateMachine.next to signify an erroneous execution request in 
    /// this state
    #[fail(display = "ErrorState")]
    ErrorState,
    /// Failure to parse rpst
    #[fail(display = "ParsingError: {}",_0)]
    ParsingError(String),
    /// Error originating in the Nom crate
    #[fail(display = "NomError: {:?}", _0)]
    NomError(String),
    /// Wrapper around another JSPTemplateError which adds execution context (line number, line, current state)
    #[fail(display = "ErrorAtLine: {}, Line: {}, State: {}, Error: {:?}", _0, _1, _2, _3)]
    ErrorAtLine(usize, String, State, Box<JSPTemplateError>),
    /// Error originating in the io crate
    //#[fail(display = "{}", _0)]
    //IoError(#[cause] io::Error),
    #[fail(display = "IO::Error {}", _0)]
    IoError(String),
    /// Error originating in the Regex crate
    #[fail(display = "{}", _0)]
    RegexError(#[cause] ext_regex::Error),
    /// Error looking up a key in the RegexMap
    #[fail(display = "Regex Map Lookup failed for: {}", _0)]
    RegexMapLookupError(String),
    /// Error looking up a key in the KeyMap
    #[fail(display = "key Map Lookup failed for: {}", _0)]
    KeyMapLookupError(String),
    /// Error trying to access a Non extant or Inaccessible file
    #[fail(display = "File: {:?} does not exist or we lack permissions to access it", _0)]
    InaccesibleFileError(PathBuf),
}

// Implement From Nom Error
impl<'a> From<nom::Err<(&'a str, nom::error::ErrorKind)>> for JSPTemplateError {
    fn from(error: nom::Err<(&'a str,nom::error::ErrorKind)> ) -> Self {
        JSPTemplateError::NomError(format!("{:?}", error))
    }
} 

// Implement From IO Error
impl From<io::Error> for JSPTemplateError {
    fn from(error: io::Error) -> Self {
        JSPTemplateError::IoError(error.to_string())
    }
}

// Implement From JSPTemplateLineError
impl From<JSPTemplateLineError> for JSPTemplateError {
    fn from(error: JSPTemplateLineError) -> Self {
        let JSPTemplateLineError::ErrorAtLine(line_num, line, state, err) = error;
        JSPTemplateError::ErrorAtLine(line_num, line, state, Box::new(err))
    }
}

// Implement From Regex crate Error
impl From<ext_regex::Error> for JSPTemplateError {
    fn from(error: ext_regex::Error) -> Self {
        JSPTemplateError::RegexError(error)
    }
}

//-------------------------------//
//     JSPTEMPLATELINEERROR      //
//-------------------------------//
/// Wrap JSPTemplateError to provide a line number associated with each error
#[derive(Debug, Fail, PartialEq, Clone)]
pub enum JSPTemplateLineError {
    #[fail(display = "Error at line: {} line: {} State: {} Error: {:?}", _0, _1, _2, _3)]
    ErrorAtLine(usize, String, State, JSPTemplateError)
}

/// Convert from a JSPTemplateError to a JSPTemplateLineError by 
/// providing a tuple of ( line number, error ).
impl From<(usize, String, State, JSPTemplateError)> for JSPTemplateLineError {
    fn from(error: (usize, String, State, JSPTemplateError) ) -> Self {
        JSPTemplateLineError::ErrorAtLine(error.0, error.1, error.2, error.3)
    }
} 

