use chrono;
use dotenv::dotenv;
use fern::{ colors::{Color, ColoredLevelConfig}, self} ;
use jsp::*;
use log::{ LevelFilter, self };
use serde_json;
use std::{ env,  path::{ Path, PathBuf }, fs::File };
use structopt::StructOpt;
use std::ffi::OsString;

const JSP_PATH: &'static str = "JSP_PATH";

#[derive(Debug, StructOpt)]
#[structopt( name = "jspmk", about = "
Job System Police make command

Interact with the jstemplate.json file. \
This command may be used to validate candidate paths, create the template, etc" )]
struct Opt {
    /// Set logging level to one of trace, debug, info, warn, error
    #[structopt( short = "l", long = "level", default_value = "info" )]
    level: String,

    /// Read the graph from a specified template file. Normally, we identify
    /// the template from the JSP_PATH environment variable
    #[structopt( short = "i", long = "input", parse(from_os_str) )]
    graph: Option<PathBuf>,

    /// Jobsystem path to validate (eg /dd/shows/FOOBAR)
    #[structopt(name="INPUT")]
    input: Option<String>,
}


fn main() {
    dotenv().ok();
    let (args, level) = setup_cli();
    setup_logger(level).unwrap();

    let graph = get_graph(false, args.graph);

    if let Some(input) = args.input {
        let volumemaker = local::DiskService::new(&graph, String::from("jonathangerber"), String::from("751"));
        match volumemaker.mk(Path::new(input.as_str())) {
            Ok(_) => println!("\nSuccess\n"),
            Err(JSPError::ValidationFailure{entry, node, depth}) => {
                report_failure(input.as_str(), &entry, node, depth, &graph );
            },
            Err(e) => println!("\nFailure\n{:?}", e),
        }

    } else {
        Opt::clap().print_help().unwrap();
    }
}

#[inline]
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

#[inline]
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


#[inline]
fn _get_template_from_env() -> Result<PathBuf, env::VarError> {
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

#[inline]
fn get_template_from_env() -> PathBuf {
    match _get_template_from_env() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("\nunable to get template from environment: {}. Is {} set?\n", e, JSP_PATH);
            std::process::exit(1);
        }
    }
}

#[inline]
fn open_template(template: &Path) -> File {
    match File::open(&template) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("\nunable to open {:?}. error: {}\n", template, e);
            std::process::exit(1);
        }
    }
}

#[inline]
fn _get_graph(graph: Option<PathBuf>) -> JGraph {
    if graph.is_none() {
        let template = get_template_from_env();
        let json_file = open_template(&template);
        let result: JGraph =
        serde_json::from_reader(json_file).expect("error while reading json");
        result
    } else {
        let json_file_path = graph.unwrap();
        let json_file = File::open(json_file_path).expect("file not found");
        let result: JGraph =
        serde_json::from_reader(json_file).expect("error while reading json");
        result
    }
}

#[inline]
fn get_graph(has_output: bool, graph: Option<PathBuf>) -> JGraph {
    if has_output {
        graph::testdata::build_graph()
    } else {
         _get_graph(graph)
    }
}


#[inline]
fn report_failure(input: &str, entry: &OsString, node: NIndex, depth: u8, graph: &JGraph ) {
    let path = Path::new(input)
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
