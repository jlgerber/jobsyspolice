use crate::{JGraph, NIndex, NodePath, ValidPath, JSPError};
use colored::*;
use log;
use std::path::{PathBuf, Path};
use std::ffi::{OsString, OsStr};

/// Enum to facilitate providing context to successful 
/// execution of code. Returned by cli::mk. 
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Success<'a> {
    Mk(ValidPath<'a>),
    General,
}


/// Report successful execution of cli::mk to user. 
/// 
/// # Parameters
/// 
/// * `path`    - Path successfully created 
/// * `verbose` - Report the success with more verbosity
/// 
/// # Returns
/// None
pub fn mk_success(path: &Path, verbose:bool ) {
    let cr = if verbose {"\n"} else {""};
    log::info!("mk_success. Created {}", path.display());
    println!("{}{} {}{}", cr, "Created:".bright_blue(), path.display(), cr);
}

/// Report successful execution of task to user. 
/// 
/// # Parameters
/// 
/// * `nodepath` - NodePath instance storing ordered list of Nodes 
/// 
/// # Returns
/// None
pub fn validate_success(nodepath: NodePath) {
    log::info!("validate_success. Success: {:?}", nodepath);
    eprintln!("\nSuccess\n");
    for n in nodepath.iter() {
        eprintln!("{:?}", n.display_name());
    }
    println!();
}

/// Report failed task to user, given context
/// 
/// # Parameters
/// 
/// * `input` - OsStr reference
/// * `entry` - OsString reference to entry
/// * `node`  - NIndex of node
/// * `depth` - number of directories deep the failure occured at
/// * `graph` - JGraph reference
/// * `verbose` - Whether to report verbosely or not
/// 
/// # Returns
/// None
pub fn failure(input: &OsStr, entry: &OsString, node: NIndex, depth: u8, graph: &JGraph, verbose: bool ) {
    let path = Path::new(input)
                .iter()
                .take((depth+1) as usize)
                .fold(PathBuf::new(), |mut p, v| {p.push(v); p});

    let neighbors = graph.neighbors(node);
    let entry_str = entry.to_str().unwrap_or("");
    log::error!("Failed to match {} in {:?}", entry_str, &path);
    if verbose { eprintln!("\n{}\n", "Failure".bright_red()); }
    eprintln!("Failed to match {} in {:?} against:", entry_str.bright_red(), &path);
    for n in neighbors {
        eprintln!("{}", graph[n].display_name().bright_red());
    }
    if verbose { eprintln!(""); }
}

/// Report simple failure to the user given an error str and a verbose bool
/// 
/// # Parameters
/// 
/// * `error` - Custom error message provided by any type which implements AsRef<str>
///             and the Display trait. 
/// *`verbose` - Whether to display verbose information or not
/// 
/// # Returns
/// None
pub fn simple_failure<F>(error: F, verbose: bool ) 
where 
    F: AsRef<str> + std::fmt::Display 
{
    let error = error.as_ref();
    log::error!("{}", error);
    if verbose { 
        eprintln!("\n{}\n", "Error".bright_red()); 
        eprintln!("\t{}", error);
        eprintln!("");
    } else {
        eprintln!("{} {}", "Error".bright_red(), error);
    }
}

/// Report a jsperror
/// 
/// # Parameters
/// * `info` - contextual information str from caller
/// * `error` - underlying JSPError
/// * `verbose` - whether to make the error reporting verbose or not
/// 
/// # Returns
/// None
pub fn jsperror(info: &str, error: JSPError, verbose: bool) {
    let error_str = error.to_string();
    log::error!("jsperror. {} {}", info, error_str);
    if verbose { 
        eprintln!("\n{}\n", "Error".bright_red()); 
        eprintln!("\t{} '{}'", info, error_str);
        eprintln!("");
    } else {
        eprintln!("{} {} '{}'", "Error".bright_red(), info, error_str);
    }
}

/// Use to print out message that works with our bash script wrapper
/// 
/// # Parameters
/// 
/// * `info` - Arbitrary error string reported by user
/// * `error` - JSPError that may be optionally supplied to complement info
/// * `verbose` - Whether to output verbose error information or not.
/// 
/// # Returns
/// None
pub fn shellerror(info: &str, error: Option<JSPError>, verbose: bool) {
    let error_str = match error {
        Some(e) => format!(" '{}'",e.to_string()),
        None => "".to_string()
    };

    if verbose { 
        eprintln!("\n{}\n", "Error".bright_red()); 
        eprintln!("\t{}{}", info, error_str);
        eprintln!("");
    } else {
        eprintln!("{} {}{}", "Error".bright_red(), info, error_str);
    }
}

/// Use to print out message that works with our bash script wrapper
/// 
/// # Parameters
/// 
/// * `info` - Information, provided by any time which may be converted to a &str
///            via as_ref(), and which implements the Display trait. 
/// * `verbose` - Whether to print out verbose info or not 
/// 
/// # Returns
/// None
pub fn shellinfo<T>(info: T,verbose: bool) where T: AsRef<str> + std::fmt::Display {
    if verbose { 
        eprintln!("\n{}\n", "Info".bright_green()); 
        eprintln!("\t'{}'", info.as_ref());
        eprintln!("");
    } else {
        eprintln!("{} '{}'", "Info".bright_green(), info.as_ref());
    }
}

/*
pub(crate) fn go_failure(path_str: &str, myshell: bool, verbose: bool) {
    let cr = if verbose { "\n" } else {""};
    if !myshell {
        eprintln!("echo {}Error: Path does not exist: '{}{}'", cr, path_str.bright_blue(), cr);
    } else {
        shellerror(format!("Path does not exist: '{}'", path_str.bright_blue()).as_str(), None, verbose);
    }
}
*/