use crate::NIndex;
use failure::Fail;
use nix;
use std::{ffi::OsString, io, num, path::PathBuf };
use regex;

#[derive(Debug, Fail)]
pub enum JSPError {
    #[fail(display = "missing NIndex: {:?}", _0)]
    MissingIndex (NIndex),

    #[fail(display = "Uid Lookup failed for '{}'", _0)]
    UidLookupFailed (String),

    #[fail(display = "Unable to Chown '{}'", _0)]
    ChownFailure (String),

    #[fail(display = "Unable to find '{}'", _0)]
    FindFailure (String),

    #[fail(display = "Unable to make directory: '{}'", _0)]
    MkdirFailure(String),

    #[fail(display = "Validation Failure: {:?}, index: {:?} depth: {}", entry, node, depth)]
    ValidationFailure{ entry: OsString, node: NIndex, depth: u8 },

    #[fail(display = "Placeholder error")]
    Placeholder,

    #[fail(display = "Unable to get filename from path: {:?}", _0)]
    FilenameFromPathFailed(PathBuf),

    #[fail(display = "Unable to convert PathBuf to str: {:?}", _0)]
    PathBufConvertToStrFailed(PathBuf),

    #[fail(display = "Invalid User Name: {}", _0)]
    InvalidUserName(String),

    #[fail(display = "Missing Owner in regex")]
    MissingOwnerInRegex,

     #[fail(display = "Boxed Error '{}'", _0)]
    BoxedError(String),

    #[fail(display = "{}", _0)]
    IoError(#[cause] io::Error),

    #[fail(display = "{}", _0)]
    ParseIntError(#[cause] num::ParseIntError),

    #[fail(display = "{}", _0)]
    NixError(#[cause] nix::Error),

    #[fail(display = "{}", _0)]
    RegexError(#[cause] regex::Error),
}


impl From<io::Error> for JSPError {
    fn from(error: io::Error) -> Self {
        JSPError::IoError(error)
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

impl From<regex::Error> for JSPError {
    fn from(error: regex::Error) -> Self {
        JSPError::RegexError(error)
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for JSPError {
    fn from(error: std::boxed::Box<dyn std::error::Error> ) -> Self {
        JSPError::BoxedError(error.to_string())
    }
}
