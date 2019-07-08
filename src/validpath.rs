use std::{fmt, path::{Path, PathBuf}};
use crate::{diskutils, NIndex, NodePath, JGraph, JSPError, validate_path, find_path, find_path_from_terms, SearchTerm, Search};
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
    /// * `subdir` - An optional subdirectory to tack on after searchterms provided that the nodepath doesnt have
    ///              any managed nodes underneath it. (ie that the graph doesnt enforce the shape)
    /// * `absolute` - Whether to convert the ValidPath into an absolute path or not. 
    /// 
    /// # Returns
    /// Ok wrapping ValidPath instance if Successful. 
    /// Otherwise, an Err wrapping JSPError
    pub fn new_from_searchterms(terms: Vec<SearchTerm>, graph: &'a JGraph, subdir: Option<&str>, absolute: bool) 
    -> Result<ValidPath<'a>, JSPError> {
        let (pathbuf, nodepath) = find_path_from_terms(terms, graph)?;
        let mut pathbuf = if absolute{diskutils::convert_relative_pathbuf_to_absolute(pathbuf)?} else {pathbuf};
        // if subdirectory has been defined
        if subdir.is_some() {
            // grab the last index from the nodepath (NIndex)
            let last_idx = nodepath.nindex();
            if last_idx.is_some() {
                // 
                let last_idx = last_idx.unwrap();
                let sz = graph.neighbors_directed(last_idx, petgraph::Direction::Outgoing).count();
                 let subdir = subdir.unwrap();
                if sz > 0 {
                    // This should be some error about trying to create subdir 
                    return Err(JSPError::MkdirFailure(format!("Cannot create '{}' under '{}'", subdir, pathbuf.display())));
                }
                pathbuf.push(subdir);
            } else {
                // wtf? this should be Some
                return Err(JSPError::JGraphError("index() method returned None. nodepath shoulndt be empty.".to_string()));
            }
        }
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

    pub fn pathbuf(&self) -> PathBuf {
        self.pathbuf.clone()
    }
    
    /// Return a Reference to a NodePath
    pub fn nodepath(&self) -> &NodePath<'a> {
        &self.nodepath
    }

    /// Pop off a ValidPath<'a> from the validpath
    pub fn pop(&mut self) -> Result<ValidPath<'a>, JSPError> {
        // pop off the last NIndex
        let idx = self.nodepath.pop().expect("no index to pop off");
        // get the last dirname of the path
        let dir = self.pathbuf.as_path().file_name().expect("no pathbuf to pop off").to_owned();
        // pop off the path (weird that i have to do this in two steps. pop()->bool for PathBuf)
        self.pathbuf.pop();
        let  graph: &'a JGraph = self.nodepath.graph();
        let mut nodepath = NodePath::new(graph);
        let mut idxvec = vec![idx];
        nodepath.append_unchecked(&mut idxvec);
        ValidPath::new_unchecked(PathBuf::from(dir), nodepath, false)
    }


    /// Givn an NIndex, drop every entyry past it
    pub fn remove_past(&mut self, index: NIndex) -> Result<(), JSPError> {
        let total_depth = self.nodepath().len();
        let mut cnt: i64 = -1;
        if let Some(depth) = self.nodepath().find_nindex_depth(index) {
            // technically could overflow but no chance that graph will reach
            // this depth. could make it an i8 for that matter
            cnt = (total_depth - depth - 1) as i64;
        }
        if cnt > 0 {
            log::info!("remove_past(...). dropping {} nodes", cnt);
            for _ in 0..cnt {
                log::debug!("remove_past() poping");
                let _ = self.pop()?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::testdata::build_graph;
    use env_logger;
    use std::env;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn can_remove_past() {
        env::set_var("RUST_LOG", "debug");
        init();
        let graph = build_graph();
        let path = PathBuf::from("/dd/shows/DEV01");
        let mut validpath = ValidPath::new(path, &graph, false).unwrap();
        let idx = NIndex::new(2);
        let result = validpath.remove_past(idx);

        assert_eq!(result, Ok(()) );
        let last = NIndex::new(2);
        assert_eq!(validpath.nodepath().nindex().unwrap(), last);
        
    }

}