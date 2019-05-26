use jstemplate2::*;
use petgraph;
use petgraph::visit::Bfs;
use petgraph::visit::IntoNodeReferences;

fn main() {
    let graph = graph::testdata::build_graph();
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
