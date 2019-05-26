use jstemplate2::*;
use std::str::FromStr;
use petgraph;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::{Bfs, IntoNeighbors};
use petgraph::graph::DefaultIx;
use std::path::Path;

fn build_graph() -> petgraph::Graph<Node, f32> {
let mut graph = petgraph::Graph::<Node, f32>::new();

    let root = graph.add_node(Node::from_str("ROOT").unwrap());
    let dd = Node::new(Valid::Name("dd".to_owned()), NodeType::Directory);

    let shows = Node::from_str("shows").unwrap();

    let dd = graph.add_node(dd);
    let shows = graph.add_node(shows);

    let show = graph.add_node(
        Node::new(Valid::Regexp{ name: "show".to_owned(), pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap()},
        NodeType::Directory)
    );

    let etc    = graph.add_node(Node::from_str("etc").unwrap());
    let user   = graph.add_node(Node::from_str("user").unwrap());
    let shared = graph.add_node(Node::from_str("SHARED").unwrap());

    let previz = graph.add_node(Node::from_str("PREVIZ").unwrap());
    let integ  = graph.add_node(Node::from_str("INTEG").unwrap());
    let model  = graph.add_node(Node::from_str("MODEL").unwrap());
    let rig    = graph.add_node(Node::from_str("RIG").unwrap());
    let anim   = graph.add_node(Node::from_str("ANIM").unwrap());
    let cfx    = graph.add_node(Node::from_str("CFX").unwrap());
    let light  = graph.add_node(Node::from_str("LIGHT").unwrap());
    let enviro = graph.add_node(Node::from_str("ENVIRO").unwrap());
    let fx     = graph.add_node(Node::from_str("FX").unwrap());
    let comp   = graph.add_node(Node::from_str("COMP").unwrap());

    let work   = graph.add_node(
        Node::new(Valid::Regexp{name: "work".to_string(), pattern: Regexp::new(r"^work\.[a-z]+$").unwrap()},
        NodeType::Directory)
    );

    let sequence = graph.add_node(
        Node::new(Valid::Regexp{ name: "sequence".to_string(), pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap()},
        NodeType::Directory)
    );

    let shot = graph.add_node(
        Node::new(Valid::Regexp{ name: "shot".to_string(),
                                 pattern: Regexp::new(r"^[0-9]+[A-Z 0-9]*$").unwrap()
                                },
        NodeType::Directory)
    );

    graph.extend_with_edges(&[
        (root,   dd,     1.0),
        (dd,     shows,  1.0),
        (shows,  show,   1.0),
        (show,   etc,    1.0),
        (show,   user,   1.0), (user,   work, 1.0),
        (show,   shared, 1.0), (shared, etc,  1.0),

        (shared, previz, 1.0),
        (shared, integ, 1.0),
        (shared, model, 1.0),
        (shared, rig, 1.0),
        (shared, anim, 1.0),
        (shared, cfx, 1.0),
        (shared, fx, 1.0),
        (shared, light, 1.0),
        (shared, enviro, 1.0),
        (shared, comp, 1.0),

        (show,     sequence, 1.0), (sequence, etc,  1.0),
        (sequence, shared,   1.0), //(shared,   etc,  1.0),
        (sequence, user,     1.0), //(user,     work, 1.0),

        (sequence, shot,   1.0), (shot,   etc,  1.0),
        (shot,     shared, 1.0), //(shared, etc,  1.0),
        (shot,     user,   1.0), //(user,   work, 1.0),
    ]);
    graph
}

fn is_valid(
    mut path: std::path::Iter,
    graph: &petgraph::Graph<Node, f32>,
    parent: petgraph::graph::NodeIndex<DefaultIx>
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
                    if is_valid(path.clone(), graph, n) {
                        return true;
                    }
                }
                cnt +=1;
            }
            // cannot find a way to get number of children for node any other way.
            // we assume that if we have made it this far, and there are no children,
            // we are successful. This allows the path to extend beyond the graph.
            if cnt == 0 {
                return true;
            }
        },
        None => { return true; }
    }
    false
}

fn main() {
    let graph = build_graph();
    //println!("{:#?}",  petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]));
    for node in graph.node_references() {
        println!("{:?}", node);
    }


    println!("\nBFS");

    let mut bfs = Bfs::new(&graph, graph.node_references().next().unwrap().0);
    while let Some(nx) = bfs.next(&graph) {
        // we can access `graph` mutably here still
        println!("{:?}", nx);
    }

    println!("\nNEIGHBORS");
    let mut neighbors = graph.neighbors(graph.node_references().next().unwrap().0);
    for n in neighbors {
        println!("{:?}", n);

    }

    let mut it = Path::new("/dd/shows/DEV01/SHARED/MODEL/foo/bar").iter();
    it.next();
    println!("is /dd/shows/DEV01/SHARED/MODEL valid? {}", is_valid(it, &graph, graph.node_references().next().unwrap().0));


}
