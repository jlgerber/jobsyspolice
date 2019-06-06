pub use crate::{ Node, ReturnValue, NIndex, NodePath, JSPError };
#[allow(unused_imports)]
use log::{debug, trace};
use petgraph::{ graph::{ DefaultIx, NodeIndex}, visit::IntoNodeReferences };
use std::{ cell::RefCell, rc::Rc, path::Path };

/// Define a type alias for the type of graph we will be using.
/// JGraph is a Jobsystem Graph
pub type JGraph = petgraph::Graph<Node, ()>;

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
pub fn is_valid<'a, I: AsRef<Path>>(path: I, graph: &'a JGraph) -> Result<NodePath<'a>, JSPError> {
    let mut it = path.as_ref().iter();
    // we have to drop the first item, which is the first "/"
    it.next();
    let level: u8 = 0;
    let indices = Vec::new();
    let result = _is_valid(it, &graph, graph.node_references().next().unwrap().0, level, Rc::new(RefCell::new(indices)));
    match result {
        ReturnValue::Success(vals) => {
            let mut vals = Rc::try_unwrap(vals)
                          .unwrap()
                          .into_inner();
            //log::debug!("vals: {:?}", vals);
            vals.reverse();
            Ok(NodePath::new(&graph).replace_nodes_unchecked(vals))
        },
        ReturnValue::Failure{entry, node, depth} => {
            Err(JSPError::ValidationFailure{entry, node, depth})
        }
    }
}

