use jstemplate2::*;
use petgraph;
use petgraph::visit::Bfs;
use petgraph::visit::IntoNodeReferences;
use fern;
use fern::colors::{Color, ColoredLevelConfig};
use chrono;
use log;

fn setup_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
    let  colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Cyan)
        .trace(Color::Magenta);;

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger(log::LevelFilter::Debug).unwrap();

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
