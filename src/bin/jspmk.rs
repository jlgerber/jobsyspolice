use chrono;
use dotenv::dotenv;
//use failure;
use fern::{ colors::{Color, ColoredLevelConfig}, self} ;
use jsp::{get_graph,  DiskType, cli, JSPError, report, MetadataTerm, find_rel, ValidPath};
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
    
    /// Automatically create any directories beneeth the supplied path that have
    /// the `autocreate` property associated with them in the template
    #[structopt(short = "a", long = "auto")]
    autocreate: bool,

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
    
    let Opt{graph, terms, autocreate, novolume, full_path, verbose,..} = args;

    setup_logger(level).unwrap();

    let (graph,  _keymap,  _regexmap) = get_graph(graph)?;
    
    let validpath = cli::validpath_from_terms(terms, &graph, full_path)?;

    let validpath = cli::mk(validpath, &graph, DiskType::Local, novolume, verbose)?;             
    if let report::Success::Mk(validpath) = validpath {
        if autocreate {
            // find relative
            if let Some(idx) = validpath.nodepath().index() {
                match find_rel( idx, MetadataTerm::Autocreate, &graph) {
                    Err(e) => {eprintln!("Error: unable to find autocreate nodes: {}", e.to_string());}
                    Ok(nodepaths) => {
                        // now we create them
                        for nodepath in nodepaths {
                            // generate a Pathbuf from the current nodepath
                            let cur_pathbuf = match nodepath.to_pathbuf() {
                                Ok(v) => v,
                                Err(e) => {
                                    //eprintln!("Error: unable to convert nodepath to pathbuf. skipping nodepath {}",
                                    //    e.to_string());
                                    report::jsperror("Unable to convert nodepath to pathbuf. skipping nodepath.", e, verbose);
                                    continue
                                } 
                            };
                            // the full pathbuf 
                            let full_pathbuf = validpath.pathbuf().join(cur_pathbuf);
                            // a copy of the cufrent nodepath
                            let mut cur_nodepath_clone = nodepath.clone();
                            // combine the full_pathbuf and the cur_nodepath_clone
                            let mut full_nodepath = validpath.nodepath().clone();
                            full_nodepath.append_unchecked(&mut cur_nodepath_clone.nodes);
                            // build a new validpath from the full pathbuf and fullnodepath
                            let new_validpath = match ValidPath::new_unchecked(full_pathbuf, full_nodepath, true) {
                                Ok(v) => v,
                                Err(e) => {
                                    //eprintln!("Error: Unable to create ValidPath. Err: {}",e.to_string());
                                    report::jsperror("Unable to create ValidPath", e, verbose);
                                    continue
                                }
                            };
                            let _ = match cli::mk(new_validpath, &graph, DiskType::Local, novolume,  verbose) {
                                Ok(v) => v,
                                Err(e) => {
                                    //eprintln!("Error making new subdirectory: {}", e.to_string());
                                    report::jsperror("Problem making automake subdirectory", e, verbose);
                                    continue
                                }
                            };
                        }
                    }
                }
            }
        }

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
        Err(e) => {report::simple_failure(e.to_string().as_str(), verbose)}
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