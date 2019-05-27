use chrono;
use fern::{ colors::{Color, ColoredLevelConfig}, self} ;
use jstest::*;
use petgraph;
use log::{ LevelFilter, self };
use serde_json;
use std::{ io::{BufWriter, Write}, path::{ Path, PathBuf }, fs::File };
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt( name = "jstest", about = "test jobsystem paths" )]
struct Opt {
    /// Set logging level to one of trace, debug, info, warn, error
    #[structopt( short = "l", long = "level", default_value = "info" )]
    level: String,

    /// Generate a Graphviz dot file of the jstemplate and print it to stdout
    #[structopt( short="d", long = "dot", parse(from_os_str))]
    dot: Option<PathBuf>,

    /// Write the graph out as json
    #[structopt( short = "f", long = "file", parse(from_os_str) )]
    output: Option<PathBuf>,

    /// read the graph from  json
    #[structopt( short = "i", long = "input", parse(from_os_str) )]
    graph: Option<PathBuf>,

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
    let graph = if args.graph.is_none() {
        graph::testdata::build_graph()
    } else {
        let json_file_path = args.graph.unwrap();
        let json_file = File::open(json_file_path).expect("file not found");
        let result: JGraph =
        serde_json::from_reader(json_file).expect("error while reading json");
        result
    };

    if args.output.is_some() {
        if let Some(output) = args.output {
            if args.input.is_some() {
                log::warn!("INPUT not compatible with --file argument. It will be ignored");
            }
            let j = serde_json::to_string_pretty(&graph).unwrap();
            let file = match File::create(output) {
                Ok(out) => {
                    log::debug!("attempting to write to {:?}", out);
                    out},
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };
            let mut f = BufWriter::new(file);
            f.write_all(j.as_bytes()).expect("Unable to write data");
        }
    } else if args.dot.is_some() {
        if let Some(output) = args.dot {
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
        match is_valid(input.as_str(), &graph) {
            ReturnValue::Success => eprintln!("\nSuccess\n"),
            ReturnValue::Failure{entry, node, depth} => {

                let path = Path::new(input.as_str())
                            .iter()
                            .take((depth+1) as usize) // +1 because '/' is considered 1st element
                            .fold(PathBuf::new(), |mut p, v| {p.push(v); p});

                let neighbors = graph.neighbors(node);
                //let ncount = graph.neighbors(node).count();
                eprintln!("\nFailed to match {:?} in {:?} against:", entry, path);
                for n in neighbors {
                    eprintln!("{}", graph[n].display_name());
                }
                eprintln!("");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("\nPass input to command. See help for more details\n")
    }
}
