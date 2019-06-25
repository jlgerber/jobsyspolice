use crate::{JGraph, NIndex};
use std::path::{PathBuf, Path};
use std::ffi::{OsString, OsStr};
use colored::*;

pub fn report_failure(input: &OsStr, entry: &OsString, node: NIndex, depth: u8, graph: &JGraph, verbose: bool ) {
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

pub fn report_simple_failure(error: &str, verbose: bool ) {
    if verbose { 
        eprintln!("\n{}\n", "Error".bright_red()); 
        eprintln!("\t{}", error);
        eprintln!("");
    } else {
        eprintln!("{} {}", "Error".bright_red(), error);
    }
    std::process::exit(1);
}
