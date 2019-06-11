use crate::{ JGraph, JSPError, NodePath, NodeType, NIndex, Search};
use std::{ cell::RefCell, rc::Rc, collections::VecDeque, path::PathBuf };
use log;
use petgraph::{visit::IntoNodeReferences};
use regex::Regex;
use lazy_static::lazy_static;

/// Given a Search reference and a JGraph reference, Find the PathBuf represented
/// by the search, or return an error if unsuccessful.
pub fn find_path(search: &Search, graph: &JGraph) -> Result<PathBuf, JSPError> {
    
    let keys = search.keys_owned();
    let nodepath = find(keys, graph)?;
    let mut values = search.values_owned();
    let mut path = PathBuf::new();

    for node in nodepath.iter() {
        match node.identity() {
            NodeType::RegEx{name, pattern, exclude} => {
                if let Some(ref value) = values.pop_front() {
                    if pattern.is_match(value) {
                        // check to see if we are supposed to be expluding as well
                        if let Some(exclude_re) = exclude {
                            if !exclude_re.is_match(value) {
                                path.push(value);
                            } else {
                                return Err(JSPError::Placeholder);
                            }
                        } else {
                            path.push(value);
                        } 
                    // reminder. capture_names()[0] is full regex. 1st actual capture
                    // starts on index 1. Hence why we are checking for a count of 2. 
                    // count() == 2 should yield only a single capture group
                    } else if pattern.capture_names().count() == 2 {
                        
                        let replacement = replace_capture_group(pattern.as_str(), name, value);
                        if replacement.is_some() {
                            path.push(replacement.unwrap());
                        } else {
                            log::error!("capture group does not match {}", name);
                            return Err(JSPError::Placeholder);
                        }
                    } else {
                        let cnt = pattern.capture_names().count(); 
                        if cnt < 2 {
                            log::error!("no capture groups");
                            return Err(JSPError::Placeholder);
                        } else {
                            log::error!("too many capture groups. we should have only 1 capture group");
                            return Err(JSPError::Placeholder);
                        }
                    } 
                }
            },
            NodeType::Simple(name) => {
                path.push(name);
            },
            NodeType::Root => {
                path.push("/");
            },
            _ => panic!("unexpected value")
        }
    }
    Ok(path)
}

// replace the named capture group with the name key, with a replacement
fn replace_capture_group(regstr: &str, key: &str, replacement: &str) -> Option<String> {
    //let re_reg =  r"\(\?P<([a-zA-Z_\-0-9]+)>.+\)";
    //let re_repl = r"\(\?P<[a-zA-Z_\-0-9]+>.+\)";
    lazy_static!{
        static ref RE: Regex = Regex::new(r"\(\?P<([a-zA-Z_\-0-9]+)>.+\)").unwrap();
    }
    let first = RE.captures(regstr);
    if first.is_some() && first.unwrap().get(1).unwrap().as_str() == key {
        log::debug!("replace_capture_group(...) match");
        lazy_static!{
        static ref RE2: Regex = regex::Regex::new(r"\(\?P<[a-zA-Z_\-0-9]+>.+\)").unwrap();
        static ref STRIP_REF: Regex = regex::Regex::new(r"[\^\$]+").unwrap();

        }
        let tmp = RE2.replace_all(regstr, "@@");
        let tmp = STRIP_REF.replace_all(tmp.as_ref(), "");
        Some(tmp.replace("@@", replacement).replace(r#"\"#,""))
    } else {
       None
    }
}
/// Find a NodePath given a vector of criteria Strings and a JGraph reference
///
/// # Parameters
/// * `criteria` - VecDeque of Strings representing a path through the graph of
///                RegEx Nodes. The nodes may be non-contiguous as long as they
///                are in order, and are padded by Simple Nodes.
/// * `graph` - JGraph reference which is expected to be populated by nodes and
///             NIndexes prior to calling find.
/// # Returns
///    A NodePath if successful. Otherwise, a JSPError
///
/// # Example
///
/// ```
/// use jsp::{graph::testdata::build_graph, find};
/// use std::collections::VecDeque;
///
/// let graph = build_graph();
/// let  search =  VecDeque::from(vec!["show".to_string(), "sequence".to_string(), "shot".to_string()]);
/// let result = find(search, &graph);
/// assert!(result.is_ok());
/// ```
pub fn find<'a>(criteria: VecDeque<String>, graph: &'a JGraph) -> Result<NodePath<'a>, JSPError> {
    let idx = graph.node_references().next().unwrap().0;
    let  np = vec![idx];
    let vec_nodes = Rc::new(RefCell::new(np));
    let criteria_rc = Rc::new(RefCell::new(criteria));
    match find_recurse(criteria_rc.clone(), vec_nodes, graph) {
        FindValue::Success(npath) => {
            let mut npath = Rc::try_unwrap(npath)
                          .unwrap()
                          .into_inner();
            let mut nodepath = NodePath::new(&graph);
            nodepath.append_unchecked(&mut npath);
            Ok(nodepath)
        },
        FindValue::Failure(_) => Err(JSPError::FindFailure(format!("{:?}", criteria_rc.borrow()))),
    }
}