// helper recursive function
fn _is_valid(
    mut path: std::path::Iter,
    graph: &JGraph,
    parent: NodeIndex<DefaultIx>,
    level: u8,
    indices: Rc<RefCell<Vec<NIndex>>>
) -> ReturnValue {

    let mut result: Option<ReturnValue> = None;
    let level = level+1;
    let component = path.next();
    match component {
        Some(val) => {
            let mut cnt = 0;
            for n in graph.neighbors(parent) {
                let node = &graph[n];
                trace!("testing {:?} against {:?}", val, node);
                if node == val {
                    trace!("MATCH");
                    let r = _is_valid(path.clone(), graph, n, level, indices.clone());
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
        let show = graph.add_node(jspnode!("show", r"^[A-Z]+[A-Z0-9]*$",r"^(REF|SHARED|OUTSOURCE|LOCATIONS)$", "owner"=>"jobsys", "perms"=>"751"));

        //ref
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

        // SHOW
        let client_dd_edit = graph.add_node(jspnode!("client_dd_edit", r"^(CLIENT|DD)$").set_volume());
        let client_dd_edit_sd = graph.add_node(
            jspnode!("client_dd_edit_sd",r"^(([0-9]{4,5})|([0-9]{1,2}?[a-z]+)|([a-z]{2}[0-9]{4,5}))$")
        );
        let tools = graph.add_node(jspnode!("tools")); // 0751 ddinst
        let logs = graph.add_node(jspnode!("logs")); // 0771
        let package = graph.add_node(jspnode!("package"));
        let extension = graph.add_node(jspnode!("extension"));
        let color = graph.add_node(jspnode!("color"));
        let category = graph.add_node(jspnode!("category", r"^(char|prop|veh|scene|enviro|kit)$"));
        let dept = graph.add_node(jspnode!("department", r"^(integ|model|previz|postviz|enviro|rig|anim|fx|cfx|light|comp|lookdev|shotmodel)$"));
        let subcontext = graph.add_node(jspnode!("subcontext", r"^[a-z]+([_]{0,1}[a-z0-9])*$"));
        let bin = graph.add_node(jspnode!("bin"));
        let etc = graph.add_node(jspnode!("etc")); //0751 ddinst
        let lib = graph.add_node(jspnode!("lib")); //ddinst
        let lib_sd = graph.add_node(
            jspnode!("lib_sd", r"^(config|cortex|dmx|houdini|integ|jspools|katana|lw|massive|max|maya|mentalray|mkfoldy|moco|mova|nfb|nuke|perl|python[0-9.]*|race|refchef|rman|scratch|setupenv|shader|shoot2x|submission|vray|wam|web)$") // 0771
        );
        let prod = graph.add_node(jspnode!("prod", "perms"=>"755")); // 755
        let docs = graph.add_node(jspnode!("docs", "perms"=>"771")); // 0771
        let user = graph.add_node(jspnode!("user", "perms"=>"751").set_volume()); //751
        let work = graph.add_node(jspnode!("work", r"^work\.(?P<user>[a-z]+)$", "owner"=> "$user", "perms"=>"770")); // 0770 default 0555
        let outsource = graph.add_node(jspnode!("OUTSOURCE").set_volume());
        let outsource_sd = graph.add_node(jspnode!("outsource_sd", r"^[a-zA-Z0-9_.]+$")); //perms default 555
        let outsource_sdd = graph.add_node(
            jspnode!( "outsource_sdd", r"[a-zA-Z0-9_.]+^$", r"^prod$", "perms"=>"770")
        ); // 0770 (?!(\bprod\b))
        let finals = graph.add_node(jspnode!("FINALS", "perms"=>"750")); // 750
        let finals_sd = graph.add_node(jspnode!("finals_sd", r"[0-9_]+"));
        let conform = graph.add_node(jspnode!("CONFORM"));
        let conform_sd =graph.add_node(jspnode!("conform_sd", r"^[a-z0-9_]+$"));
        // conform can also have SHARED as subdir as well as user docs and prod

        let artdept = graph.add_node(jspnode!("ARTDEPT"));
        let artdept_sd = graph.add_node(jspnode!("artdept_sd", r"^[a-zA-Z0-9_.-]+$", "perms"=>"770")); //0770
        let storyboard = graph.add_node(jspnode!("STORYBOARD"));
        let storyboard_sd = graph.add_node(
            jspnode!("storyboard_sd", r"^[0-9]{2}_[0-9]{4}$")
        );// 0770
        let editorial = graph.add_node(jspnode!("STORYBOARD"));
        let film_lens = graph.add_node(jspnode!("film_lens", r"^(FILM|LENS)$"));
        let dailies = graph.add_node(jspnode!("DAILIES"));

        let shared = graph.add_node(jspnode!("SHARED"));
        let shared_dirs = graph.add_node(jspnode!("shared_dirs", r"^(PREVIZ|INTEG|MODEL|RIG|ANIM|CFX|LIGHT|ENVIRO|FX|COMP|IMG)$"));
        let assetdev = graph.add_node(jspnode!("ASSETDEV"));
        let adshot = graph.add_node(jspnode!("assetdev shot", r"^([A-Z][A-Z0-9]+[_]{0,1})+[A-Z0-9]+$"));
        let sequence = graph.add_node(
            jspnode!(
                "sequence", r"^(([A-Z]{2,4})|LIBRARY)$", r"^(SHARED|etc|lib|tool|user|bin)$")
        );
        let shot = graph.add_node(jspnode!("shot", r"^[0-9]+[A-Z0-9]*$"));

        graph.extend_with_edges(&[
            (root, dd),
            (dd, shows),
            (shows, show),
            (show, tools),
            (show, logs),
            (show, etc),
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
            (show, etc),
            (show, color),
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
        assert!(is_valid(p, &tgraph).is_ok());
    }

    #[test]
    fn shot_is_valid_graph() {
        let tgraph = build_graph();
        let p = "/dd/shows/DEV01/RD/9999/SHARED/MODEL";

        assert!(is_valid(p, &tgraph).is_ok());
    }

    #[test]
    fn wrong_path_is_invalid_graph() {
        let tgraph = build_graph();
        let p = "/dd/shows/DEV01/RD/9999/FOO/SHARED/MODEL";
        assert!(is_valid(p, &tgraph).is_err());
    }
}
