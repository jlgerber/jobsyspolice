pub use crate::Node;
use petgraph::graph::DefaultIx;
use petgraph::visit::IntoNodeReferences;
#[allow(unused_imports)]
use log::{debug, trace};
//use petgraph::visit::{Bfs, IntoNeighbors};

/// Define a type alias for the type of graph we will be using.
/// JGraph is a Jobsystem Graph
pub type JGraph = petgraph::Graph<Node, ()>;

/// Determine if the provided path is valid or not.NodeType
///
/// # Parameters
/// * `path` - a str reference representing a candidate path
/// * `graph` - a reference to a JGrapch, which is the graph
///             representing the valid paths within the Jobsystem.
///
/// # Returns
///     `bool` indicating whether or not `path` is valid based on
/// the schema described by the input `graph`.
pub fn is_valid(path: &str, graph: &JGraph) -> bool {
    let mut it = std::path::Path::new(path).iter();
    // we have to drop the first item, which is the first "/"
    it.next();
    return _is_valid(it, &graph, graph.node_references().next().unwrap().0);
}

// helper recursive function
fn _is_valid(
    mut path: std::path::Iter,
    graph: &JGraph,
    parent: petgraph::graph::NodeIndex<DefaultIx>,
) -> bool {
    let component = path.next();
    match component {
        Some(val) => {
            let mut cnt = 0;
            for n in graph.neighbors(parent) {
                let node = &graph[n];
                trace!("testing {:?} against {:?}", val, node);
                if node == val {
                    trace!("MATCH");
                    if _is_valid(path.clone(), graph, n) {
                        return true;
                    }
                } else {
                    trace!("NO MATCH");
                }
                cnt += 1;
            }
            // cannot find a way to get number of children for node any other way.
            // we assume that if we have made it this far, and there are no children,
            // we are successful. This allows the path to extend beyond the graph.
            if cnt == 0 {
                return true;
            }
        }
        None => {
            return true;
        }
    }
    false
}

pub mod testdata {
    use std::str::FromStr;
    use crate::{ JGraph, Node, NodeType, EntryType, Regexp };

    pub fn build_graph() -> JGraph {
        let mut graph = JGraph::new();

        let root = graph.add_node(Node::new_root());
        let dd = Node::new(NodeType::Simple("dd".to_owned()), EntryType::Directory);

        let shows = Node::from_str("shows").unwrap();

        let dd = graph.add_node(dd);
        let shows = graph.add_node(shows);

        let show = graph.add_node(Node::new(
            NodeType::Regexp {
                name: "show".to_owned(),
                pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
            },
            EntryType::Directory,
        ));

        let tools = graph.add_node(Node::from_str("tools").unwrap());
        let package = graph.add_node(Node::from_str("package").unwrap());
        let extension = graph.add_node(Node::from_str("extension").unwrap());
        let bin = graph.add_node(Node::from_str("bin").unwrap());
        let etc = graph.add_node(Node::from_str("etc").unwrap());
        let user = graph.add_node(Node::from_str("user").unwrap());
        let shared = graph.add_node(Node::from_str("SHARED").unwrap());
        let previz = graph.add_node(Node::from_str("PREVIZ").unwrap());
        let integ = graph.add_node(Node::from_str("INTEG").unwrap());
        let model = graph.add_node(Node::from_str("MODEL").unwrap());
        let rig = graph.add_node(Node::from_str("RIG").unwrap());
        let anim = graph.add_node(Node::from_str("ANIM").unwrap());
        let cfx = graph.add_node(Node::from_str("CFX").unwrap());
        let light = graph.add_node(Node::from_str("LIGHT").unwrap());
        let enviro = graph.add_node(Node::from_str("ENVIRO").unwrap());
        let fx = graph.add_node(Node::from_str("FX").unwrap());
        let comp = graph.add_node(Node::from_str("COMP").unwrap());
        let img = graph.add_node(Node::from_str("IMG").unwrap());

        let work = graph.add_node(Node::new(
            NodeType::Regexp {
                name: "work".to_string(),
                pattern: Regexp::new(r"^work\.[a-z]+$").unwrap(),
            },
            EntryType::Directory,
        ));

        let sequence = graph.add_node(Node::new(
            NodeType::Regexp {
                name: "sequence".to_string(),
                pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
            },
            EntryType::Directory,
        ));

        let shot = graph.add_node(Node::new(
            NodeType::Regexp {
                name: "shot".to_string(),
                pattern: Regexp::new(r"^[0-9]+[A-Z 0-9]*$").unwrap(),
            },
            EntryType::Directory,
        ));

        graph.extend_with_edges(&[
            (root, dd),
            (dd, shows),
            (shows, show),
            (show, tools),
            (tools, package),
            (tools, extension),
            (tools, bin),
            (show, etc),
            (show, user),
            (user, work),
            (show, shared),
            (shared, etc),
            (shared, previz),
            (shared, integ),
            (shared, model),
            (shared, rig),
            (shared, anim),
            (shared, cfx),
            (shared, fx),
            (shared, light),
            (shared, enviro),
            (shared, comp),
            (shared, img),
            (show, sequence),
        ]);
        // split it up because there appears to be
        // a max size for &[]
        graph.extend_with_edges(&[
            (sequence, tools),
            (sequence, etc),
            (sequence, shared),
            (sequence, user),
            (sequence, shot),
        ]);

        graph.extend_with_edges(&[
            (shot, tools),
            (shot, etc),
            (shot, shared),
            (shot, user),
        ]);

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::testdata::build_graph;

    #[test]
    fn path_extends_beyond_graph() {
        let tgraph = build_graph();
        let p = "/dd/shows/DEV01/SHARED/MODEL/foo/bar";
        assert!(is_valid(p, &tgraph));
    }

    #[test]
    fn shot_is_valid_graph() {
        let tgraph = build_graph();
        let p = "/dd/shows/DEV01/RD/9999/SHARED/MODEL";
        assert!(is_valid(p, &tgraph));
    }


    #[test]
    fn wrong_path_is_invalid_graph() {
        let tgraph = build_graph();
        let p = "/dd/shows/DEV01/RD/9999/FOO/SHARED/MODEL";
        assert_eq!(is_valid(p, &tgraph), false);
    }
}
