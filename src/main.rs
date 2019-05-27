use chrono;
use dotenv::dotenv;
use fern::{ colors::{Color, ColoredLevelConfig}, self} ;
use jst::*;
use petgraph;
use log::{ LevelFilter, self };
use serde_json;
use std::{ env, io::{BufWriter, Write}, path::{ Path, PathBuf }, fs::File };
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt( name = "jst", about = "Interact with the jstemplate.json file. \
This command may be used to validate candidate paths, create the template, etc" )]
struct Opt {
    /// Set logging level to one of trace, debug, info, warn, error
    #[structopt( short = "l", long = "level", default_value = "info" )]
    level: String,

    /// Generate a Graphviz dot file of the jstemplate and print it to stdout
    #[structopt( short="d", long = "dot", parse(from_os_str))]
    dot: Option<PathBuf>,

    /// Write the graph out as json using an interally maintained definition
    #[structopt( short = "f", long = "file", parse(from_os_str) )]
    output: Option<PathBuf>,

    /// Read the graph from a specified template file. Normally, we identify
    /// the template from the JST_PATH environment variable
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

const JST_PATH: &'static str = "JST_PATH";
const JST_NAME: &'static str = "jstemplate.json";

fn get_template_from_env() -> Result<PathBuf, env::VarError> {
    let jst_path = env::var(JST_PATH)?;
    log::trace!("expanding tilde for {:?}", jst_path);
    let jst_path = shellexpand::tilde(jst_path.as_str());
    log::trace!("attempting to cannonicalize {:?}", jst_path);
    let jst_path = match PathBuf::from(jst_path.into_owned().as_str()).canonicalize() {
        Ok(p) => p,
        Err(e) => {
            log::error!("failed to cannonicalize {}", e);
            // Todo swap this out when implement failure
            return Err(env::VarError::NotPresent);
        }
    };
    log::trace!("returning {:?}", jst_path);
    Ok(jst_path)
}

fn main() {
    dotenv().ok();
    let (args, level) = setup_cli();
    setup_logger(level).unwrap();
    let graph = if args.graph.is_none() {
        //graph::testdata::build_graph()
        let template = match get_template_from_env() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("\nunable to get template from environment: {}. Is {} set?\n", e, JST_PATH);
                std::process::exit(1);
            }
        };
        let json_file = match File::open(&template) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("\nunable to open {:?}. error: {}\n", template, e);
                std::process::exit(1);
            }
        };
        let result: JGraph =
        serde_json::from_reader(json_file).expect("error while reading json");
        result
    } else {
        let json_file_path = args.graph.unwrap();
        let json_file = File::open(json_file_path).expect("file not found");
        let result: JGraph =
        serde_json::from_reader(json_file).expect("error while reading json");
        result
    };

    if args.output.is_some() {
        if let Some(mut output) = args.output {
            if args.input.is_some() {
                log::warn!("INPUT not compatible with --file argument. It will be ignored");
            }
            // if we are writing out the template, we use the internal definition
            let graph = graph::testdata::build_graph();

            // test to see if buffer is a directory. if it is apply the standard name
            if output.is_dir() {
                output.push(JST_NAME);
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
                            .take((depth+1) as usize)
                            .fold(PathBuf::new(), |mut p, v| {p.push(v); p});

                let neighbors = graph.neighbors(node);
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
