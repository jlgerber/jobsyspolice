use crate::{
    CachedAliases,
    CachedEnvVars,
    constants,
    DiskType,
    find_rel,
    FindRelStrategy,
    get_disk_service,
    JGraph,
    JSPError,
    MetadataTerm,
    Navalias,
    NIndex,
    report,
    SearchTerm,
    SupportedShell,
    ShellEnvManager,
    ValidPath,
    NodeType,
};
use chrono::prelude::*;
use levelspecter::{LevelSpec, LevelName};
use std::{
    collections::VecDeque,
    env,
    path::{Component, PathBuf},
    str::FromStr
};
use std::collections::HashMap;


type NavaliasMap = HashMap<String, PathBuf>;

/// Generate a ValidPath from input. This input may either be an absolute or 
/// relative path, a levelspec and terms,
/// or a straight vector of terms. In any case, `validpath_from_terms` will 
/// attempt to do the right thing.
/// 
/// # Parameters
/// *`terms` - Vec<String> representing a relative or abslolute path 
///            (terms.len() should be 1), or a vec of SearchTerms
/// * `graph` - a reference to the `JGraph` instance
/// * `force_fullpath` - A bool indicating that we wish to force terms[0] to be 
///                      interpreted as a fullpath. Under normal circumstances, 
///                      this should not be necessary, as `validpath_from_terms` 
///                      will already attempt to ascertain the nature of the 
///                      input. However, there are certain ambiguous scenarios
///                      where this is necessary.
/// 
/// # Returns
/// An Ok wrapped ValidPath instance when successful 
/// An Error wrapped JSPError when unsuccessful
pub fn validpath_from_terms<'a>(
    mut terms: Vec<String>, 
    graph: &'a JGraph, 
    datetime_dir: bool, 
    force_fullpath: bool
) -> Result<ValidPath<'a>, JSPError> {
    if force_fullpath || ( !terms.is_empty() && terms[0].contains('/') ) {
        let mut pathbuf = PathBuf::from(terms.pop().expect("unable to unwrap"));
        if datetime_dir {
            // construct datetime dir
            pathbuf.push(gen_datetime_dir().as_str());
        }
        // made this true since we are in the fullpath branch
        ValidPath::new(pathbuf, graph, true)
    } else {
        let terms = gen_terms_from_strings(terms)?;
        if datetime_dir {
            let dt = gen_datetime_dir();
            ValidPath::new_from_searchterms(terms, graph, Some(dt.as_str()), force_fullpath)
        } else {
            ValidPath::new_from_searchterms(terms, graph, None, force_fullpath)
        }  
    }
}

#[cfg(test)]
mod validpath_from_terms_test {
    use super::*;
    use crate::graph;
    use std::path::Path;
    use crate::{ Node, jspnode, NodeType, EntryType };

    fn setup_curdir_graph(dirs: Vec<&str> ) -> JGraph {
        let cwd = std::env::current_dir().unwrap();
        let mut graph = JGraph::new();

        let _root = graph.add_node(Node::new_root());
        for dir in cwd.iter() {
            graph.add_node(jspnode!(dir.to_str().unwrap()));
        }
        for dir in dirs {
            graph.add_node(jspnode!(dir));
        }
        graph
    }

    fn setup_curdir_up_graph(dirs: Vec<&str> ) -> JGraph {
        let mut cwd = std::env::current_dir().unwrap();
        cwd.pop();
        let mut graph = JGraph::new();

        let _root = graph.add_node(Node::new_root());
        for dir in cwd.iter() {
            graph.add_node(jspnode!(dir.to_str().unwrap()));
        }
        for dir in dirs {
            graph.add_node(jspnode!(dir));
        }
        graph
    }

    #[test]
    fn fullpath() {
        let graph = graph::testdata::build_graph();
        let vp = validpath_from_terms(vec!["/dd/shows/FOOBAR".to_string()], &graph, false, false).unwrap();
        assert_eq!(vp.path(), Path::new("/dd/shows/FOOBAR"));
    }

    #[test]
    fn levelspec_show() {
        let graph = graph::testdata::build_graph();
        let vp = validpath_from_terms(vec!["FOOBAR".to_string()], &graph, false, false).unwrap();
        assert_eq!(vp.path(), Path::new("/dd/shows/FOOBAR"));
    }

