use chrono;
use colored::Colorize;
use dotenv::dotenv;
use fern::{ colors::{Color, ColoredLevelConfig}, self} ;
use jsp::{ go, mk, SupportedShell, CachedEnvVars, constants, diskutils, DiskType, find, get_disk_service, graph, validate_path, JGraph, JSPError, NodePath, NIndex, SearchTerm, ShellEnvManager};
use petgraph;
use log::{ LevelFilter, self };
use serde_json;
use std::{ env, io::{BufWriter, Write}, path::{ Path, Component, PathBuf }, fs::File };
use structopt::StructOpt;
use std::ffi::OsString;
use std::str::FromStr;
use std::collections::VecDeque;
use levelspecter::{LevelSpec, LevelName};

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
    
    let graph = get_graph(args.file.is_some(), args.graph);

    //
    // Handle jstemplate file output in main command. 
    // 
    if args.file.is_some() {
        if let Some(mut output) = args.file {
            if args.input.is_some() {
                log::warn!("INPUT not compatible with --file argument. It will be ignored");
            }
            output = diskutils::convert_relative_pathbuf_to_absolute(output)?;
            write_template(&mut output, &graph);
        }
    
    //
    // Handle Dot output in the main command. We are writing the template out as a dot file
    //
    } else if args.dot.is_some() {
        if let Some(mut output) = args.dot {
            if args.input.is_some() {
                log::warn!("INPUT not compatible with --dot argument. It will be ignored");
            }
            output = diskutils::convert_relative_pathbuf_to_absolute(output)?;
            write_template_as_dotfile(&output, &graph);
        } else {
            println!("{:#?}",  petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]));
        }

    //
    // Handle Directory Creation via the mk subcommand
    //
    } else if let Some(Subcommand::Mk{terms, full_path, verbose}) = args.subcmd {
        mk(terms, &graph, full_path, verbose)?;
        /*
        let cr = if verbose {"\n"} else {""};
        let diskservice = get_disk_service(DiskType::Local, &graph);
        if full_path{
            let mut input = PathBuf::from(terms.pop().expect("uanble to unwrap"));
            input = diskutils::convert_relative_pathbuf_to_absolute(input)?;

            //match diskservice.mk(Path::new(input.as_str())) {
            match diskservice.mk(input.as_path()) {

                Ok(_) => println!("{}{}{}", cr, "Success".bright_blue(), cr),
                Err(JSPError::ValidationFailure{entry, node, depth}) => {
                    report_failure(input.as_os_str(), &entry, node, depth, &graph, verbose );
                },
                Err(e) => println!("{}{}{}{}{}",cr, "Failure".bright_red(),e.to_string(),cr, cr),
            }
        } else {
            let terms = gen_terms_from_strings(terms)?;
            let input = match find::find_path_from_terms(terms, &graph) {
                Ok(( path,  _)) => { 
                    match diskservice.mk(path.as_path()) {
                        Ok(_) => println!("{}{}{}", cr, "Success".bright_blue(), cr),
                        Err(JSPError::ValidationFailure{entry, node, depth}) => {
                            report_failure(path.as_os_str(), &entry, node, depth, &graph, verbose );
                        },
                        Err(e) => println!("{}{}{}{}{}",cr, "Failure".bright_red(),e.to_string(),cr, cr),
                    }
                },
                Err(e) => {
                    eprintln!("{}{}{}", cr, e.to_string().as_str().bright_red(), cr)
                },
            };
            //match diskservice.mk(Path::new(input.as_str())) {
            
        }
        // if we are dealing with a relative path..
        */
    //   
    // Handle Navigation via the Go subcommand
    //
    }  else if let Some(Subcommand::Go{terms, myshell, full_path, verbose}) = args.subcmd {
        go(terms, myshell, &graph, full_path, verbose)?;
        /*
        let myshell = myshell.unwrap_or("bash".to_string());
        let myshelldyn = SupportedShell::from_str(myshell.as_str())?.get();
        let cr = if verbose {"\n"} else {""};

        if full_path == true {
            // Parse the full path, as opposed to SearchTerms
            let mut input = PathBuf::from(terms.pop().expect("uanble to unwrap"));
            input = diskutils::convert_relative_pathbuf_to_absolute(input)?;
            
            match validate_path(&input, &graph) {
                Ok(ref nodepath) => {
                    if !input.exists() {
                        eprintln!("{}{} does not exist{}", cr, input.to_str().unwrap().bright_blue(), cr);
                    } else {
                        process_go_success(input, nodepath, myshelldyn);
                    }
                },
                Err(JSPError::ValidationFailure{entry, node, depth}) => {
                    report_failure(input.as_os_str(), &entry, node, depth, &graph, verbose );
                }
                Err(_) => panic!("JSPError type returned invalid")
            }
        // Parse SearchTerms 
        } else {
            
            let terms = gen_terms_from_strings(terms)?;

            match find::find_path_from_terms(terms, &graph) {
                Ok(( path,  nodepath)) => { 
                    let path_str = path.to_str().expect("unable to convert path to str. Does it contain non-ascii chars?");
                    if path.is_dir() {
                        process_go_success(path, &nodepath, myshelldyn);
                    } else {
                        print_go_failure(path_str, true, verbose);
                    }
                },
                Err(e) => {
                    eprintln!("{}{}{}", cr, e.to_string().as_str().bright_red(), cr)
                },
            };
        }
    */
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
fn gen_terms_from_strings(mut terms: Vec<String>) -> Result<Vec<SearchTerm>, JSPError> {

    let lspec_term;
    if terms.len() == 0 {
        lspec_term = Vec::new();
    } else if terms.len() == 1 {
        lspec_term = vec![terms.pop().unwrap()];
    } else {
        let tmp = terms.split_off(1);
        lspec_term = terms;
        terms = tmp;
    }
    // convert spec term to searchterms
    let ls = LevelSpec::new(&lspec_term[0])?;

    let mut ls = ls.rel_to_abs(|level|{
        match level {
            LevelName::Show => env::var("DD_SHOW").ok(),
            LevelName::Sequence => env::var("DD_SEQUENCE").ok(),
            LevelName::Shot => env::var("DD_SHOT").ok(),
        }
    })?;

    ls.set_upper();
    let mut levelspec_terms = 
        ls.to_vec_str()
        .into_iter()
        .enumerate()
        .map(|(idx,x)| format!("{}:{}", constants::LEVELS[idx], x))
        .collect::<Vec<String>>();
    levelspec_terms.append(&mut terms);
    // fold over the input vector of Strings, discarding any Strings which cannot
    // be converted to SearchTerms
    let terms: Vec<SearchTerm> = levelspec_terms.into_iter().fold(Vec::new(), |mut acc, x| {
    //let terms: Vec<SearchTerm> = terms.into_iter().fold(Vec::new(), |mut acc, x| {
        match SearchTerm::from_str(&x) {
            Ok(term) => acc.push(term),
            Err(e) => log::error!("{}", e.to_string()),
        };
        acc 
    });
    
    Ok(terms)
} 

#[inline]
fn process_go_success(path: PathBuf, nodepath: &NodePath, myshell: Box<dyn ShellEnvManager>) {
    
    log::info!("process_go_success(...)");
    
    let components = path.components().map(|x| {
        match x {
            Component::RootDir => String::from("/"),
            Component::Normal(level) => level.to_str().unwrap().to_string(),
            Component::CurDir => String::from("."),
            Component::ParentDir => String::from(".."),
            Component::Prefix(_) => panic!("prefix in path not supported"),
        }
    }).collect::<VecDeque<String>>();
       
    let mut varnames: Vec<&str> = Vec::new();

    // generate string to clear previously cached variables
    let cached = CachedEnvVars::new();
    print!("{}", cached.clear(&myshell));
    // generate code to export a variable
    // TODO: make this part of the trait so that we can abstract over shell
    for (idx, n) in nodepath.iter().enumerate() {
        if n.metadata().has_varname() {
            let varname = n.metadata().varname_ref().unwrap();
            print!("{}", &myshell.set_env_var(varname, &components[idx]));
            varnames.push(varname);
        }
    }
    // if we have variable names that we have set, we also need to preserve their names, so that
    // we can clear them out on subsequent runs. This solves the scenario where you navigate
    // deep into the tree, and then later navigate to a shallower level; you don't want the 
    // variables tracking levels deeper than the current depth to be set. 
    if varnames.len() > 0 {
        print!("{}", &myshell.set_env_var(constants::JSP_TRACKING_VAR, varnames.join(":").as_str())) ;
    } else {
        print!("{}", &myshell.unset_env_var(constants::JSP_TRACKING_VAR));
    }
    // Now the final output of where we are actually gong.
    println!("cd {};", path.as_os_str().to_str().unwrap());
}

#[inline]
fn print_go_failure(path_str: &str, myshell: bool, verbose: bool) {
    let cr = if verbose { "\n" } else {""};
    if myshell == false {
        println!("echo {}Error: Path does not exist: {}{}", cr, path_str.bright_blue(), cr);
    } else {
        eprintln!("{}Error: Path does not exist: '{}'{}", cr, path_str.bright_blue(), cr);
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
fn write_template(output: &mut PathBuf, graph: &JGraph) {

    // if we are writing out the template, we use the internal definition
    //let graph = graph::testdata::build_graph();

    // test to see if buffer is a directory. if it is apply the standard name
    if output.is_dir() {
        output.push(constants::JSP_NAME);
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

#[inline]
fn write_template_as_dotfile(output: &PathBuf, graph: &JGraph) {
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
