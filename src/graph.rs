pub use crate::{ Node, ReturnValue, NIndex };
use petgraph::{ graph::{ DefaultIx, NodeIndex}, visit::IntoNodeReferences };
#[allow(unused_imports)]
use log::{debug, trace};
use std::cell::RefCell;
use std::rc::Rc;

/// Define a type alias for the type of graph we will be using.
/// JGraph is a Jobsystem Graph
pub type JGraph = petgraph::Graph<Node, ()>;

/// Determine if the provided path is valid or not.NodeType
///
/// # Parameters
/// * `path` - a str reference representing a candidate path
/// * `graph` - a reference to a JGrapch, which is the graph
///             representing the valid paths within the Jobsystem.
///
/// # Returns
///     `bool` indicating whether or not `path` is valid based on
/// the schema described by the input `graph`.
pub fn is_valid(path: &str, graph: &JGraph) -> ReturnValue {
    let mut it = std::path::Path::new(path).iter();
    // we have to drop the first item, which is the first "/"
    it.next();
    let level: u8 = 0;
    let indices = Vec::new();
    return _is_valid(it, &graph, graph.node_references().next().unwrap().0, level, Rc::new(RefCell::new(indices)));
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
    use std::str::FromStr;
    use crate::{ JGraph, Node };

    pub fn build_graph() -> JGraph {
        let mut graph = JGraph::new();

        let root = graph.add_node(Node::new_root());
        let dd = graph.add_node(jstnode!("dd"));
        let shows = graph.add_node(jstnode!("shows"));
        let show = graph.add_node(jstnode!("show", r"^[A-Z]+[A-Z0-9]*$"));

        //ref
        let refdir = graph.add_node(jstnode!("REF").set_volume());
        let quicktimes = graph.add_node(jstnode!("quicktimes"));
        let qtsubdir = graph.add_node(jstnode!("qtsubdir", r"^[0-9_]+$"));
        let clientvault = graph.add_node(jstnode!("CLIENT_VAULT").set_volume());
        let clientvaultsd = graph.add_node(jstnode!("clientvault_subdir", r"^(incoming|outgoing)$"));
        let clientvaultssd = graph.add_node(jstnode!("clientvault_ssd", r"^[0-9_]+$"));
        let slates_n_categories = graph.add_node(jstnode!("slatesNcategories", r"(SLATES|CATGORIES)^$"));
        let snc_sd = graph.add_node(jstnode!("snc_sd", r"^[a-z0-9_.-]+$"));
        let locations = graph.add_node(jstnode!("LOCATIONS"));
        let loc_sd = graph.add_node(jstnode!("loc_sd", r"^[a-z0-9_.-]+$"));
        let loc_ssd = graph.add_node(jstnode!("loc_ssd", r"^[a-z0-9_.-]+$"));
        let documents = graph.add_node(jstnode!("documents"));
        let doc_sd = graph.add_node(jstnode!("doc_sd", r"^(agency|director_treatments|vfx_methodology|shcedules|scripts|storyboards)$"));
        let audio = graph.add_node(jstnode!("audio"));
        let audio_sd = graph.add_node(jstnode!("audio_sd", r"^(mixes|sources)$"));
        let threed = graph.add_node(jstnode!("3d"));
        let threed_sd = graph.add_node(jstnode!("3d_sd", r"^(3d_assets|mocap)$"));
        let chars = graph.add_node(jstnode!("CHARACTERS"));
        let chars_sd = graph.add_node(
            jstnode!("chars_sd", r"^[a-z0-9_]+$", r"^(DEVL|SHARED|etc|lib|bin|user)$")
        );

        // SHOW
        let client_dd_edit = graph.add_node(jstnode!("client_dd_edit", r"^(CLIENT|DD)$").set_volume());
        let client_dd_edit_sd = graph.add_node(
            jstnode!("client_dd_edit_sd",r"^(([0-9]{4,5})|([0-9]{1,2}?[a-z]+)|([a-z]{2}[0-9]{4,5}))$")
        );
        let tools = graph.add_node(jstnode!("tools")); // 0751 ddinst
        let logs = graph.add_node(jstnode!("logs")); // 0771
        let package = graph.add_node(jstnode!("package"));
        let extension = graph.add_node(jstnode!("extension"));
        let color = graph.add_node(jstnode!("color"));
        let category = graph.add_node(jstnode!("category", r"^(char|prop|veh|scene|enviro|kit)$"));
        let dept = graph.add_node(jstnode!("department", r"^(integ|model|previz|postviz|enviro|rig|anim|fx|cfx|light|comp|lookdev|shotmodel)$"));
        let subcontext = graph.add_node(jstnode!("subcontext", r"^[a-z]+([_]{0,1}[a-z0-9])*$"));
        let bin = graph.add_node(jstnode!("bin"));
        let etc = graph.add_node(jstnode!("etc")); //0751 ddinst
        let lib = graph.add_node(jstnode!("lib")); //ddinst
        let lib_sd = graph.add_node(
            jstnode!("lib_sd", r"^(config|cortex|dmx|houdini|integ|jstools|katana|lw|massive|max|maya|mentalray|mkfoldy|moco|mova|nfb|nuke|perl|python[0-9.]*|race|refchef|rman|scratch|setupenv|shader|shoot2x|submission|vray|wam|web)$") // 0771
        );
        let prod = graph.add_node(jstnode!("prod")); // 755
        let docs = graph.add_node(jstnode!("docs")); // 0771
        let user = graph.add_node(jstnode!("user").set_volume()); //751
        let work = graph.add_node(jstnode!("work", r"^work\.[a-z]+$")); // 0770 default 0555
        let outsource = graph.add_node(jstnode!("OUTSOURCE").set_volume());
        let outsource_sd = graph.add_node(jstnode!("outsource_sd", r"^[a-zA-Z0-9_.]+$")); //perms default 555
        let outsource_sdd = graph.add_node(
            jstnode!( "outsource_sdd", r"[a-zA-Z0-9_.]+^$", r"^prod$")
        ); // 0770 (?!(\bprod\b))
        let finals = graph.add_node(jstnode!("FINALS")); // 750
        let finals_sd = graph.add_node(jstnode!("finals_sd", r"[0-9_]+"));
        let conform = graph.add_node(jstnode!("CONFORM"));
        let conform_sd =graph.add_node(jstnode!("conform_sd", r"^[a-z0-9_]+$"));
        // conform can also have SHARED as subdir as well as user docs and prod

        let artdept = graph.add_node(jstnode!("ARTDEPT"));
        let artdept_sd = graph.add_node(jstnode!("artdept_sd", r"^[a-zA-Z0-9_.-]+$")); //0770
        let storyboard = graph.add_node(jstnode!("STORYBOARD"));
        let storyboard_sd = graph.add_node(
            jstnode!("storyboard_sd", r"^[0-9]{2}_[0-9]{4}$")
        );// 0770
        let editorial = graph.add_node(jstnode!("STORYBOARD"));
        let film_lens = graph.add_node(jstnode!("film_lens", r"^(FILM|LENS)$"));
        let dailies = graph.add_node(jstnode!("DAILIES"));

        let shared = graph.add_node(jstnode!("SHARED"));
        let shared_dirs = graph.add_node(jstnode!("shared_dirs", r"^(PREVIZ|INTEG|MODEL|RIG|ANIM|CFX|LIGHT|ENVIRO|FX|COMP|IMG)$"));
        let assetdev = graph.add_node(jstnode!("ASSETDEV"));
        let adshot = graph.add_node(jstnode!("assetdev shot", r"^([A-Z][A-Z0-9]+[_]{0,1})+[A-Z0-9]+$"));
        let sequence = graph.add_node(
            jstnode!(
                "sequence", r"^(([A-Z]{2,4})|LIBRARY)$", r"^(SHARED|etc|lib|tool|user|bin)$")
        );
        let shot = graph.add_node(jstnode!("shot", r"^[0-9]+[A-Z0-9]*$"));

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
            (show, assetdev),
            (assetdev, adshot),
            (sequence, tools),
            (sequence, etc),
            (sequence, shared),
            (sequence, user),
            (sequence, shot),

        ]);

        graph.extend_with_edges(&[
            (shot, tools),
            (shot, etc),
            (shot, shared),
            (shot, user),
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
        assert!(is_valid(p, &tgraph).is_success());
    }

    #[test]
    fn shot_is_valid_graph() {
        let tgraph = build_graph();
        let p = "/dd/shows/DEV01/RD/9999/SHARED/MODEL";

        assert!(is_valid(p, &tgraph).is_success());
    }

    #[test]
    fn wrong_path_is_invalid_graph() {
        let tgraph = build_graph();
        let p = "/dd/shows/DEV01/RD/9999/FOO/SHARED/MODEL";
        assert!(is_valid(p, &tgraph).is_failure());
    }
}
