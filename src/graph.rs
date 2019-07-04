use crate::{ 
    constants,
    Node, 
    ReturnValue, 
    NIndex, 
    NodePath, 
    JSPError, 
    jspt::{
        JGraphKeyMap, 
        RegexMap, 
        Loader,
    } 
};

#[allow(unused_imports)]
use log::{debug, trace};
use petgraph::{ graph::{ DefaultIx, NodeIndex}, visit::IntoNodeReferences };
use std::{ cell::RefCell, env, fs::File, rc::Rc, io::{BufReader}, path::{Path, PathBuf}};


/// Define a type alias for the type of graph we will be using.
/// JGraph is a Jobsystem Graph
pub type JGraph = petgraph::Graph<Node, ()>;

/// Given amn optional path to the graph template, a reference to a vector of arguments, and a function
/// that takes a reference to a vector of args and returns a PathBuf, fetch the 
/// JGraph KeyMap, and RegexMap from the graph if the path is Some. Otherwise invoke the 
/// function, passing it a reference to the args, to retrieve the path to the graph
/// template. And use this path to open the template and fetch the JGraph, KeyMap and 
/// RegexMap instances.
/// 
/// The point of this function is to allow us to write arbitrarily complex code to 
/// fetch the template. The expectation is that one passes the arugment list 
/// from the command line into the function to allow it to search for the PathBuf.
/// 
/// One use might be to preparse the arguments passed in to find the template in a 
/// location that is relative to the input data.
pub fn get_graph_from_fn<T>(graph: Option<PathBuf>, args: &Vec<&str>, fnc: T) 
->  Result<(JGraph, JGraphKeyMap, RegexMap), JSPError> 
where 
    T: Fn(&Vec<&str>) -> Result<PathBuf,JSPError>
{
    let file_path = if let Some(graph) = graph {graph} else { fnc(args)?};
    let file = File::open(file_path)?;
    let bufreader =  BufReader::new(file);

    // lets create structs that Loader::new requires
    let (mut jgraph, mut keymap, mut regexmap) = Loader::setup();
    // and now call Loader::new with them.
    let mut loader = Loader::new(&mut jgraph, &mut keymap, &mut regexmap);

    loader.load(bufreader)?;
    
    Ok((jgraph, keymap, regexmap))
}

/// Reetrieve a graph from a path
pub fn get_graph(graph: Option<PathBuf>) ->  Result<(JGraph, JGraphKeyMap, RegexMap), JSPError>  {
    let args = Vec::new();
    get_graph_from_fn(graph, &args, |_|{ get_template_from_env_or_exit() })
}

/// Determine if the provided path is valid or not.NodeType
///
/// # Parameters
/// * `path` - a &str, String, Path, or PathBuf representing a candidate path
/// * `graph` - a reference to a JGrapch, which is the graph
///             representing the valid paths within the Jobsystem.
///
/// # Returns
///
/// `bool` indicating whether or not `path` is valid based on
/// the schema described by the input `graph`.
pub fn validate_path<'a, I: AsRef<Path>>(path: I, graph: &'a JGraph) -> Result<NodePath<'a>, JSPError> {
    let mut it = path.as_ref().iter();
    // we have to drop the first item, which is the first "/"
    it.next();

    let level: u8 = 0;
    let root_index = graph.node_references().next().unwrap().0;
    // we store the first index as we will be asking for its children, and
    // we both need it to be present and know that it will match all future
    // queries.
    let indices = Vec::new();//vec![root_index]; 
    let result = validate_path_recurse(it, &graph, root_index, level, Rc::new(RefCell::new(indices)));
    match result {
        ReturnValue::Success(vals) => {
            let mut vals = Rc::try_unwrap(vals)
                          .unwrap()
                          .into_inner();
            //log::debug!("vals: {:?}", vals);
            vals.push(root_index); // now that we are reversing, we need to push this onto the end
            vals.reverse();
            Ok(NodePath::new(&graph).replace_nodes_unchecked(vals))
        },
        ReturnValue::Failure{entry, node, depth} => {
            Err(JSPError::ValidationFailure{entry, node, depth})
        }
    }
}

fn get_template_from_env_or_exit() -> Result<PathBuf,JSPError> {
    match get_template_from_env() {
        Ok(p) => Ok(p),
        Err(e) => {
            eprintln!("Unable to get template from environment: {}", e.to_string());
            std::process::exit(1);
        } 
    }
}

