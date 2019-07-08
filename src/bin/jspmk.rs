use chrono;
use dotenv::dotenv;
//use failure;
use fern::{ colors::{Color, ColoredLevelConfig}, self} ;
use jsp::{ 
    cli, 
    DiskType, 
    find_rel, 
    FindRelStrategy, 
    get_graph,
    get_graph_from_fn,
    JGraph, 
    JSPError,
    MetadataTerm, 
    //Navalias, 
    NIndex, 
    //parse_show_from_arg, 
    report, 
    //SearchTerm,
    ValidPath
};
use levelspecter::LevelSpec;
use log::{ LevelFilter, self };
use std::{path::PathBuf, convert::AsRef};
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

    /// Set stickybit on directory being created
    #[structopt(long = "sticky")]
    sticky: bool,

    /// Create YYYY_MM_DD directory under provided path
    #[structopt(short="t", long = "datetime")]
    datetime_dir: bool,

    /// Ignore the volume tag in the template and treat those nodes
    /// like regular directories. 
    #[structopt(short = "n", long = "novolume")]
    novolume: bool,

    /// accept a fullpath instead of key:value pairs
    #[structopt(short = "f", long = "fullpath")]
    full_path: bool,

    /*
    /// Use new method for finding show template
    #[structopt(short = "z", long = "new")]
    newfind: bool,
    */
    /// Print Success / Failure information. And in color!
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}


fn doit(args: Opt, level: LevelFilter) -> Result<(), /*failure::Error*/ JSPError > {
    
    let Opt{graph, terms, autocreate, sticky, datetime_dir, novolume, full_path, /*newfind,*/ verbose,..} = args;
    if terms.len() == 0 {
        eprintln!("Must supply at least one term as input. See help");
        Opt::clap().print_help().unwrap();
        std::process::exit(1);
    }
    setup_logger(level).unwrap();

    let (graph,  _keymap,  _regexmap) =  {//if newfind {
        get_graph_from_fn(graph, &terms.iter().map(AsRef::as_ref).collect::<Vec<&str>>(), |_|{ 
            // get the graph
            let (graph, keymap, _regexmap) = get_graph(None)?;
            
            let term = match LevelSpec::new(&terms[0]) {
                Ok(ls) => ls.show().to_string(),
                Err(_) => terms[0].to_string(),
            };
            let search = vec![term];
            // todo handle abs path
            let mut validpath = cli::validpath_from_terms(search, &graph, false, full_path)?;
        
            let idx = keymap.get("show").unwrap();
            log::trace!("got index {:?}",idx );
            validpath.remove_past(idx)?;

            let mut pathbuf = validpath.pathbuf();
            pathbuf.push("etc");
            pathbuf.push("template.jspt");
            log::info!("Returning template {:?}", pathbuf);
            Ok( pathbuf)
        })?
    };
     /*
    {
        get_graph_from_fn(graph, &terms.iter().map(AsRef::as_ref).collect::<Vec<&str>>(), |_|{ 
            let show = parse_show_from_arg(terms[0].as_str())?;
            let path = format!("/dd/shows/{}/etc/template.jspt", show);
            Ok( PathBuf::from(path))
        })?
    };
    */
  
    let validpath = cli::validpath_from_terms(terms, &graph, datetime_dir, full_path)?;
    
    let validpath = cli::mk(validpath, &graph, DiskType::Local, sticky, novolume, verbose)?;             
    if let report::Success::Mk(validpath) = validpath {
        
            // find relative
        if let Some(idx) = validpath.nodepath().nindex() {
            if autocreate {
                process_autocreate(idx, &validpath, &graph, novolume, verbose);
                
            }
            // now we process any navaliases
            // this shouldn't be in jspmk
            //process_navalias(idx, &validpath, &graph, verbose);
        } else {
            panic!("unable to get index NIndex from nodepath");
        }
         
        report::mk_success(validpath.path(), verbose);
    }
    Ok(())
}

#[inline]
fn process_autocreate(idx: NIndex, validpath: &ValidPath,  graph: &JGraph, novolume:bool, verbose: bool) {
    match find_rel( idx, MetadataTerm::Autocreate, &graph, FindRelStrategy::Deepest) {
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
                // it doesnt matter whether otp.sticky is true. for the sudbirs we set sticky to false
                let _ = match cli::mk(new_validpath, &graph, DiskType::Local, false, novolume,  verbose) {
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

// #[inline]
// fn process_navalias(idx: NIndex, validpath: &ValidPath, graph: &JGraph, verbose: bool) {
//     match find_rel( idx, MetadataTerm::Navalias, &graph, FindRelStrategy::First) {
//         Err(e) => { report::shellerror(
//             format!("Error: unable to find navalias nodes: {}", e.to_string()).as_str(),
//             None, 
//             verbose); 
//         }
//         Ok(nodepaths) => {
//             println!("searching through {:?}", nodepaths);

//             // now we create them
//             for nodepath in nodepaths {
//                 println!("testing nodepath for navalias");
//                 // generate a Pathbuf from the current nodepath
//                 let cur_pathbuf = match nodepath.to_pathbuf() {
//                     Ok(v) => v,
//                     Err(e) => {
//                         //eprintln!("Error: unable to convert nodepath to pathbuf. skipping nodepath {}",
//                         //    e.to_string());
//                         report::shellerror("Unable to convert nodepath to pathbuf. skipping nodepath.", Some(e), verbose);
//                         continue
//                     } 
//                 };
//                 // the full pathbuf 
//                 let full_pathbuf = validpath.pathbuf().join(cur_pathbuf);
//                 // a copy of the cufrent nodepath
//                 let mut cur_nodepath_clone = nodepath.clone();
//                 // combine the full_pathbuf and the cur_nodepath_clone
//                 let mut full_nodepath = validpath.nodepath().clone();
//                 full_nodepath.append_unchecked(&mut cur_nodepath_clone.nodes);
//                 // build a new validpath from the full pathbuf and fullnodepath
//                 let new_validpath = match ValidPath::new_unchecked(full_pathbuf, full_nodepath, true) {
//                     Ok(v) => v,
//                     Err(e) => {
//                         //eprintln!("Error: Unable to create ValidPath. Err: {}",e.to_string());
//                         report::shellerror("Unable to create ValidPath", Some(e), verbose);
//                         continue
//                     }
//                 };
//                 if let Some(node) = nodepath.leaf() {
//                     if let Some(navalias) = node.metadata().navalias() {
//                         match navalias {
//                             Navalias::Simple(name) => report::shellinfo(format!("I Founds a navalias {}", name), verbose),
//                             Navalias::Complex{name,value} => report::shellinfo(format!("I found {} {}", name, value), verbose),
//                         }
//                     } else {
//                         report::shellerror("In process_navalias, unable to retrieve navalias from Node", None, verbose);
//                         continue;    
//                     }
//                 } else {
//                     report::shellerror("In process_navalias, unable to retrieve leaf node from nodepath", None, verbose)
//                 }
//             }
//         }
//     } 
// }

fn main() {
    dotenv().ok();

    let (args, level) = setup_cli();
    let verbose = args.verbose;

    match doit(args, level) {
        Ok(_) => {}
        Err(e) => {
            report::simple_failure(e.to_string().as_str(), verbose);
            std::process::exit(1);
        }
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