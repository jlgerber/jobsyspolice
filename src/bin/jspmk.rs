use chrono;
use dotenv::dotenv;
//use failure;
use fern::{ colors::{Color, ColoredLevelConfig}, self} ;
use jsp::{get_graph,  DiskType, cli, report_simple_failure, JSPError, report};
use log::{ LevelFilter, self };
use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt( name = "jspmk", about = "
Job System Police make command

Interact with the jstemplate.json file. \
This command may be used to make filepaths" )]
struct Opt {
    /// Set logging level to one of trace, debug, info, warn, error
    #[structopt( short = "l", long = "level", default_value = "warn" )]
    level: String,

    /// Read the graph from a specified template file. Normally, we identify
    /// the template from the JSP_PATH environment variable
    #[structopt( short = "i", long = "input", parse(from_os_str) )]
    graph: Option<PathBuf>,

    /// one or more search tearms of the form key:value , or a 
    /// fullpath, depending upon other field
    #[structopt(name="TERMS")]
    terms: Vec<String>,
    
    /// Ignore the volume tag in the template and treat those nodes
    /// like regular directories. 
    #[structopt(short = "n", long = "novolume")]
    novolume: bool,

    /// accept a fullpath instead of key:value pairs
    #[structopt(short = "f", long = "fullpath")]
    full_path: bool,

    /// Print Success / Failure information. And in color!
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}


fn doit(args: Opt, level: LevelFilter) -> Result<(), /*failure::Error*/ JSPError > {
    
    let Opt{graph, terms, novolume, full_path, verbose,..} = args;

    setup_logger(level).unwrap();

    let (graph,  _keymap,  _regexmap) = get_graph(graph)?;
    
    let validpath = cli::validpath_from_terms(terms, &graph, full_path)?;

    let validpath = cli::mk(validpath, &graph, DiskType::Local, novolume,  verbose)?;             
    if let report::Success::Mk(validpath) = validpath {
        report::mk_success(validpath.path(), verbose);
    }
    Ok(())
}

fn main() {
    dotenv().ok();

    let (args, level) = setup_cli();
    let verbose = args.verbose;

    match doit(args, level) {
        Ok(_) => {}
        Err(e) => {report_simple_failure(e.to_string().as_str(), verbose)}
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