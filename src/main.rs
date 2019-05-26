use jstemplate2::*;
use petgraph;
//use petgraph::visit::Bfs;
//use petgraph::visit::IntoNodeReferences;
use fern;
use fern::colors::{Color, ColoredLevelConfig};
use chrono;
use log;
use log::LevelFilter;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "jstest", about = "test jobsystem paths")]
struct Opt {
    /// Set logging level to one of trace, debug, info, warn, error
    #[structopt(short = "l", long = "level", default_value="info")]
    level: String,

    /// Save out a dot file of the graph
    #[structopt( long = "dot")]
    dot: bool,

    /// Input file
    #[structopt(name="INPUT")]
    input: Option<String>,
}

fn setup_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
    let  colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Cyan)
        .trace(Color::BrightCyan);;

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

fn setup_cli() -> (Opt, log::LevelFilter) {
    let args = Opt::from_args();
    let level = match args.level.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info"  => LevelFilter::Info,
        "warn"  | "warning" => LevelFilter::Warn,
        "err" | "error" => LevelFilter::Error,
        _ => LevelFilter::Warn,
    };

    (args, level)
}

fn main() {
    let (args, level) = setup_cli();
    setup_logger(level).unwrap();
    let graph = graph::testdata::build_graph();
    //println!("{:#?}",  petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]));
    /*
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
    */
    //let p = "/dd/shows/DEV01/SHARED/MODEL/foo/bar";
    if args.dot {
        println!("{:#?}",  petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]));
    } else if let Some(input) = args.input {
        println!("{}", is_valid(input.as_str(), &graph));
    } else {
        eprintln!("\nPass input to command. See help for more details\n")
    }
}