    #[test]
    fn levelspec_seq() {
        let graph = graph::testdata::build_graph();
        let vp = validpath_from_terms(vec!["FOOBAR.RD".to_string()], &graph, false, false).unwrap();
        assert_eq!(vp.path(), Path::new("/dd/shows/FOOBAR/RD"));

    }

    #[test]
    fn levelspec_shot() {
        let graph = graph::testdata::build_graph();
        let vp = validpath_from_terms(vec!["FOOBAR.RD.9999".to_string()], &graph, false, false).unwrap();
        assert_eq!(vp.path(), Path::new("/dd/shows/FOOBAR/RD/9999"));
    }

    #[test]
    fn levelspec_relpath_seq() {
        std::env::set_var("DD_SEQUENCE", "RD");
        std::env::set_var("DD_SHOT", "9999");
        std::env::set_var("DD_SHOW", "FOOBAR");
        let graph = setup_curdir_graph(vec!["AA", "9999"]);
        let mut expected = std::env::current_dir().unwrap();
        expected.push("AA");
        expected.push("9999");
        let vp = validpath_from_terms(vec!["./AA/9999".to_string()], &graph, false, false).unwrap();
        assert_eq!(vp.path(), expected.as_path());
    }
    
    // we go up one directory 
    #[test]
    fn levelspec_relpath_shot() {
        std::env::set_var("DD_SEQUENCE", "RD");
        std::env::set_var("DD_SHOT", "9999");
        std::env::set_var("DD_SHOW", "FOOBAR");
        let graph = setup_curdir_up_graph(vec!["AA", "9999"]);
        let mut expected = std::env::current_dir().unwrap();
        expected.pop();
        expected.push("AA");
        expected.push("9999");
        let vp = validpath_from_terms(vec!["../AA/9999".to_string()], &graph, false, false).unwrap();
        assert_eq!(vp.path(), expected.as_path());
    }
    
    #[test]
    fn levelspec_rel_show() {
        let graph = graph::testdata::build_graph();
        std::env::set_var("DD_SEQUENCE", "RD");
        std::env::set_var("DD_SHOT", "9999");
        std::env::set_var("DD_SHOW", "FOOBAR");
        let vp = validpath_from_terms(vec!["BLA..".to_string()], &graph, false, false).unwrap();
        assert_eq!(vp.path(), Path::new("/dd/shows/BLA/RD/9999"));
    }

    #[test]
    fn levelspec_rel_seq() {
        let graph = graph::testdata::build_graph();
        std::env::set_var("DD_SEQUENCE", "RD");
        std::env::set_var("DD_SHOT", "9999");
        std::env::set_var("DD_SHOW", "FOOBAR");
        let vp = validpath_from_terms(vec![".AA.".to_string()], &graph, false, false).unwrap();
        assert_eq!(vp.path(), Path::new("/dd/shows/FOOBAR/AA/9999"));
    }

    #[test]
    fn levelspec_rel_shot() {
        let graph = graph::testdata::build_graph();
        std::env::set_var("DD_SEQUENCE", "RD");
        std::env::set_var("DD_SHOT", "9999");
        std::env::set_var("DD_SHOW", "FOOBAR");
        let vp = validpath_from_terms(vec!["..1000".to_string()], &graph, false, false).unwrap();
        assert_eq!(vp.path(), Path::new("/dd/shows/FOOBAR/RD/1000"));
    }

    #[test]
    fn levelspec_rel_show_shot() {
        let graph = graph::testdata::build_graph();
        std::env::set_var("DD_SEQUENCE", "RD");
        std::env::set_var("DD_SHOT", "9999");
        std::env::set_var("DD_SHOW", "FOOBAR");
        let vp = validpath_from_terms(vec![".AA.".to_string()], &graph, false, false).unwrap();
        assert_eq!(vp.path(), Path::new("/dd/shows/FOOBAR/AA/9999"));
    }
}

// Generate a datetime directory
fn gen_datetime_dir() -> String {
    let dt: DateTime<Local> = Local::now();
    format!("{}_{}_{}",dt.year(), dt.month(), dt.day())
}

