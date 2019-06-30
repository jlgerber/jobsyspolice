use chrono;
use dotenv::dotenv;
use fern::{ 
    colors::{
        Color, 
        ColoredLevelConfig
    }, 
    self
};
use jsp::{ 
    report_failure,
    report_simple_failure,
    validate_success,
    cli,
    diskutils, 
    validate_path, 
    JSPError, 
    get_graph,
    gen_terms_from_strings,
    find,
};
use log::{ LevelFilter, self };
use petgraph;
use std::path::PathBuf ;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt( name = "jsp", about = "
Job System Police

Interact with the jstemplate.json file. \
This command may be used to validate candidate paths, create the template, etc" )]
struct Opt {
    /// Set logging level to one of trace, debug, info, warn, error
    #[structopt( short = "l", long = "level", default_value = "warn" )]
    level: String,

    /// Generate a Graphviz dot file of the jstemplate and save it to the provided file
    #[structopt( short="d", long = "dot", parse(from_os_str))]
    dot: Option<PathBuf>,

    /// Read the graph from a specified template file. Normally, we identify
    /// the template from the JSP_PATH environment variable
    #[structopt( short = "i", long = "input", parse(from_os_str) )]
    graph: Option<PathBuf>,

    /// one or more search tearms of the form key:value , or a 
    /// fullpath, depending upon other field
    #[structopt(name="TERMS")]
    input: Vec<String>,

    #[structopt(subcommand)]
    subcmd: Option<Subcommand>,
}

#[derive(StructOpt, Debug)]
enum Subcommand {
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
    
    let  ( graph,  _keymap,  _regexmap)  = get_graph(args.graph)?;
 
    if args.dot.is_some() {
        if let Some(mut output) = args.dot {
            if !args.input.is_empty(){
                log::warn!("INPUT not compatible with --dot argument. It will be ignored");
            }
            output = diskutils::convert_relative_pathbuf_to_absolute(output)?;
            // TODO: check to see that output doesnt exist and that its parent partory does exist
            diskutils::write_template_as_dotfile(&output, &graph);
        } else {
            println!("{:#?}",  petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]));
        }

    //   
    // Handle Navigation via the Go subcommand
    //
    }  else if let Some(Subcommand::Go{terms, myshell, full_path, verbose}) = args.subcmd {
        match cli::go(terms, myshell, &graph, full_path, verbose) {
            Ok(()) => (),
            Err(e) => report_simple_failure(e.to_string().as_str(), verbose)
        }
    //
    // Validate supplied argument to determine whether it is a valid path or not
    //
    } else if !args.input.is_empty() {
        let input = args.input;
        
        //let diskservice = get_disk_service(DiskType::Local, &graph);
        if !input.is_empty() && input[0].contains('/')  {
            let mut input = PathBuf::from(&input[0]);
            input = diskutils::convert_relative_pathbuf_to_absolute(input)?;
            match validate_path(&input, &graph) {
                Ok(nodepath) => {
                    validate_success(nodepath);
                },
                Err(JSPError::ValidationFailure{entry, node, depth}) => {
                    report_failure(input.as_os_str(), &entry, node, depth, &graph, true );
                }
                Err(_) => panic!("JSPError type returned invalid")
            }
         } else {

            let terms = gen_terms_from_strings(input)?;

            match find::find_path_from_terms(terms, &graph) {
                Ok(( _path,  nodepath)) => { 
                    validate_success(nodepath);
                },
                Err(e) => {
                    return Err(e)?;
                },
            };
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