// Internal recursive function
fn find_recurse<'a>(
    criteria: Rc<RefCell<VecDeque<String>>> ,
    nodepath: Rc<RefCell<Vec<NIndex>>>,
    graph: &'a JGraph,
) -> FindValue {
    log::info!("");
    log::info!("find_recurse({:?}, {:?})", criteria, nodepath.borrow());

    // explicit lifetime bounds for criteria_borrow. Should test to see
    // if NLL works here. ie is this needed?
    {
        let criteria_borrow = criteria.borrow();
        if criteria_borrow.len() == 0 {
            return FindValue::Success(nodepath);
        }
    }

    let front_criteria_elem;
    {
        let mut criteria_borrow = criteria.borrow_mut();
        front_criteria_elem = criteria_borrow.pop_front();
        log::info!("find_recurse(...) front_criteria_elem {:?}", front_criteria_elem);
    }

    match front_criteria_elem {
        Some(candidate_node_name) => {
            let last_node;
            {
                let np = nodepath.borrow();
                last_node = np[np.len()-1];
            }
            log::debug!("find_recurse(...) last node: {:?}. Iterating through last_node's children...", last_node);
            for nindex in graph.neighbors(last_node) {
                let node = &graph[nindex];
                log::debug!("find_recurse(...) for nindex in neighbors()... node: {:?}, nindex: {:?}", node, nindex);
                match node.identity() {
                    NodeType::RegEx{name, pattern:_, exclude: _} =>  {
                        log::debug!("find_recurse(...) NodeType::RegEx");
                        if name == &candidate_node_name {
                            log::debug!("find_recurse(...) {} == {}", name, &candidate_node_name);
                             {
                                 nodepath.borrow_mut().push(nindex);
                             }
                            let r = find_recurse(criteria.clone(), nodepath.clone(), graph);
                            if r.is_success() {
                                log::debug!("find_recurse(...) successful");
                                return FindValue::Success(nodepath);
                            } else {
                                nodepath.borrow_mut().pop();
                            }
                        } else {
                            log::debug!("find_recurse(...) {} != {}", name, &candidate_node_name);
                        }
                    },
                    NodeType::Simple(_) =>  {
                        log::debug!("NodeType::Simple");
                        // As this is a simple node, we will do two things Automatically:
                        // 1) Add the current nindex to the nodepath vector
                        // 2) Add the candidate_node_name to the front of the candidate VecQueue
                        // We do this because we accept the Simple nodetype without a means
                        // test, unlike NodeType::RegEx.
                        {
                            nodepath.borrow_mut().push(nindex);
                            criteria.borrow_mut().push_front(candidate_node_name.clone());
                        }

                        let r = find_recurse(criteria.clone(), nodepath.clone(), graph);
                        if r.is_success() {
                            log::debug!("find_recurse successful");
                            return FindValue::Success(nodepath);
                        } else {
                            log::debug!("find_recurse unsuccessful");
                            // If we did not find what we were looking for
                            // we pop off the last nodepath item, and we pop off the front
                            // of the criteria vector, both of which we added prior to calling
                            // the recursion
                            {
                                nodepath.borrow_mut().pop();
                                criteria.borrow_mut().pop_front();
                            }
                        }
                    },
                    NodeType::Root =>  {
                        // This should only be added to an empty nodepath
                        assert_eq!(nodepath.borrow().len(), 0);
                        return find_recurse(criteria.clone(), nodepath.clone(), graph);
                    },
                    // If we have gone into untracked territory, then we know we have failed
                    NodeType::Untracked => {return FindValue::Failure(nodepath);},
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

// FindValue is the return type for find_recurse
// I suppose that i could have used a Result, but It was not clear
// at the outset what would be needed.
#[derive(Debug, PartialEq, Eq)]
enum FindValue {
    Success(Rc<RefCell<Vec<NIndex>>>),
    Failure(Rc<RefCell<Vec<NIndex>>>)
}

impl FindValue {
    /// Return whether or not the instance of FindValue is Success
    fn is_success(&self) -> bool {
        match self {
            FindValue::Success(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{graph::testdata::build_graph, SearchTerm};
    use env_logger;
    use std::env;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn replace_capture_group_works() {
        let regexstr = r"^work\.(?P<user>[a-z]+)$";
        let key = "user";
        let replacement = "jgerber";
        let result = replace_capture_group(regexstr, key, replacement);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "work.jgerber".to_owned());
    }

    #[test]
    fn will_find_single_criterion() {
        env::set_var("RUST_LOG", "error");
        //env_logger::init();
        init();
        let graph = build_graph();
        let  search =  VecDeque::from(vec!["show".to_string()]);
        let result = find(search, &graph);
        assert!(result.is_ok());
    }

    #[test]
    fn will_find_multiple_criteria() {
        env::set_var("RUST_LOG", "error");
        //env_logger::init();
        init();
        let graph = build_graph();
        let  search =  VecDeque::from(vec!["show".to_string(), "sequence".to_string(), "shot".to_string()]);
        let result = find(search, &graph);
        assert!(result.is_ok());
    }

    #[test]
    fn will_not_succeed_when_given_unmatchable_criteria() {
        env::set_var("RUST_LOG", "error");
        //env_logger::init();
        init();
        let graph = build_graph();
        let  search =  VecDeque::from(vec!["bs".to_string(), "sequence".to_string(), "shot".to_string()]);
        let result = find(search, &graph);
        assert!(result.is_err());
    }


    #[test]
    fn will_find_multiple_path() {
        env::set_var("RUST_LOG", "warn");
        init();
        let graph = build_graph();
        let  mut search =  Search::new();//VecDeque::from(vec!["show".to_string(), "sequence".to_string(), "shot".to_string()]);
        search.push_back(SearchTerm::new("show", "DEV01"));
        search.push_back(SearchTerm::new("sequence", "RD"));

        let result = find_path(&search, &graph).unwrap();
        log::warn!("{:?}", result);
        assert_eq!(result, PathBuf::from("/dd/shows/DEV01/RD"));

    }
}