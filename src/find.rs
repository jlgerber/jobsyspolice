use crate::{ JGraph, JSPError, NodePath, NodeType, NIndex};
use std::{ cell::RefCell, rc::Rc, collections::VecDeque };
use log;
use petgraph::{visit::IntoNodeReferences};

#[derive(Debug, PartialEq, Eq)]
pub enum FindValue {
    Success(Rc<RefCell<Vec<NIndex>>>),
    Failure(Rc<RefCell<Vec<NIndex>>>)
}

impl FindValue {
    /// Return whether or not the instance of FindValue is Success
    pub fn is_success(&self) -> bool {
        match self {
            FindValue::Success(_) => true,
            _ => false,
        }
    }
}
/// Find a NodePath given a vector of criteria
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

fn find_recurse<'a>(
    criteria: Rc<RefCell<VecDeque<String>>> ,
    nodepath: Rc<RefCell<Vec<NIndex>>>,
    graph: &'a JGraph,
) -> FindValue {
    log::info!("");
    log::info!("find_recurse({:?}, {:?})", criteria, nodepath.borrow());

    {
        let criteria_borrow = criteria.borrow();
        if criteria_borrow.len() == 0 {
            return FindValue::Success(nodepath.clone());
        }
    }
    let component;
    {
        let mut criteria_borrow = criteria.borrow_mut();
        component = criteria_borrow.pop_front();
        log::info!("find_recurse component {:?}", component);
    }
    match component {
        Some(val) => {
            let last_node;
            {
                let np = nodepath.borrow();
                last_node = np[np.len()-1];
            }

            log::debug!("last node: {:?}", last_node);
            for nindex in graph.neighbors(last_node) {
            //for nindex in neighbors {

                let node = &graph[nindex];

                log::debug!("for nindex in neighbors()... node: {:?}, nindex: {:?}", node, nindex);
                //cnt += 1;
                match node.identity() {
                    NodeType::RegEx{name, pattern:_, exclude: _} =>  {
                        log::debug!("match node.indenty() RegEx");
                        if name == &val {
                            log::debug!("{} == {}", name, &val);
                             {
                                 nodepath.borrow_mut().push(nindex);
                             }
                            let r = find_recurse(criteria.clone(), nodepath.clone(), graph);
                            if r.is_success() {
                                return FindValue::Success(nodepath.clone());
                            } else {
                                nodepath.borrow_mut().pop();
                            }
                        } else {
                            log::debug!("{} != {}", name, &val);
                        }
                    },
                    NodeType::Simple(_) =>  {
                        log::debug!("NodeType::Simple");
                        {
                            nodepath.borrow_mut().push(nindex);
                            // we were not able to find
                            criteria.borrow_mut().push_front(val.clone());
                        }
                        let r = find_recurse(criteria.clone(), nodepath.clone(), graph);
                        if r.is_success() {
                            log::debug!("find_recurse successful");
                            return FindValue::Success(nodepath.clone());
                        } else {
                            log::debug!("find_recurse unsuccessful");
                            {
                                nodepath.borrow_mut().pop();
                                criteria.borrow_mut().pop_front();
                            }
                        }
                    },
                    NodeType::Root =>  {
                        //assert_eq!(nodepath.borrow().len(), 0);
                        return find_recurse(criteria.clone(), nodepath.clone(), graph);
                    },
                    NodeType::Untracked => panic!("should never hit an untracked node in find"),
                }
            }
            // made it through all of the children without returning
            // a successful match, so we must be in a failure state.
            return FindValue::Failure(nodepath.clone());
        }
        None => {
            return FindValue::Failure(nodepath.clone());
        }
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
    fn can_find_single() {
        env::set_var("RUST_LOG", "error");
        //env_logger::init();
        init();
        let graph = build_graph();
        let  search =  VecDeque::from(vec!["show".to_string()]);
        let result = find(search, &graph);
        assert!(result.is_ok());
    }

    #[test]
    fn can_find_multi() {
        env::set_var("RUST_LOG", "error");
        env_logger::init();
        let graph = build_graph();
        let  search =  VecDeque::from(vec!["show".to_string(), "sequence".to_string(), "shot".to_string()]);
        let result = find(search, &graph);
        assert!(result.is_ok());
    }
}