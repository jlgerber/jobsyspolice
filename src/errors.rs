use crate::{NIndex, NodeType, jspt};
use failure::Fail;
use nix;
use std::{ffi::OsString, io, num, path::PathBuf };
use ext_regex;
use levelspecter;

/// Error enum implementing Fail trait
#[derive(Debug, Fail, PartialEq, Clone)]
pub enum JSPError {
    #[fail(display = "missing NIndex: {:?}", _0)]
    MissingIndex (NIndex),

    #[fail(display = "Uid Lookup failed for '{}'", _0)]
    UidLookupFailed (String),

    #[fail(display = "Unable to Chown '{}'", _0)]
    ChownFailure (String),

    #[fail(display = "FindFailure. Encountered issue searching Jobsystem Template: '{}'", _0)]
    FindFailure (String),

    #[fail(display = "Path does not exist: '{:?}'", _0)]
    NonExtantPathError (PathBuf),

    #[fail(display = "Failed to convert NodeType: {:?} to PathBuf ", _0)]
    NodePathConversionFailure (NodeType),

    #[fail(display = "Unknown Shell: '{}'", _0)]
    UnknownShell (String),

    #[fail(display = "SearchTerm Error '{}'", _0)]
    SearchTermError(String),

    #[fail(display = "Unable to make directory: '{}'", _0)]
    MkdirFailure(String),

    #[fail(display = "Validation Failure: {:?}, index: {:?} depth: {}", entry, node, depth)]
    ValidationFailure{ entry: OsString, node: NIndex, depth: u8 },
    
    #[fail(display = "Validation Failure for {:?} : {:?}, index: {:?} depth: {}", path, entry, node, depth)]
    ValidationFailureFor{ path: PathBuf, entry: OsString, node: NIndex, depth: u8 },
    
    #[fail(display = "Validation Failure of {:?}: entry:{:?}, index: {:?} depth: {}", path, entry, node, depth)]
    ValidationFailureAt{ path: OsString , entry: OsString, node: NIndex, depth: u8},

    #[fail(display = "Placeholder error")]
    Placeholder,

    #[fail(display = "Unable to get filename from path: {:?}", _0)]
    FilenameFromPathFailed(PathBuf),

    #[fail(display = "Unable to convert PathBuf to str: {:?}", _0)]
    PathBufConvertToStrFailed(PathBuf),

    #[fail(display = "Invalid User Name: {}", _0)]
    InvalidUserName(String),

    #[fail(display = "JGraph Error: {}", _0)]
    JGraphError(String),

    #[fail(display = "Missing Owner in regex")]
    MissingOwnerInRegex,

    #[fail(display = "No group exists with the provided group name: {}", _0)]
    NoGroupForName(String),

    #[fail(display = "Boxed Error '{}'", _0)]
    BoxedError(String),

    #[fail(display = "{}", _0)]
    JSPTemplateError(#[cause] jspt::JSPTemplateError),

    //#[fail(display = "{}", _0)]
    //IoError(#[cause] io::Error),
    #[fail(display = "io::Error {}", _0)]
    IoError(String),

    #[fail(display = "{}", _0)]
    ParseIntError(#[cause] num::ParseIntError),

    #[fail(display = "{}", _0)]
    NixError(#[cause] nix::Error),

    #[fail(display = "{}", _0)]
    RegexError(#[cause] ext_regex::Error),
    
    #[fail(display = "{}", _0)]
    LevelSpecError(#[cause] levelspecter::LevelSpecterError),

    #[fail(display = "Empty argument list")]
    EmptyArgumentListError,

    #[fail(display = "Uid Retrieval Error: {}", _0)]
    UidRetrievalError(String),

    #[fail(display = "Issue with template: '{}'", _0)]
    TemplateError(String),

    #[fail(display = "{}", _0)]
    VarError(#[cause] std::env::VarError),

    #[fail(display = "JSPError '{}'", _0)]
    GeneralError(String),
}

impl From<std::env::VarError> for JSPError {
    fn from(error: std::env::VarError) -> Self {
        JSPError::VarError(error)
    }
}

impl From<jspt::JSPTemplateError> for JSPError {
    fn from(error: jspt::JSPTemplateError) -> Self {
        JSPError::JSPTemplateError(error)
    }
}

impl From<io::Error> for JSPError {
    fn from(error: io::Error) -> Self {
        JSPError::IoError(error.to_string())
    }
}

impl From<num::ParseIntError> for JSPError {
    fn from(error: num::ParseIntError) -> Self {
        JSPError::ParseIntError(error)
    }
}

impl From<nix::Error> for JSPError {
    fn from(error: nix::Error) -> Self {
        JSPError::NixError(error)
    }
}

impl From<ext_regex::Error> for JSPError {
    fn from(error: ext_regex::Error) -> Self {
        JSPError::RegexError(error)
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for JSPError {
    fn from(error: std::boxed::Box<dyn std::error::Error> ) -> Self {
        JSPError::BoxedError(error.to_string())
    }
}

// impl From<levelspec::LSpecError> for JSPError {
//     fn from(error: levelspec::LSpecError) -> Self {
//         JSPError::LevelSpecError(error)
//     }
// }


impl From<levelspecter::LevelSpecterError> for JSPError {
    fn from(error: levelspecter::LevelSpecterError) -> Self {
        JSPError::LevelSpecError(error)
    }
}