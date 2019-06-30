use crate::{JGraph, NIndex, NodePath, ValidPath, JSPError};
use colored::*;
use log;
use std::path::{PathBuf, Path};
use std::ffi::{OsString, OsStr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Success<'a> {
    Mk(ValidPath<'a>),
    General,
}

pub fn mk_success(path: &Path, verbose:bool ) {
    let cr = if verbose {"\n"} else {""};
    println!("{}{} {}{}", cr, "Created:".bright_blue(), path.display(), cr);
    log::info!("mk_success. Created {}", path.display());
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
    eprintln!("\nSuccess\n");
    for n in nodepath.iter() {
        eprintln!("{:?}", n.display_name());
    }
    
    println!();
    log::info!("validate_success. Success: {:?}", nodepath);
}

/// Report failed task to user, given context
pub fn failure(input: &OsStr, entry: &OsString, node: NIndex, depth: u8, graph: &JGraph, verbose: bool ) {
    let path = Path::new(input)
                .iter()
                .take((depth+1) as usize)
                .fold(PathBuf::new(), |mut p, v| {p.push(v); p});

    let neighbors = graph.neighbors(node);
    let entry_str = entry.to_str().unwrap_or("");
    if verbose { eprintln!("\n{}\n", "Failure".bright_red()); }
    eprintln!("Failed to match {} in {:?} against:", entry_str.bright_red(), &path);
    for n in neighbors {
        eprintln!("{}", graph[n].display_name().bright_red());
    }
    if verbose { eprintln!(""); }
    log::error!("report_failure. Failed to match {} in {:?}", entry_str, &path);
    std::process::exit(1);
}

/// Report simple failure to the user given an error str and a verbose bool
pub fn simple_failure(error: &str, verbose: bool ) {
    if verbose { 
        eprintln!("\n{}\n", "Error".bright_red()); 
        eprintln!("\t{}", error);
        eprintln!("");
    } else {
        eprintln!("{} {}", "Error".bright_red(), error);
    }
    log::error!("report_simple_failure. {}", error);
    // what am i doing here? this should't be in log
    std::process::exit(1);
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
    if verbose { 
        eprintln!("\n{}\n", "Error".bright_red()); 
        eprintln!("\t{} '{}'", info, error_str);
        eprintln!("");
    } else {
        eprintln!("{} {} '{}'", "Error".bright_red(), info, error_str);
    }
    log::error!("jsperror. {} {}", info, error_str);
}

pub(crate) fn go_failure(path_str: &str, myshell: bool, verbose: bool) {
    let cr = if verbose { "\n" } else {""};
    if !myshell {
        eprintln!("echo {}Error: Path does not exist: {}{}", cr, path_str.bright_blue(), cr);
    } else {
        eprintln!("{}Error: Path does not exist: '{}'{}", cr, path_str.bright_blue(), cr);
    }
    log::error!("go_failure: Path does not exist: '{}'", path_str);
}