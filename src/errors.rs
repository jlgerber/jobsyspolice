use failure::Fail;
use crate::NIndex;
use std::ffi::OsString;

#[derive(Debug, Fail)]
pub enum JSPError {
    #[fail(display = "missing NIndex: {:?}", _0)]
    MissingIndex (NIndex),
    #[fail(display = "Validation Failure: {:?}, index: {:?} depth: {}", entry, node, depth)]
    ValidationFailure{ entry: OsString, node: NIndex, depth: u8 },
    #[fail(display = "Placeholder error")]
    Placeholder,
}