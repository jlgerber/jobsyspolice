use chrono;
use colored::Colorize;
use dotenv::dotenv;
use fern::{ colors::{Color, ColoredLevelConfig}, self} ;
use jsp::{ go, mk, Node, SupportedShell, CachedEnvVars, constants, diskutils, DiskType, find, get_disk_service, graph, validate_path, JGraph, JSPError, NodePath, NIndex, SearchTerm, ShellEnvManager};
use petgraph;
use log::{ LevelFilter, self };
use serde_json;
use std::{ env, io::{BufWriter, Write}, path::{ Path, Component, PathBuf }, fs::File };
use structopt::StructOpt;
use std::ffi::OsString;
use std::str::FromStr;
use std::collections::VecDeque;
use levelspecter::{LevelSpec, LevelName};
use jsp::jspt::{JSPTemplateError, Loader, State, JGraphKeyMap, RegexMap};
use std::io::{BufReader, BufRead};

#[derive(Debug, StructOpt)]
#[structopt( name = "jsp", about = "
Job System Police

Interact with the jstemplate.json file. \
This command may be used to validate candidate paths, create the template, etc" )]
struct Opt {
    /// Set logging level to one of trace, debug, info, warn, error
    #[structopt( short = "l", long = "level", default_value = "warn" )]
    level: String,

    /// Generate a Graphviz dot file of the jstemplate and print it to stdout
    #[structopt( short="d", long = "dot", parse(from_os_str))]
    dot: Option<PathBuf>,

    /// Write the graph out as json using an interally maintained definition
    #[structopt( short = "f", long = "file", parse(from_os_str) )]
    file: Option<PathBuf>,

    /// Read the graph from a specified template file. Normally, we identify
    /// the template from the JSP_PATH environment variable
    #[structopt( short = "i", long = "input", parse(from_os_str) )]
    graph: Option<PathBuf>,

    /// Jobsystem path to validate (eg /dd/shows/FOOBAR)
    #[structopt(name="INPUT", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(subcommand)]
    subcmd: Option<Subcommand>,
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    /// Jobsystem path to create (eg /dd/shows/FOOBAR)
    #[structopt(name = "mk")]
    Mk {
        //#[structopt(name="INPUT", parse(from_os_str))]
        //input: PathBuf,
        
        /// one or more search tearms of the form key:value , or a 
        /// fullpath, depending upon other field
        #[structopt(name="TERMS")]
        terms: Vec<String>,
        
        /// accept a fullpath instead of key:value pairs
        #[structopt(short = "f", long = "fullpath")]
        full_path: bool,

        /// Print Success / Failure information. And in color!
        #[structopt(short = "v", long = "verbose")]
        verbose: bool,
    },
    /// Navigation command
    #[structopt(name = "go")]
    Go {
        /// one or more search tearms of the form key:value , or a 
        /// fullpath, depending upon other field
        #[structopt(name="TERMS")]
        terms: Vec<String>,

        /// choose a shell (bash)
        #[structopt(short = "s", long = "shell")]
        myshell: Option<String>,

        /// accept a fullpath instead of key:value pairs
        #[structopt(short = "f", long = "fullpath")]
        full_path: bool,

        /// Print Success / Failure information. And in color!
        #[structopt(short = "v", long = "verbose")]
        verbose: bool,

    }
}


