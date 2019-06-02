use failure::Fail;
use crate::NIndex;
use std::ffi::OsString;
use std::io;
use std::num;

#[derive(Debug, Fail)]
pub enum JSPError {
    #[fail(display = "missing NIndex: {:?}", _0)]
    MissingIndex (NIndex),
    #[fail(display = "Validation Failure: {:?}, index: {:?} depth: {}", entry, node, depth)]
    ValidationFailure{ entry: OsString, node: NIndex, depth: u8 },
    #[fail(display = "Placeholder error")]
    Placeholder,
    #[fail(display = "{}", _0)]
    IoError(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    ParseIntError(#[cause] num::ParseIntError),
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
