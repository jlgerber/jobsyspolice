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
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

#[derive(Debug, StructOpt)]
#[structopt( name = "jstest", about = "test jobsystem paths" )]
struct Opt {
    /// Set logging level to one of trace, debug, info, warn, error
    #[structopt( short = "l", long = "level", default_value = "info" )]
    level: String,

    /// Generate a Graphviz dot file of the jstemplate and print it to stdout
    #[structopt( long = "dot" )]
    dot: bool,

    /// Set a file as output for relevant commands
    #[structopt( short = "f", long = "file", parse(from_os_str) )]
    output: Option<PathBuf>,

    /// Jobsystem path to validate (eg /dd/shows/FOOBAR)
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
        if let Some(output) = args.output {
            if args.input.is_some() {
                log::warn!("INPUT not compatible with --dot argument. It will be ignored");
            }
            let mut file = match File::create(output) {
                Ok(out) => {
                    log::debug!("attempting to write to {:?}", out);
                    out},
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };
            match file.write_all(
                format!(
                    "{:#?}"
                    ,petgraph::dot::Dot::with_config(
                        &graph,
                        &[petgraph::dot::Config::EdgeNoLabel]
                    )
                ).as_bytes()
            ) {
                Err(e) => {
                    eprintln!("{}",e);
                    std::process::exit(1);
                }
                Ok(_) => ()
            };
        } else {
            println!("{:#?}",  petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]));
        }
    } else if let Some(input) = args.input {
        if args.output.is_some() {
            log::warn!("-f | --file flag does nothing with current combination of flags");
        }
        println!("{}", is_valid(input.as_str(), &graph));
    } else {
        eprintln!("\nPass input to command. See help for more details\n")
    }
}