fn main() -> Result<(), failure::Error> {

    // Slurp in env vars from .env files in the path.
    dotenv().ok();
    let (args, level) = setup_cli();
    setup_logger(level).unwrap();
    
    let  ( graph,  keymap,  regexmap)  = get_graph(args.file.is_some(), args.graph)?;

    //
    // Handle jstemplate file output in main command. 
    // 
    // if args.file.is_some() {
    //     if let Some(mut output) = args.file {
    //         if args.input.is_some() {
    //             log::warn!("INPUT not compatible with --file argument. It will be ignored");
    //         }
    //         output = diskutils::convert_relative_pathbuf_to_absolute(output)?;
    //         diskutils::write_template(&mut output, &graph);
    //     }
    // //
    // // Handle Dot output in the main command. We are writing the template out as a dot file
    // //
    // } else 
    
    if args.dot.is_some() {
        if let Some(mut output) = args.dot {
            if args.input.is_some() {
                log::warn!("INPUT not compatible with --dot argument. It will be ignored");
            }
            output = diskutils::convert_relative_pathbuf_to_absolute(output)?;
            diskutils::write_template_as_dotfile(&output, &graph);
        } else {
            println!("{:#?}",  petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]));
        }
    //
    // Handle Directory Creation via the mk subcommand
    //
    } else if let Some(Subcommand::Mk{terms, full_path, verbose}) = args.subcmd {
        mk(terms, &graph, full_path, verbose)?;
    //   
    // Handle Navigation via the Go subcommand
    //
    }  else if let Some(Subcommand::Go{terms, myshell, full_path, verbose}) = args.subcmd {
        go(terms, myshell, &graph, full_path, verbose)?;
    //
    // Validate supplied argument to determine whether it is a valid path or not
    //
    } else if let Some(mut input) = args.input {
        input = diskutils::convert_relative_pathbuf_to_absolute(input)?;
        match validate_path(&input, &graph) {
            Ok(nodepath) => {
                report_success(nodepath);
            },
            Err(JSPError::ValidationFailure{entry, node, depth}) => {
                report_failure(input.as_os_str(), &entry, node, depth, &graph, true );
            }
            Err(_) => panic!("JSPError type returned invalid")
        }

    //
    // Don't know what you are thinking. I will print help and get out of your way
    //
    } else {
        Opt::clap().print_help().unwrap();
    }
    Ok(())
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
    let jsp_path = env::var(constants::JSP_PATH)?;
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
            eprintln!("\nunable to get template from environment: {}. Is {} set?\n", e, constants::JSP_PATH);
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
pub fn get_graph(has_output: bool,graph: Option<PathBuf>) ->  Result<(JGraph, JGraphKeyMap, RegexMap), JSPTemplateError>  {
    if graph.is_none() {
        let template = get_template_from_env();
        let file = open_template(&template);


        //let file = File::open(opt.input)?;
        let bufreader =  BufReader::new(file);

        // lets create structs that Loader::new requires
        let (mut jgraph, mut keymap, mut regexmap) = Loader::setup();
        // and now call Loader::new with them.
        let mut loader = Loader::new(&mut jgraph, &mut keymap, &mut regexmap);


            // let result: JGraph =
            // serde_json::from_reader(json_file).expect("error while reading json");
            // result

        loader.load(bufreader)?;
        
        Ok((jgraph, keymap, regexmap))
    } else {
        let file_path = graph.unwrap();
        let file = File::open(file_path).expect("file not found");
         //let file = File::open(opt.input)?;
        let bufreader =  BufReader::new(file);

        // lets create structs that Loader::new requires
        let (mut jgraph, mut keymap, mut regexmap) = Loader::setup();
        // and now call Loader::new with them.
        let mut loader = Loader::new(&mut jgraph, &mut keymap, &mut regexmap);


            // let result: JGraph =
            // serde_json::from_reader(json_file).expect("error while reading json");
            // result

        loader.load(bufreader)?;
        
        Ok((jgraph, keymap, regexmap))
    }
}

// #[inline]
// fn get_graph(has_output: bool, graph: Option<PathBuf>) -> Result<(JGraph, JGraphKeyMap, RegexMap), JSPTemplateError> {
//     if has_output {
//         graph::testdata::build_graph()
//     } else {
//          _get_graph(graph)
//     }
// }

#[inline]
fn report_success(nodepath: NodePath) {
    eprintln!("\nSuccess\n");

    for n in nodepath.iter() {
        eprintln!("{:?}", n.display_name());
    }

    println!("");
}

#[inline]
fn report_failure(input: &std::ffi::OsStr, entry: &OsString, node: NIndex, depth: u8, graph: &JGraph, verbose: bool ) {
    let path = Path::new(input)
                .iter()
                .take((depth+1) as usize)
                .fold(PathBuf::new(), |mut p, v| {p.push(v); p});

    let neighbors = graph.neighbors(node);
    if verbose { eprintln!("\n{}\n", "Failure".bright_red()); }
    eprintln!("Failed to match {} in {:?} against:", entry.to_str().unwrap_or("").bright_red(), path);
    for n in neighbors {
        eprintln!("{}", graph[n].display_name().bright_red());
    }
    if verbose { eprintln!(""); }
    std::process::exit(1);
}
