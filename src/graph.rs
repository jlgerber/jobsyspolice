pub use crate::Node;
use petgraph::graph::DefaultIx;
use petgraph::visit::IntoNodeReferences;
//use petgraph::visit::{Bfs, IntoNeighbors};

/// Define a type alias for the type of graph we will be using.
/// JGraph is a Jobsystem Graph
pub type JGraph = petgraph::Graph<Node, f32>;

/// Determine if the provided path is valid
pub fn is_valid(path: &str, graph: &JGraph) -> bool {
    let mut it = std::path::Path::new(path).iter();
    // we have to drop the first item, which is the first "/"
    it.next();
    return _is_valid(it, &graph, graph.node_references().next().unwrap().0);
}

fn _is_valid(
    mut path: std::path::Iter,
    graph: &JGraph,
    parent: petgraph::graph::NodeIndex<DefaultIx>,
) -> bool {
    let component = path.next();
    dbg!(component);
    match component {
        Some(val) => {
            let mut cnt = 0;
            for n in graph.neighbors(parent) {
                let node = &graph[n];
                println!("testing {:?}", node);
                if node == val {
                    println!("match {:?}", node);
                    if _is_valid(path.clone(), graph, n) {
                        return true;
                    }
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
