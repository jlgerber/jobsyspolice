use std::{fmt, path::{Path, PathBuf}};
use crate::{NodePath, JGraph, JSPError, validate_path};
use std::fmt::Debug;

/// A ValidPath provides a path that has been validated againt the template. 
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ValidPath<'a> {
    pathbuf: PathBuf,
    nodepath: NodePath<'a>
}

/// When using Display, we simply display the pathbuf portion
impl<'a> fmt::Display for ValidPath<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pathbuf.fmt(f)
    }
}

impl<'a> ValidPath<'a> {

    /// Construct a ValidPath from a pathbuf
    pub fn new<I: Into<PathBuf>>(pathbuf: I, graph: &'a JGraph) -> Result<ValidPath<'a>, JSPError> {

        let pathbuf = pathbuf.into();
        let nodepath = validate_path(&pathbuf, graph)?;

        Ok(ValidPath {
            pathbuf, 
            nodepath
        })
    }

    /// Take responsibility for validation. Take one PathBuf, add one NodePath, and voila
    pub fn new_unchecked<I: Into<PathBuf>>(pathbuf: I, nodepath: NodePath<'a>) -> ValidPath<'a> {
        let pathbuf = pathbuf.into();
       
        ValidPath {
            pathbuf, 
            nodepath
        }
    }

    /// Return a reference to a Path
    pub fn path(&self) -> &Path {
        self.pathbuf.as_path()
    }

    /// Return a Reference to a NodePath
    pub fn nodepath(&self) -> &NodePath<'a> {
        &self.nodepath
    }
}