use crate::{
    CachedEnvVars,
    constants,
    diskutils,
    find,
    JGraph,
    JSPError,
    NIndex,
    NodePath,
    SearchTerm,
    SupportedShell,
    ShellEnvManager,
    validate_path,
    get_disk_service,
    DiskType,
};
use colored::Colorize;
use levelspecter::{LevelSpec, LevelName};
use std::{
    collections::VecDeque,
    env,
    ffi::OsString,
    path::{Path, Component, PathBuf},
    str::FromStr
};

/// Make a series of directories if they do not already exist. 
/// 
/// # Parameters
/// 
/// * `terms`     - Vector of strings representing request, either as levelspec and additional key:value pairs,
///                 or as a single absolute path. 
/// * `graph`     - Reference to the JGraph which describes the jobsytem template
/// * `full_path` - Indicates that the first term provided is a regular path, as opposed to a levelspec
///                 Normally, Mk detects this using the path separator as an indicator. However this may
///                 be explicitly set.
/// * `verbose`   - Output is more extensive, colored, etc.
pub fn mk(
    mut terms: Vec<String>, 
    graph: &JGraph, 
    full_path: bool, 
    verbose: bool
) -> Result<(), JSPError> {

    let cr = if verbose {"\n"} else {""};
    let diskservice = get_disk_service(DiskType::Local, graph);

    if full_path || ( terms.len() > 0 && terms[0].contains("/") ) {
        let mut input = PathBuf::from(terms.pop().expect("uanble to unwrap"));
        input = diskutils::convert_relative_pathbuf_to_absolute(input)?;

        match diskservice.mk(input.as_path()) {

            Ok(_) => println!("{}{}{}", cr, "Success".bright_blue(), cr),
            Err(JSPError::ValidationFailure{entry, node, depth}) => {
                report_failure(input.as_os_str(), &entry, node, depth, &graph, verbose );
            },
            Err(e) => println!("{}{}{}{}{}",cr, "Failure".bright_red(),e.to_string(),cr, cr),
        }
    } else {
        let terms = gen_terms_from_strings(terms)?;

        let _input = match find::find_path_from_terms(terms, &graph) {
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
    }
    Ok(()) 
}

/// Return shell commands that, wnen evaluated, result in a change of location in the job system 
/// and initialization of environment variables defined in the template in relation to the path, 
/// providing either a levelspec and optional key:value terms additionally,vor an absolute path 
/// to a location. 
/// 
/// # Parameters
/// 
/// * `terms`     - vector of terms representing the navigation request. The first item in the vector
///                 may either be an absolute path or a levelspec. Subsequent values should adhehere
///                 to `key:value` form. 
/// * `myshell`   - Optionally, the name of the shell that one wishes the commands returned to target. 
///                 bash is the default shell. 
/// * `graph`     - an reference to the JGraph describing the jobsystem template 
/// * `full_path` - explicitly declare that the input is a full path. Under usual circumstances, 
///                 the command will automatically determine this based on the presence of a 
///                 path separator in the input.
/// * `verbose`   - Output is more extensive, colored, etc.
pub fn go (
    mut terms: Vec<String>, 
    myshell: Option<String>, 
    graph: &JGraph, 
    full_path: bool, 
    verbose: bool
) -> Result<(),JSPError> {
        
    let myshell = myshell.unwrap_or("bash".to_string());
    let myshelldyn = SupportedShell::from_str(myshell.as_str())?.get();

    let cr = if verbose {"\n"} else {""};
    if full_path == true || ( terms.len() > 0 && terms[0].contains("/") ) {
        // Parse the full path, as opposed to SearchTerms
        let mut input = PathBuf::from(terms.pop().expect("uanble to unwrap"));
        input = diskutils::convert_relative_pathbuf_to_absolute(input)?;
        
        match validate_path(&input, graph) {
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