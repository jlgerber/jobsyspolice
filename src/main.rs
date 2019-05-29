use chrono;
use dotenv::dotenv;
use fern::{ colors::{Color, ColoredLevelConfig}, self} ;
use jsp::*;
use petgraph;
use log::{ LevelFilter, self };
use serde_json;
use std::{ env, io::{BufWriter, Write}, path::{ Path, PathBuf }, fs::File };
use structopt::StructOpt;
use std::rc::Rc;

#[derive(Debug, StructOpt)]
#[structopt( name = "jsp", about = "
Job System Police

Interact with the jstemplate.json file. \
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
    /// the template from the JSP_PATH environment variable
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

const JSP_PATH: &'static str = "JSP_PATH";
const JSP_NAME: &'static str = "jstemplate.json";

fn get_template_from_env() -> Result<PathBuf, env::VarError> {
    let jsp_path = env::var(JSP_PATH)?;
    log::trace!("expanding tilde for {:?}", jsp_path);
    let jsp_path = shellexpand::tilde(jsp_path.as_str());
    log::trace!("attempting to cannonicalize {:?}", jsp_path);
    let jsp_path = match PathBuf::from(jsp_path.into_owned().as_str()).canonicalize() {
        Ok(p) => p,
        Err(e) => {
            log::error!("failed to cannonicalize {}", e);
            // Todo swap this out when implement failure
            return Err(env::VarError::NotPresent);
        }
    };
    log::trace!("returning {:?}", jsp_path);
    Ok(jsp_path)
}

fn main() {
    dotenv().ok();
    let (args, level) = setup_cli();
    setup_logger(level).unwrap();
    let graph = if args.graph.is_none() {
        let template = match get_template_from_env() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("\nunable to get template from environment: {}. Is {} set?\n", e, JSP_PATH);
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
                output.push(JSP_NAME);
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
            ReturnValue::Success(vals) => {
                eprintln!("\nSuccess\n");
                let vals = Rc::try_unwrap(vals)
                            .unwrap()
                            .into_inner();

                //let vals = vals.into_inner();
                for n in vals.into_iter().rev() {
                    eprintln!("{:?}", graph[n].display_name());
                }
                println!("");
            },
            ReturnValue::Failure{entry, node, depth} => {

                let path = Path::new(input.as_str())
                            .iter()
                            .take((depth+1) as usize)
                            .fold(PathBuf::new(), |mut p, v| {p.push(v); p});

                let neighbors = graph.neighbors(node);
                eprintln!("\nFailure\n");
                eprintln!("Failed to match {:?} in {:?} against:", entry, path);
                for n in neighbors {
                    eprintln!("{}", graph[n].display_name());
                }
                eprintln!("");
                std::process::exit(1);
            }
        }
    } else {
        Opt::clap().print_help().unwrap();
    }
}
