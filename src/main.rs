use jstemplate2::*;
use petgraph;
use petgraph::visit::Bfs;
use petgraph::visit::IntoNodeReferences;
use std::str::FromStr;

fn build_graph() -> JGraph {
    let mut graph = JGraph::new();

    let root = graph.add_node(Node::new_root());
    let dd = Node::new(Valid::Name("dd".to_owned()), EntryType::Directory);

    let shows = Node::from_str("shows").unwrap();

    let dd = graph.add_node(dd);
    let shows = graph.add_node(shows);

    let show = graph.add_node(Node::new(
        Valid::Regexp {
            name: "show".to_owned(),
            pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
        },
        EntryType::Directory,
    ));

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

    let work = graph.add_node(Node::new(
        Valid::Regexp {
            name: "work".to_string(),
            pattern: Regexp::new(r"^work\.[a-z]+$").unwrap(),
        },
        EntryType::Directory,
    ));

    let sequence = graph.add_node(Node::new(
        Valid::Regexp {
            name: "sequence".to_string(),
            pattern: Regexp::new(r"^[A-Z]+[A-Z 0-9]*$").unwrap(),
        },
        EntryType::Directory,
    ));

    let shot = graph.add_node(Node::new(
        Valid::Regexp {
            name: "shot".to_string(),
            pattern: Regexp::new(r"^[0-9]+[A-Z 0-9]*$").unwrap(),
        },
        EntryType::Directory,
    ));

    graph.extend_with_edges(&[
        (root, dd, 1.0),
        (dd, shows, 1.0),
        (shows, show, 1.0),
        (show, etc, 1.0),
        (show, user, 1.0),
        (user, work, 1.0),
        (show, shared, 1.0),
        (shared, etc, 1.0),
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
        (show, sequence, 1.0),
        (sequence, etc, 1.0),
        (sequence, shared, 1.0), //(shared,   etc,  1.0),
        (sequence, user, 1.0),   //(user,     work, 1.0),
        (sequence, shot, 1.0),
        (shot, etc, 1.0),
        (shot, shared, 1.0), //(shared, etc,  1.0),
        (shot, user, 1.0),   //(user,   work, 1.0),
    ]);
    graph
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
    let neighbors = graph.neighbors(graph.node_references().next().unwrap().0);
    for n in neighbors {
        println!("{:?}", n);
    }

    let p = "/dd/shows/DEV01/SHARED/MODEL/foo/bar";
    println!("is {} valid? {}", p, is_valid(p, &graph));
}