/// Make a series of directories if they do not already exist. 
/// 
/// # Parameters
/// 
/// * `validpath`     - ValidPath instance representing the path we wish to create,
///                     or as a single absolute path. 
/// * `graph`         - Reference to the JGraph which describes the jobsytem template
/// * `full_path`     - Indicates that the first term provided is a regular 
///                     path, as opposed to a levelspec
///                     Normally, Mk detects this using the path separator as an 
///                     indicator. However this may be explicitly set.
/// * `ingore_volume` - If true, treat all Volume nodes as Directory nodes when
///                     creating them
/// * `verbose`   - Output is more extensive, colored, etc.
/// 
/// # Returns
/// An Ok wrapped report::Success::Mk(ValidPath) if successful, 
///An Err wrapped JSPError if unable to make the  provided directory.
pub fn mk<'a>(
    validpath: ValidPath<'a>, 
    graph: &'a JGraph, 
    disktype: DiskType,
    set_stickybit: bool,
    ignore_volume: bool,
    verbose: bool
) -> Result<report::Success<'a>, JSPError> {
    let diskservice = get_disk_service(disktype, graph);
    match diskservice.mk(validpath.path(), set_stickybit, ignore_volume) {
        Ok(_) => { 
            Ok(report::Success::Mk(validpath))
        },
        Err(JSPError::ValidationFailure{entry, node, depth}) => {
            // TODO: think i can remove report here as i should be reporting this higher up the call chain
            report::failure(validpath.path().as_os_str(), &entry, node, depth, &graph, verbose );
            Err(JSPError::ValidationFailureFor{path: validpath.path().to_path_buf(), entry,node,depth})
        },
        Err(e) => {
            Err(e)
        },
    }
}

/// Return shell commands that, wnen evaluated, result in a change of location 
/// in the job system and initialization of environment variables defined in 
/// the template in relation to the path, providing either a levelspec and 
/// optional key:value terms additionally,vor an absolute path to a location. 
/// 
/// # Parameters
/// 
/// * `terms`     - vector of terms representing the navigation request. The 
///                 first item in the vector may either be an absolute path 
///                 or a levelspec. Subsequent values should adhehere to 
///                 `key:value` form. 
/// * `myshell`   - Optionally, the name of the shell that one wishes the 
///                 commands returned to target. bash is the default shell. 
/// * `graph`     - an reference to the JGraph describing the jobsystem template 
/// * `full_path` - explicitly declare that the input is a full path. Under 
///                 usual circumstances,  the command will automatically 
///                 determine this based on the presence of a  path separator 
///                 in the input.
/// * `verbose`   - Output is more extensive, colored, etc.
/// 
/// # Returns
/// A Result wrapping a ValidPath if successful, or a JSPError if unable to navigate 
/// to the supplied directory.
pub fn go<'a> (
    terms: Vec<String>, 
    myshell: Option<String>, 
    graph: &'a JGraph,
    full_path: bool, 
    verbose: bool
) -> Result<ValidPath<'a>, JSPError> {

    let myshell = myshell.unwrap_or_else(|| "bash".to_string());

    let myshelldyn = SupportedShell::from_str(myshell.as_str())?.get();

    match validpath_from_terms(terms, &graph, false, full_path) {
        Ok(validpath) => {
            if let Some(idx) = validpath.nodepath().nindex() {
                // now we process any navaliases
                let navalias_map = process_navalias(idx, &validpath, &graph, verbose);
                // for (k,v) in navmap.into_iter() {
                //     println!("{} {:?}", k, v);
                // }
                process_go_success(&validpath, &navalias_map, myshelldyn);
                Ok(validpath)
            } else {
                panic!("unable to get index NIndex from nodepath");
            }
        },
        
        Err(e) => {
            report::shellerror("Problem converting terms to path", Some(e.clone()), verbose);
            Err(e)
        },
    }
}

