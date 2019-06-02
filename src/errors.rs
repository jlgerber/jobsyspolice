use failure::Fail;
use crate::NIndex;
use std::ffi::OsString;
use std::io;

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
}


impl From<io::Error> for JSPError {
  fn from(error: io::Error) -> Self {
        JSPError::IoError(error)
    }
}
