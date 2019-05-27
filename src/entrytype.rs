use std::fmt::{Display, Formatter, self};
use serde::{Deserialize,Serialize};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Deserialize, Serialize)]
pub enum EntryType {
    Directory,
    Volume,
    Root,
}

impl Display for EntryType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            EntryType::Directory =>  write!(f, "Directory"),
            EntryType::Volume =>  write!(f, "Volume"),
            EntryType::Root =>  write!(f, "Root"),

        }
    }
}