#[inline]
fn process_navalias(idx: NIndex, validpath: &ValidPath, graph: &JGraph, verbose: bool) -> NavaliasMap {
    let mut navaliasmap = NavaliasMap::new();

    match find_rel( idx, MetadataTerm::Navalias, &graph, FindRelStrategy::First) {
        Err(e) => { report::shellerror(
            format!("Error: unable to find navalias nodes: {}", e.to_string()).as_str(),
            None, 
            verbose); 
        }
        Ok(nodepaths) => {
            // now we create them
            for mut nodepath in nodepaths {
                
                let last = nodepath.pop().unwrap();
                let lastnode = &graph[last];
                
                match nodepath.to_pathbuf() {
                    Ok(mut v) => {
                        match lastnode.metadata().navalias() {
                            Some(Navalias::Complex{name, value}) => {
                                v.push(value);
                                let full_pathbuf = validpath.pathbuf().join(v);
                                // we need to account for the posibility that an alias has been found for multiple nodes.
                                // The strategy we will use is to only replace a k/v pair if the value is shorter
                                // in length. 
                                match navaliasmap.get(name) {
                                    Some(value) => {
                                        if full_pathbuf.components().count() < value.components().count() {
                                            navaliasmap.insert(name.to_owned(), full_pathbuf);
                                        }
                                    }
                                    None => {
                                        navaliasmap.insert(name.to_owned(), full_pathbuf);
                                    } 
                                }
                            }
                            Some(Navalias::Simple(name)) => {
                                match lastnode.identity() {
                                    NodeType::Simple(n) => v.push(n),
                                    _ => panic!("Illegal combination of non Simple NodeType and Simple Navalias"),
                                }
                                let full_pathbuf = validpath.pathbuf().join(v);
                                match navaliasmap.get(name) {
                                    Some(value) => {
                                        if full_pathbuf.components().count() < value.components().count() {
                                            navaliasmap.insert(name.to_owned(), full_pathbuf);
                                        }
                                    }
                                    None => {
                                        navaliasmap.insert(name.to_owned(), full_pathbuf);
                                    } 
                                }
                            }
                            None => { panic!("lastnode.metadata.navalias is None");}
                        }
                    },
                    Err(e) => {
                        report::shellerror("Unable to convert nodepath to pathbuf. skipping nodepath.", Some(e), verbose);
                        continue
                    } 
                };
            }
        }
    } 
    navaliasmap
}

pub fn gen_terms_from_strings(mut terms: Vec<String>) -> Result<Vec<SearchTerm>, JSPError> {

    let lspec_term;
    if terms.is_empty() {
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
fn process_go_success(validpath: &ValidPath, navalias_map: &NavaliasMap, myshell: Box<dyn ShellEnvManager>) {

    log::info!("process_go_success(...)");
    
    let components = validpath.pathbuf().components().map(|x| {
        match x {
            Component::RootDir => String::from("/"),
            Component::Normal(level) => level.to_str().unwrap().to_string(),
            Component::CurDir => String::from("."),
            Component::ParentDir => String::from(".."),
            Component::Prefix(_) => panic!("prefix in path not supported"),
        }
    }).collect::<VecDeque<String>>();
    
    // set env vars 
    let mut varnames: Vec<&str> = Vec::new();

    // generate string to clear previously cached variables
    let cached = CachedEnvVars::new();
    print!("{}", cached.clear(&myshell));
    // generate code to export a variable
    // TODO: make this part of the trait so that we can abstract over shell
    for (idx, n) in validpath.nodepath().iter().enumerate() {
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
    if !varnames.is_empty() {
        print!("{}", &myshell.set_env_var(constants::JSP_TRACKING_VAR, varnames.join(":").as_str())) ;
    } else {
        print!("{}", &myshell.unset_env_var(constants::JSP_TRACKING_VAR));
    }

    // reuse varnanmes variable
    varnames.clear();
    // iterate through cached aliases, clearing each alias
    let cached = CachedAliases::new();
    print!("{}", cached.clear(&myshell));
    // iterate trhough the navaliases, setting each one
    for (k,v) in navalias_map.into_iter() {
        print!("{}", &myshell.set_alias(k,v));
        varnames.push(k);
    }
    // Reset the JSP_ALIAS_NAMES env var which tracks the previously set aliases
    if !varnames.is_empty() {
        print!("{}", &myshell.set_env_var(constants::JSP_ALIAS_NAMES, varnames.join(":").as_str())) ;
    } else {
        print!("{}", &myshell.unset_env_var(constants::JSP_ALIAS_NAMES));
    }

    // Now the final output of where we are actually going.
    let path = validpath.pathbuf();
    let target_dir = path.as_os_str().to_str().unwrap();
    println!("cd {};", target_dir);
    println!("echo Changed Directory To: {}\n", target_dir);
}