#[inline]
fn get_template_from_env() -> Result<PathBuf, JSPError> {
    let jsp_path = env::var(constants::JSP_PATH)?;
    log::trace!("expanding tilde for {:?}", jsp_path);
    let jsp_path = shellexpand::tilde(jsp_path.as_str());
    log::trace!("attempting to cannonicalize {:?}", jsp_path);
    let jsp_path_owned = jsp_path.into_owned();
    let jsp_path = match PathBuf::from(&jsp_path_owned.as_str()).canonicalize() {
        Ok(p) => p,
        Err(e) => {
            log::error!("failed to cannonicalize {}", e);
            // Todo swap this out when implement failure
            return Err(JSPError::TemplateError(format!("unable to cannonicalize: {:?}", &jsp_path_owned)));
        }
    };
    log::trace!("returning {:?}", jsp_path);
    Ok(jsp_path)
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

// helper recursive function
fn validate_path_recurse(
    mut path: std::path::Iter,
    graph: &JGraph,
    parent: NodeIndex<DefaultIx>,
    level: u8,
    indices: Rc<RefCell<Vec<NIndex>>>
) -> ReturnValue {
    //log::warn!("parent {:?}",parent);
    let mut result: Option<ReturnValue> = None;
    let level = level+1;
    let component = path.next();
    match component {
        Some(val) => {
            let mut cnt = 0;
            for n in graph.neighbors(parent) {
                let node = &graph[n];
                log::trace!("testing {:?} against {:?}", val, node);
                if node == val {
                    trace!("MATCH");
                    let r = validate_path_recurse(path.clone(), graph, n, level, indices.clone());
                    if r.is_success() {
                        indices.borrow_mut().push(n);
                        return ReturnValue::Success(indices);
                    } else {
                        match result {
                            None => result = Some(r),
                            Some(ref val) => {
                                if val.depth() < r.depth() {
                                    result = Some(r);
                                }
                            }
                        }
                    }
                } else {
                    trace!("NO MATCH");
                }
                cnt += 1;
            }
            // cannot find a way to get number of children for node any other way.
            // we assume that if we have made it this far, and there are no children,
            // we are successful. This allows the path to extend beyond the graph.
            if cnt == 0 {
                return ReturnValue::Success(indices);
            }
        }
        None => {
            return ReturnValue::Success(indices);
        }
    }
    if result.is_some() {
        result.unwrap()
    } else {
        ReturnValue::Failure{entry: component.unwrap().to_os_string(), node: parent, depth: level }
    }
}

pub mod testdata {
    use crate::{ JGraph, Node, jspnode, Regexp, NodeType, EntryType };

    pub fn build_graph() -> JGraph {
        let mut graph = JGraph::new();

        let root = graph.add_node(Node::new_root());
        let dd = graph.add_node(jspnode!("dd", "owner"=>"jobsys", "perms"=>"751"));
        let shows = graph.add_node(jspnode!("shows"));

        //-------------------//
        //        SHOW       //
        //-------------------//

        let show = graph.add_node(
            jspnode!(
                "show", 
                r"^[A-Z]+[A-Z0-9]*$",r"^(REF|SHARED|OUTSOURCE|LOCATIONS)$", 
                "owner"=>"jobsys", 
                "perms"=>"751",
                "varname" => "DD_SHOW"
            )
        );
        
        //--------------------//
        //          REF       //
        //--------------------//

        /* REF directory structure appears at the show level only */

        let refdir = graph.add_node(jspnode!("REF").set_volume());
        let quicktimes = graph.add_node(jspnode!("quicktimes", "perms"=>"751"));
        let qtsubdir = graph.add_node(jspnode!("qtsubdir", r"^[0-9_]+$"));
        let clientvault = graph.add_node(jspnode!("CLIENT_VAULT").set_volume());
        let clientvaultsd = graph.add_node(jspnode!("clientvault_subdir", r"^(incoming|outgoing)$"));
        let clientvaultssd = graph.add_node(jspnode!("clientvault_ssd", r"^[0-9_]+$"));
        let slates_n_categories = graph.add_node(jspnode!("slatesNcategories", r"(SLATES|CATGORIES)^$"));
        let snc_sd = graph.add_node(jspnode!("snc_sd", r"^[a-z0-9_.-]+$"));
        let locations = graph.add_node(jspnode!("LOCATIONS"));
        let loc_sd = graph.add_node(jspnode!("loc_sd", r"^[a-z0-9_.-]+$"));
        let loc_ssd = graph.add_node(jspnode!("loc_ssd", r"^[a-z0-9_.-]+$"));
        let documents = graph.add_node(jspnode!("documents"));
        let doc_sd = graph.add_node(jspnode!("doc_sd", r"^(agency|director_treatments|vfx_methodology|schedules|scripts|storyboards)$"));
        let audio = graph.add_node(jspnode!("audio"));
        let audio_sd = graph.add_node(jspnode!("audio_sd", r"^(mixes|sources)$"));
        let threed = graph.add_node(jspnode!("3d"));
        let threed_sd = graph.add_node(jspnode!("3d_sd", r"^(3d_assets|mocap)$"));
        let chars = graph.add_node(jspnode!("CHARACTERS"));
        let chars_sd = graph.add_node(
            jspnode!("chars_sd", r"^[a-z0-9_]+$", r"^(DEVL|SHARED|etc|lib|bin|user)$")
        );

        //-------------------------//
        //    ANY LEVEL NODES      //
        //-------------------------//

        /* These nodes may appear at any level in the graph */

        /*   TOOLS  */

        let tools = graph.add_node(jspnode!("tools", "autocreate" => "true")); // 0751 ddinst
        let logs = graph.add_node(jspnode!("logs", "autocreate" => "true")); // 0771
        let package = graph.add_node(jspnode!("package"));
        let extension = graph.add_node(jspnode!("extension"));
        let bin = graph.add_node(jspnode!("bin", "autocreate" => "true"));
        let etc = graph.add_node(jspnode!("etc", "autocreate" => "true")); //0751 ddinst
        let lib = graph.add_node(jspnode!("lib", "autocreate" => "true")); //ddinst
        let lib_sd = graph.add_node(
            jspnode!("lib_sd", r"^(config|cortex|dmx|houdini|integ|jspools|katana|lw|massive|max|maya|mentalray|mkfoldy|moco|mova|nfb|nuke|perl|python[0-9.]*|race|refchef|rman|scratch|setupenv|shader|shoot2x|submission|vray|wam|web)$") // 0771
        );
        let prod = graph.add_node(jspnode!("prod", "perms"=>"755","autocreate" => "true")); 
        let docs = graph.add_node(jspnode!("docs", "perms"=>"771","autocreate" => "true"));

        /*   USER WORK  */

        let user = graph.add_node(jspnode!("user", "perms"=>"751").set_volume()); 
        let work = graph.add_node(
            jspnode!(
                "work", 
                r"^work\.(?P<work>[a-z]+)$", 
                "owner" => "$work", 
                "perms"=>"770",
                "varname" => "DD_WORK"
            )
        );  //default 0555

        /*   SHARED   */

        let shared = graph.add_node(jspnode!("SHARED"));
        let shared_dirs = graph.add_node(jspnode!("dept", r"^(PREVIZ|INTEG|MODEL|RIG|ANIM|CFX|LIGHT|ENVIRO|FX|COMP|IMG)$"));
        let category = graph.add_node(jspnode!("category", r"^(char|prop|veh|scene|enviro|kit)$", "varname" => "DD_CATEGORY"));
        let dept = graph.add_node(jspnode!("department", r"^(integ|model|previz|postviz|enviro|rig|anim|fx|cfx|light|comp|lookdev|shotmodel)$"));
        let subcontext = graph.add_node(jspnode!("subcontext", r"^[a-z]+([_]{0,1}[a-z0-9])*$", "varname" => "DD_SUBCONTEXT"));

        //------------------------//
        //    SHOW LEVEL NODES    //
        //------------------------//

        /*   ASSETDEV */

        let assetdev = graph.add_node(jspnode!("ASSETDEV", "varname" => "DD_SEQUENCE"));
        let adshot = graph.add_node(jspnode!("assetdev shot", r"^([A-Z][A-Z0-9]+[_]{0,1})+[A-Z0-9]+$", "varname" => "DD_SHOT"));

        /*   CLIENT   */

        let client_dd_edit = graph.add_node(jspnode!("client_dd_edit", r"^(CLIENT|DD)$").set_volume());
        let client_dd_edit_sd = graph.add_node(
            jspnode!("client_dd_edit_sd",r"^(([0-9]{4,5})|([0-9]{1,2}?[a-z]+)|([a-z]{2}[0-9]{4,5}))$")
        );
        let color = graph.add_node(jspnode!("color"));

        /*  OUTSOURCE  */

        let outsource = graph.add_node(jspnode!("OUTSOURCE").set_volume());
        let outsource_sd = graph.add_node(jspnode!("outsource_sd", r"^[a-zA-Z0-9_.]+$"));  // default 555
        let outsource_sdd = graph.add_node(
            jspnode!( "outsource_sdd", r"[a-zA-Z0-9_.]+^$", r"^prod$", "perms"=>"770")
        ); // (?!(\bprod\b))

        /*  FINALS */

        let finals = graph.add_node(jspnode!("FINALS", "perms"=>"750")); 
        let finals_sd = graph.add_node(jspnode!("finals_sd", r"[0-9_]+"));

        /*  CONFORM  */

        let conform = graph.add_node(jspnode!("CONFORM"));
        let conform_sd =graph.add_node(jspnode!("conform_sd", r"^[a-z0-9_]+$"));
        // conform can also have SHARED as subdir as well as user docs and prod

        /*  ART DEPT AND EDITORIAL */

        let artdept = graph.add_node(jspnode!("ARTDEPT"));
        let artdept_sd = graph.add_node(jspnode!("artdept_sd", r"^[a-zA-Z0-9_.-]+$", "perms"=>"770")); 
        let storyboard = graph.add_node(jspnode!("STORYBOARD"));
        let storyboard_sd = graph.add_node(
            jspnode!("storyboard_sd", r"^[0-9]{2}_[0-9]{4}$", "perms" => "770")
        );
        let editorial = graph.add_node(jspnode!("STORYBOARD"));
        let film_lens = graph.add_node(jspnode!("film_lens", r"^(FILM|LENS)$"));

        /*  DAILIES */

        let dailies = graph.add_node(jspnode!("DAILIES"));

        //--------------------//
        //      SEQUENCE      //
        //--------------------//

        let sequence = graph.add_node(
            jspnode!(
                "sequence", 
                r"^(([A-Z]{2,4})|LIBRARY)$", 
                r"^(SHARED|etc|lib|tool|user|bin)$",
                "varname" => "DD_SEQUENCE"
            )
        );

        //---------------------//
        //         SHOT        //
        //---------------------//

        let shot = graph.add_node(
            jspnode!(
                "shot", 
                r"^[0-9]+[A-Z0-9]*$",
                "varname" => "DD_SHOT"
            )
        );

        //-------------------//
        //   GRAPH EDGES     //
        //-------------------//

        graph.extend_with_edges(&[
            (root, dd),
            (dd, shows),
            (shows, show),
            (show, tools),
            (show, logs),
            (show, etc),
            (show, color),
            (show, refdir),
              (refdir, quicktimes),
                (quicktimes, qtsubdir),
              (refdir, clientvault),
                (clientvault, clientvaultsd),
                  (clientvaultsd, clientvaultssd),
              (refdir, slates_n_categories),
                (slates_n_categories, snc_sd),
              (refdir, locations),
                (locations, loc_sd),
                  (loc_sd, loc_ssd),
              (refdir, documents),
                (documents, doc_sd),
              (refdir, audio),
              (refdir, audio_sd),
              (refdir, threed),
                (threed, threed_sd),
              (refdir, chars),
                (chars, chars_sd),
        ]);

        graph.extend_with_edges(&[
            (tools, package),
            (tools, extension),
            (tools, bin),
            (show, user),
              (user, work),
            (show, client_dd_edit),
              (client_dd_edit, client_dd_edit_sd),
            (show, shared),
              (shared, etc),
              (shared, shared_dirs),
            (show, lib),
              (lib, lib_sd),
            (show, prod),
            (show, docs),
            (show, outsource),
              (outsource, outsource_sd),
                (outsource_sd, outsource_sdd),
            (show, finals),
              (finals, finals_sd),
            (show, conform),
              (conform, user),
              (conform, shared),
              (conform, conform_sd),
              (conform, docs),
              (conform, prod),
        ]);

        graph.extend_with_edges(&[
            (show, artdept),
              (artdept, artdept_sd),
            (show, storyboard),
              (storyboard, storyboard_sd),
            (show, editorial),
            (show, film_lens),
            (show, dailies),
        ]);
        // split it up because there appears to be
        // a max size for &[]
        graph.extend_with_edges(&[
            (shared_dirs, category),
            (category, dept),
            (dept, subcontext),
            (show, sequence),
              (sequence, tools),
              (sequence, etc),
              (sequence, shared),
              (sequence, user),
              (sequence, shot),
              (sequence, lib),
              (sequence, prod),
            (show, assetdev),
            (assetdev, adshot),

        ]);

        graph.extend_with_edges(&[
            (shot, tools),
            (shot, etc),
            (shot, shared),
            (shot, user),
            (shot, lib),
            (shot, prod),
            (adshot, tools),
            (adshot, etc),
            (adshot, shared),
            (adshot, user),
            (adshot, prod),
            (adshot, docs),
        ]);

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::testdata::build_graph;

    #[test]
    fn path_extends_beyond_graph() {
        let tgraph = build_graph();
        let p = "/dd/shows/DEV01/SHARED/MODEL/veh/model";
        assert!(validate_path(p, &tgraph).is_ok());
    }

    #[test]
    fn shotvalidate_path_recurse_graph() {
        let tgraph = build_graph();
        let p = "/dd/shows/DEV01/RD/9999/SHARED/MODEL";

        assert!(validate_path(p, &tgraph).is_ok());
    }

    #[test]
    fn wrong_path_is_invalid_graph() {
        let tgraph = build_graph();
        let p = "/dd/shows/DEV01/RD/9999/FOO/SHARED/MODEL";
        assert!(validate_path(p, &tgraph).is_err());
    }
}
