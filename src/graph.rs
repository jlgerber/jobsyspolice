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
        let dd = graph.add_node(Node::from_str("dd").unwrap());
        let shows = graph.add_node(Node::from_str("shows").unwrap());
        let show = graph.add_node(Node::new_regexp("show", r"^[A-Z]+[A-Z0-9]*$", None));
        let mut refdir = Node::from_str("REF").unwrap();

        // ref
        refdir.set_volume();
        let refdir = graph.add_node(refdir);
        let quicktimes = graph.add_node(Node::from_str("quicktimes").unwrap());
        let qtsubdir = graph.add_node(Node::new_regexp("qtsubdir", r"^[0-9_]+$", None));
        let mut clientvault = Node::from_str("CLIENT_VAULT").unwrap();
        clientvault.set_volume();
        let clientvault = graph.add_node(clientvault);
        let clientvaultsd = graph.add_node(Node::new_regexp("clientvault_subdir", r"^(incoming|outgoing)$", None));
        let clientvaultssd = graph.add_node(Node::new_regexp("clientvault_ssd", r"^[0-9_]+$", None));
        let slates_n_categories = graph.add_node(Node::new_regexp("slatesNcategories", r"(SLATES|CATGORIES)^$", None));
        let snc_sd = graph.add_node(Node::new_regexp("snc_sd", r"^[a-z0-9_.-]+$", None));
        let locations = graph.add_node(Node::from_str("LOCATIONS").unwrap());
        let loc_sd = graph.add_node(Node::new_regexp("loc_sd", r"^[a-z0-9_.-]+$", None));
        let loc_ssd = graph.add_node(Node::new_regexp("loc_ssd", r"^[a-z0-9_.-]+$", None));
        let documents = graph.add_node(Node::from_str("documents").unwrap());
        let doc_sd = graph.add_node(Node::new_regexp("doc_sd", r"^(agency|director_treatments|vfx_methodology|shcedules|scripts|storyboards)$", None));
        let audio = graph.add_node(Node::from_str("audio").unwrap());
        let audio_sd = graph.add_node(Node::new_regexp("audio_sd", r"^(mixes|sources)$", None));
        let threed = graph.add_node(Node::from_str("3d").unwrap());
        let threed_sd = graph.add_node(Node::new_regexp("3d_sd", r"^(3d_assets|mocap)$", None));
        let chars = graph.add_node(Node::from_str("CHARACTERS").unwrap());
        let chars_sd = graph.add_node(
            Node::new_regexp_adv(
                "chars_sd",
                r"^[a-z0-9_]+$",
                r"^(DEVL|SHARED|etc|lib|bin|user)$",
                None)
        );

        // SHOW
        let mut client_dd_edit = Node::new_regexp("client_dd_edit", r"^(CLIENT|DD)$", None);
        client_dd_edit.set_volume();
        let client_dd_edit = graph.add_node(client_dd_edit);
        let client_dd_edit_sd = graph.add_node(
            Node::new_regexp("client_dd_edit_sd",r"^(([0-9]{4,5})|([0-9]{1,2}?[a-z]+)|([a-z]{2}[0-9]{4,5}))$",None)
        );
        let tools = graph.add_node(Node::from_str("tools").unwrap()); // 0751 ddinst
        let logs = graph.add_node(Node::from_str("logs").unwrap()); // 0771
        let package = graph.add_node(Node::from_str("package").unwrap());
        let extension = graph.add_node(Node::from_str("extension").unwrap());
        let color = graph.add_node(Node::from_str("color").unwrap());
        let category = graph.add_node(Node::new_regexp("category", r"^(char|prop|veh|scene|enviro|kit)$", None));
        let dept = graph.add_node(Node::new_regexp("department", r"^(integ|model|previz|postviz|enviro|rig|anim|fx|cfx|light|comp|lookdev|shotmodel)$", None));
        let subcontext = graph.add_node(Node::new_regexp("subcontext", r"^[a-z]+([_]{0,1}[a-z0-9])*$", None));
        let bin = graph.add_node(Node::from_str("bin").unwrap());
        let etc = graph.add_node(Node::from_str("etc").unwrap()); //0751 ddinst
        let lib = graph.add_node(Node::from_str("lib").unwrap()); //ddinst
        let lib_sd = graph.add_node(
            Node::new_regexp("lib_sd", r"^(config|cortex|dmx|houdini|integ|jstools|katana|lw|massive|max|maya|mentalray|mkfoldy|moco|mova|nfb|nuke|perl|python[0-9.]*|race|refchef|rman|scratch|setupenv|shader|shoot2x|submission|vray|wam|web)$", None) // 0771
        );
        let prod = graph.add_node(Node::from_str("prod").unwrap()); // 755
        let docs = graph.add_node(Node::from_str("docs").unwrap()); // 0771
        let mut user = Node::from_str("user").unwrap(); // 751
        user.set_volume();
        let user = graph.add_node(user);
        let work = graph.add_node(Node::new_regexp("work", r"^work\.[a-z]+$", None)); // 0770 default 0555
        let mut outsource = Node::from_str("OUTSOURCE").unwrap();
        outsource.set_volume();
        let outsource = graph.add_node(outsource);
        let outsource_sd = graph.add_node(Node::new_regexp("outsource_sd", r"^[a-zA-Z0-9_.]+$", None)); //perms default 555
        let outsource_sdd = graph.add_node(
            Node::new_regexp_adv(
                "outsource_sdd",
                r"[a-zA-Z0-9_.]+^$",
                r"^prod$",
                None)
        ); // 0770 (?!(\bprod\b))
        let finals = graph.add_node(Node::from_str("FINALS").unwrap()); // 750
        let finals_sd = graph.add_node(Node::new_regexp("finals_sd", r"[0-9_]+", None));
        let conform = graph.add_node(Node::from_str("CONFORM").unwrap());
        let conform_sd =graph.add_node(Node::new_regexp("conform_sd", r"^[a-z0-9_]+$", None));
        // conform can also have SHARED as subdir as well as user docs and prod

        let artdept = graph.add_node(Node::from_str("ARTDEPT").unwrap());
        let artdept_sd = graph.add_node(Node::new_regexp("artdept_sd", r"^[a-zA-Z0-9_.-]+$", None)); //0770
        let storyboard = graph.add_node(Node::from_str("STORYBOARD").unwrap());
        let storyboard_sd = graph.add_node(
            Node::new_regexp("storyboard_sd", r"^[0-9]{2}_[0-9]{4}$", None)
        );// 0770
        let editorial = graph.add_node(Node::from_str("STORYBOARD").unwrap());
        let film_lens = graph.add_node(Node::new_regexp("film_lens", r"^(FILM|LENS)$", None));
        let dailies = graph.add_node(Node::from_str("DAILIES").unwrap());

        let shared = graph.add_node(Node::from_str("SHARED").unwrap());
        let shared_dirs = graph.add_node(Node::new_regexp("shared_dirs", r"^(PREVIZ|INTEG|MODEL|RIG|ANIM|CFX|LIGHT|ENVIRO|FX|COMP|IMG)$", None));
        let assetdev = graph.add_node(Node::from_str("ASSETDEV").unwrap());
        let adshot = graph.add_node(Node::new_regexp("assetdev shot", r"^([A-Z][A-Z0-9]+[_]{0,1})+[A-Z0-9]+$", None));
        let sequence = graph.add_node(
            Node::new_regexp_adv(
                "sequence",
                r"^(([A-Z]{2,4})|LIBRARY)$",
                r"^(SHARED|etc|lib|tool|user|bin)$",
                None)
        );
        let shot = graph.add_node(Node::new_regexp("shot", r"^[0-9]+[A-Z0-9]*$", None));

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
