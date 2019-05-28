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
        let show = graph.add_node(Node::new_regexp("show", r"^[A-Z]+[A-Z 0-9]*$", None));
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
        let chars_sd = graph.add_node(Node::new_regexp("chars_sd",r"^[a-z0-9_]+$", None));
        // the full chars regexp has a lookahead negative which the regexp lib does not support.
        // TODO: add an optional Negative expression to regexp that will reject matches
        // RegexpWneg

        let tools = graph.add_node(Node::from_str("tools").unwrap());
        let package = graph.add_node(Node::from_str("package").unwrap());
        let extension = graph.add_node(Node::from_str("extension").unwrap());
        let color = graph.add_node(Node::from_str("COLOR").unwrap());
        let category = graph.add_node(Node::new_regexp("category", r"^(char|prop|veh|scene|enviro|kit)$", None));
        let dept = graph.add_node(Node::new_regexp("department", r"^(integ|model|previz|postviz|enviro|rig|anim|fx|cfx|light|comp|lookdev|shotmodel)$", None));
        let subcontext = graph.add_node(Node::new_regexp("subcontext", r"^[a-z]+([_]{0,1}[a-z0-9])*$", None));
        let bin = graph.add_node(Node::from_str("bin").unwrap());
        let etc = graph.add_node(Node::from_str("etc").unwrap());
        let user = graph.add_node(Node::from_str("user").unwrap());
        let shared = graph.add_node(Node::from_str("SHARED").unwrap());
        let shared_dirs = graph.add_node(Node::new_regexp("shared_dirs", r"^(PREVIZ|INTEG|MODEL|RIG|ANIM|CFX|LIGHT|ENVIRO|FX|COMP|IMG)$", None));
        let work = graph.add_node(Node::new_regexp("work", r"^work\.[a-z]+$", None));
        let sequence = graph.add_node(Node::new_regexp("sequence", r"^(([A-Z]{2,4})|LIBRARY)$", None));
        let adsequence = graph.add_node(Node::from_str("ASSETDEV").unwrap());
        let shot = graph.add_node(Node::new_regexp("shot", r"^[0-9]+[A-Z0-9]*$", None));
        let adshot = graph.add_node(Node::new_regexp("assetdev shot", r"^([A-Z][A-Z0-9]+[_]{0,1})+[A-Z0-9]+$", None));

        graph.extend_with_edges(&[
            (root, dd),
            (dd, shows),
            (shows, show),
            (show, tools),
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
            (show, shared),
            (shared, etc),
            (shared, shared_dirs),
        ]);
        // split it up because there appears to be
        // a max size for &[]
        graph.extend_with_edges(&[
            (shared_dirs, category),
            (category, dept),
            (dept, subcontext),
            (show, sequence),
            (show, adsequence),
            (sequence, tools),
            (sequence, etc),
            (sequence, shared),
            (sequence, user),
            (sequence, shot),
            (adsequence, tools),
            (adsequence, etc),
            (adsequence, shared),
            (adsequence, user),
            (adsequence, adshot),
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
