use chrono;
use dotenv::dotenv;
use fern::{
    self,
    colors::{Color, ColoredLevelConfig},
};
use jsp::{
    cli, diskutils, find, gen_terms_from_strings, get_graph, get_graph_from_fn, report,
    validate_path, JSPError,
};
use levelspecter::{LevelSpec, LevelType};
use log::{self, LevelFilter};
use serde_json;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "jsp",
    about = "
Job System Police

Interact with the jsptemplate. \
This command may be used to validate candidate paths, write out a dotgraph, \
and invoke shell commands. \
The primary jobsystem commands however, are jspmk and jspgo."
)]
struct Opt {
    /// Set logging level to one of trace, debug, info, warn, error
    #[structopt(short = "l", long = "level", default_value = "warn")]
    level: String,

    /// Print Success / Failure information. And in color!
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Read the graph from a specified template file. Normally, we identify
    /// the template from the JSP_PATH environment variable
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    graph: Option<PathBuf>,

    /// one or more search terms of the form key:value , or a
    /// fullpath, depending upon other field
    #[structopt(name = "TERMS")]
    terms: Vec<String>,

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
        #[structopt(name = "TERMS")]
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
    },

    #[structopt(name = "save")]
    Save {
        /// A search term or full path to show
        #[structopt(name = "SHOW")]
        show: String,
        /// Generate a Graphviz dot file of the jstemplate and save it to the provided file
        #[structopt(parse(from_os_str))]
        path: PathBuf,
        /// Generate a dot file
        #[structopt(long = "dot")]
        dot: bool,
    },
}

fn main() {
    // Slurp in env vars from .env files in the path.
    dotenv().ok();

    let (args, level) = setup_cli();
    setup_logger(level).unwrap();

    let Opt {
        verbose,
        graph,
        terms,
        subcmd,
        ..
    } = args;
    match doit(graph, terms, subcmd) {
        Ok(_) => (),
        Err(JSPError::EmptyArgumentListError) => {
            report::shellerror("Error: No arguments supplied to command", None, verbose);
        }
        Err(JSPError::IoError(e)) => {
            report::shellerror(
                "The supplied input does not resolve to a valid directory or file",
                Some(JSPError::IoError(e)),
                verbose,
            );
        }
        Err(e) => {
            report::shellerror("Error Encountered", Some(e), verbose);
        }
    }
}

fn doit(
    graph: Option<PathBuf>,
    terms: Vec<String>,
    subcmd: Option<Subcommand>,
) -> Result<(), JSPError> {
    if let Some(Subcommand::Go {
        terms,
        myshell,
        full_path,
        verbose,
    }) = subcmd
    {
        if terms.is_empty() {
            return Err(JSPError::EmptyArgumentListError);
        }

        let (graph, _keymap, _regexmap) = {
            get_graph_from_fn(
                graph,
                // turn terms from a Vec<String> to a Vec<&str>
                &terms.iter().map(AsRef::as_ref).collect::<Vec<&str>>(),
                |_| get_path_to_template(terms[0].as_str()),
            )?
        };
        match cli::go(terms, myshell, &graph, full_path, verbose) {
            Ok(_validpath) => (),
            Err(e) => {
                return Err(e);
            }
        }
    } else if let Some(Subcommand::Save { show, path, dot }) = subcmd {
        let (graph, _keymap, _regexmap) = {
            get_graph_from_fn(
                graph,
                &terms.iter().map(AsRef::as_ref).collect::<Vec<&str>>(),
                |_| get_path_to_template(show.as_str()),
            )?
        };

        let path = diskutils::convert_relative_pathbuf_to_absolute(path)?;

        if dot {
            diskutils::write_template_as_dotfile(&path, &graph);
        } else {
            let file = File::create(path)?;
            serde_json::to_writer_pretty(file, &graph)?;
        }
    //
    // Validate supplied argument to determine whether it is a valid path or not
    //
    } else {
        if !terms.is_empty() {
            let (graph, _keymap, _regexmap) = {
                get_graph_from_fn(
                    graph,
                    &terms.iter().map(AsRef::as_ref).collect::<Vec<&str>>(),
                    |_| get_path_to_template(terms[0].as_str()),
                )?
            };

            if !terms.is_empty() && terms[0].contains('/') {
                let mut terms = PathBuf::from(&terms[0]);
                terms = diskutils::convert_relative_pathbuf_to_absolute(terms)?;
                match validate_path(&terms, &graph) {
                    Ok(nodepath) => {
                        report::validate_success(nodepath);
                    }
                    Err(JSPError::ValidationFailure { entry, node, depth }) => {
                        report::failure(terms.as_os_str(), &entry, node, depth, &graph, true);
                    }
                    Err(_) => panic!("JSPError type returned invalid"),
                }
            } else {
                let terms = gen_terms_from_strings(terms)?;

                match find::find_path_from_terms(terms, &graph) {
                    Ok((_path, nodepath)) => {
                        report::validate_success(nodepath);
                    }
                    Err(e) => {
                        return Err(e)?;
                    }
                };
            }
        //
        // Don't know what you are thinking. I will print help and get out of your way
        //
        } else {
            Opt::clap().print_help().unwrap();
        }
    }
    Ok(())
}

#[inline]
fn get_path_to_template(show_str: &str) -> Result<PathBuf, JSPError> {
    let (graph, keymap, _regexmap) = get_graph(None)?;
    let term = match LevelSpec::new(show_str) {
        Ok(ls) => {
            let show = ls.show();
            if show == &LevelType::Relative {
                std::env::var("DD_SHOW")?
            } else {
                show.to_str().to_owned()
            }
        }
        // we assume that a path was passed in as opposed to a levelspec
        Err(_) => show_str.to_string(),
    };
    let search = vec![term];
    // todo handle abs path
    let mut validpath = cli::validpath_from_terms(search, &graph, false, false)?;
    let idx = keymap.get("show").unwrap();
    log::trace!("got index {:?}", idx);
    validpath.remove_past(idx)?;

    let mut pathbuf = validpath.pathbuf();
    pathbuf.push("etc");
    pathbuf.push("template.jspt");
    log::info!("Returning template {:?}", pathbuf);
    Ok(pathbuf)
}

#[inline]
fn setup_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
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
        "info" => LevelFilter::Info,
        "warn" | "warning" => LevelFilter::Warn,
        "err" | "error" => LevelFilter::Error,
        _ => LevelFilter::Warn,
    };

    (args, level)
}
