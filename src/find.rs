use crate::{ JGraph, JSPError, NodePath, NodeType, NIndex,  Node};
use std::{ cell::RefCell, rc::Rc, path::Path };

#[derive(Debug, PartialEq, Eq)]
pub enum FindValue<'a> {
    Success(NodePath<'a>),
    Failure(NodePath<'a>)
}
impl<'a> FindValue<'a> {
    /// Return whether or not the instance of FindValue is Success
    pub fn is_success(&self) -> bool {
        match self {
            FindValue::Success(_) => true,
            _ => false,
        }
    }
}
/// Find a NodePath given a vector of criteria
pub fn find<'a>(mut criteria: Vec<String>, graph: &'a JGraph) -> Result<NodePath<'a>, JSPError> {
    let mut nodepath = NodePath::new(graph);
    match find_recurse(criteria.iter_mut(), nodepath) {
        FindValue::Success(npath) => Ok(npath),
        FindValue::Failure(_) => Err(JSPError::FindFailure(format!("{:?}", criteria))),
    }
}

fn find_recurse<'a>(
    mut criteria: std::slice::IterMut<String>,
    nodepath: NodePath<'a>,
) -> FindValue<'a> {
    let component = criteria.next();
    match component {
        Some(val) => {
            let mut cnt = 0;
            for nindex in nodepath.neighbors() {
                let node = nodepath.node_for(nindex);
                cnt += 1;
                match *node.identity() {
                    NodeType::RegEx{name, pattern:_, exclude: _} =>  {
                        if name == *val {
                            nodepath.push(nindex);
                            let r = find_recurse(criteria, nodepath);
                            if r.is_success() {
                                return FindValue::Success(nodepath);
                            } else {
                                nodepath.pop();
                            }
                        }
                    },
                    NodeType::Simple(name) =>  {
                        nodepath.push(nindex);
                        let r = find_recurse(criteria, nodepath);
                        if r.is_success() {
                            return FindValue::Success(nodepath);
                        } else {
                            nodepath.pop();
                        }
                    },
                    NodeType::Root =>  {
                        assert_eq!(nodepath.len(), 0);
                        return find_recurse(criteria, nodepath);
                    },
                    NodeType::Untracked => panic!("should never hit an untracked node in find"),
                }
            }
            // made it through all of the children without returning
            // a successful match, so we must be in a failure state.
            return FindValue::Failure(nodepath);
        }
        None => {
            return FindValue::Failure(nodepath);
        }
    }
}
