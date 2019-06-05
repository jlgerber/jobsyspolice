use crate::NIndex;
use failure::Fail;
use nix;
use std::{ffi::OsString, io, num };

#[derive(Debug, Fail)]
pub enum JSPError {
    #[fail(display = "missing NIndex: {:?}", _0)]
    MissingIndex (NIndex),
    #[fail(display = "Validation Failure: {:?}, index: {:?} depth: {}", entry, node, depth)]
    ValidationFailure{ entry: OsString, node: NIndex, depth: u8 },
    #[fail(display = "Placeholder error")]
    Placeholder,
    #[fail(display = "Invalid User Name '{}'", _0)]
    InvalidUserName(String),
     #[fail(display = "Boxed Error '{}'", _0)]
    BoxedError(String),
    #[fail(display = "{}", _0)]
    IoError(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    ParseIntError(#[cause] num::ParseIntError),
    #[fail(display = "{}", _0)]
    NixError(#[cause] nix::Error),
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

impl From<std::boxed::Box<dyn std::error::Error>> for JSPError {
    fn from(error: std::boxed::Box<dyn std::error::Error> ) -> Self {
        JSPError::BoxedError(error.to_string())
    }
}
