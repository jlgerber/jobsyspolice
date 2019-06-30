use std::{fmt, path::{Path, PathBuf}};
use crate::{diskutils, NodePath, JGraph, JSPError, validate_path, find_path, find_path_from_terms, SearchTerm, Search};
use std::fmt::Debug;

/// A ValidPath provides a pairing of a PathBuf and a NodePath, representing a path that 
/// has been validated against the JGraph. 
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

    /// New up a ValidPath from a PathBuf. 
    /// 
    /// # Parameters
    /// 
    /// * `pathbuf` - A `PathBuf` instance
    /// * `graph` - Reference to the JGraph instance 
    /// * `absolute` - Whether to convert the ValidPath into an absolute path or not. 
    /// 
    /// # Returns
    /// Ok wrapping ValidPath instance if Successful. 
    /// Otherwise, an Err wrapping JSPError
    pub fn new<I: Into<PathBuf>>(pathbuf: I, graph: &'a JGraph, absolute: bool) -> Result<ValidPath<'a>, JSPError> {

        let pathbuf = if absolute{diskutils::convert_relative_pathbuf_to_absolute(pathbuf.into())?} else {pathbuf.into()};
        let nodepath = validate_path(&pathbuf, graph)?;

        Ok(ValidPath {
            pathbuf, 
            nodepath
        })
    }

    /// New up a ValidPath from components. This method is dangerous and should only be used if one knows that the 
    /// validation has taken place.  
    /// Take one PathBuf, add one NodePath, and voila
    /// 
    /// # Parameters
    /// 
    /// * `pathbuf` - a type that is `Into<PathBuf>`
    /// * `nodepath` - A NodePath instance
    /// * `absolute` - Whether to convert the ValidPath into an absolute path or not. 
    /// 
    /// # Returns
    /// Ok wrapping ValidPath instance if Successful. 
    /// Otherwise, an Err wrapping JSPError
    pub fn new_unchecked<I: Into<PathBuf>>(pathbuf: I, nodepath: NodePath<'a>,  absolute: bool) -> Result<ValidPath<'a>, JSPError> {
        let pathbuf = if absolute{diskutils::convert_relative_pathbuf_to_absolute(pathbuf.into())?} else {pathbuf.into()};
       Ok(
            ValidPath {
                pathbuf, 
                nodepath
            }
       )
    }

    /// New up a ValidPath from a vector of SearchTerm
    /// 
    /// # Parameters
    /// 
    /// * `terms` - A vector of `SearchTearm`s, representing an ordered (though sparse) traversal of the JGraph
    /// * `graph` - Reference to the JGraph instance 
    /// * `absolute` - Whether to convert the ValidPath into an absolute path or not. 
    /// 
    /// # Returns
    /// Ok wrapping ValidPath instance if Successful. 
    /// Otherwise, an Err wrapping JSPError
    pub fn new_from_searchterms(terms: Vec<SearchTerm>, graph: &'a JGraph, absolute: bool) -> Result<ValidPath<'a>, JSPError> {
        let (pathbuf, nodepath) = find_path_from_terms(terms, graph)?;
        let pathbuf = if absolute{diskutils::convert_relative_pathbuf_to_absolute(pathbuf)?} else {pathbuf};

        Ok(ValidPath {
            pathbuf, 
            nodepath
        })
    }
    
    /// New up a ValidPath from a Search reference
    /// 
    /// # Parameters
    /// 
    /// * `search` - Reference to `Search`, containing a list of `SearchTerm`s 
    /// * `graph` - Reference to the JGraph instance 
    /// * `absolute` - Whether to convert the ValidPath into an absolute path or not. 
    ///  
    /// # Returns
    /// Ok wrapping ValidPath instance if Successful. 
    /// Otherwise, an Err wrapping JSPError
    pub fn new_from_search(search: &Search, graph: &'a JGraph, absolute: bool) -> Result<ValidPath<'a>, JSPError> {
        let (pathbuf, nodepath) = find_path(search, graph)?;
        let pathbuf = if absolute{diskutils::convert_relative_pathbuf_to_absolute(pathbuf)?} else {pathbuf};

        Ok(ValidPath {
            pathbuf, 
            nodepath
        })
    }

    // pub fn absolute(mut self) -> Result<Self, JSPError> {
    //     if let mut ValidPath{pathbuf, nodepath} = self {

    //         let pathbuf = diskutils::convert_relative_pathbuf_to_absolute(pathbuf)?;
    //         Ok(
    //             PathBuf{
    //                 pathbuf,
    //                 nodepath
    //             }
    //         )
    //     } else {
    //         Err(JSPError::Placeholder)
    //     }
    // }

    /// Return a reference to a Path
    pub fn path(&self) -> &Path {
        self.pathbuf.as_path()
    }

    /// Return a Reference to a NodePath
    pub fn nodepath(&self) -> &NodePath<'a> {
        &self.nodepath
    }
}