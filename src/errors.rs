use failure::Fail;
use crate::NIndex;

#[derive(Debug, Fail)]
pub enum JSPError {
    #[fail(display = "missing NIndex: {:?}", _0)]
    MissingIndex (NIndex),
}