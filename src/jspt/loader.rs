use crate::jspt::{
    StateMachine,
    JSPTemplateLineError,
    JSPTemplateError,
    //State,
    ParseResult,
    JsptRegex,
    Node as SNode,
    Edge,
    JsptMetadata
};
use crate::{JGraph, NIndex, User, Node, Regexp, EntryType, NodeType, Metadata as JspMetadata };
use log;
use std::{io::BufRead, collections::HashMap};


/// s! calls to_string() on its input
#[macro_use]
pub mod macros {
    macro_rules! s {
        ($val: expr) => {
            $val.to_string();
        }
    }
}

// Is there any reason to make this a public trait?
// A bit of sugar to allow us to call is_volume
// as a method on Option<metadata> instead of having to define
// this as a loose function. 
pub trait IsVolume {
    fn is_volume(&self) -> bool;
}

impl IsVolume for &Option<JsptMetadata> {
    fn is_volume(&self) ->bool {
        if let Some(meta) = self {
        meta.is_volume()
        } else {
            false
        }
    }
}

/// A HashMap which associates names with Node `NIndex`s. This is used to
/// build a JGraph after successfully parsing a jsptemplate.
pub type JGraphKeyMap = HashMap<String, NIndex>;
/// A HashMap which associates names with `NodeType`s. This is used to build
/// a JGraph instance after successfully parsing the jsptemplate.
pub type RegexMap     = HashMap<String, NodeType>;

/// Loader is responsible for loading the jspt  from something that implements
/// the BufRead interace (like a buffered file or a Cursor) and producing
/// a populated JGraph. 
/// As a note, the reason why we hold mutable references to graph, keymap and regexmap
/// is so we (A) dont have to pass them into all of the methods, and (B) so that 
/// the graph may outlive the loader.
/// 
/// There are a couple of alternatives worth exploring from a design perspective, assuming
/// we dont want the graph, keymap and regexmap to stick around after loading:
/// 
/// (1) hold owned instances, and swap the JGraph with a blank one at the end of the load
/// method, which would return a JGraph. We would also like to clear the KeyMap and RegexMap
/// in that case, as they would be "dirty". Alternatively, we could have a dirty flag
/// for each and check that at the beginning of load, only clearing them in the event that
/// the dirty flag is true. I don't forsee actually using the loader multiple times,
/// at least not in the cli, but other uses are possible (like a gui) so...
/// 
/// (2) make the values all Option<T>, and use take to unwrap their inner value. 
/// 
/// However, I believe that there are reasons to stick with the current architecture. 
/// For one, should we decide to encorporate direct loading of the jstemplate instead of 
/// conversion as a preprocess, the structures would be quite useful for retrieval of arbitrary
/// nodes. For instance, if we want to be able to get all shot nodes, that becomes an O(1) 
/// operation.
pub struct Loader<'a> {
    graph: &'a mut JGraph,
    keymap: &'a mut JGraphKeyMap,
    regexmap: &'a mut RegexMap,
}

impl<'a> Loader<'a> {

    /// Setup data structures prior to newing up Loader 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jspcompile::Loader;
    /// 
    /// let (mut graph, mut keymap, mut regexmap) = Loader::setup();
    /// let loader = Loader::new(&mut graph, &mut keymap, &mut regexmap);
    /// ```
    pub fn setup() -> (JGraph, JGraphKeyMap, RegexMap) {
        let graph = JGraph::new();
        let keymap = JGraphKeyMap::new();
        let regexmap = RegexMap::new();
        (graph, keymap, regexmap)
    }

    /// Instantiate a new Loader, given a mutable JGraph reference, along with 
    /// mutable references toJGraphKeyMap and RegexMap instances.
    pub fn new(graph: &'a mut JGraph, keymap: &'a mut JGraphKeyMap, regexmap: &'a mut RegexMap) -> Self {
        // add in the root node
        keymap.insert(s!("root"), graph.add_node(Node::new_root()));

        Self {
            graph, keymap, regexmap
        }
    }

    /// Load the jspt data via the reader.
    /// 
    /// # Parameters
    /// 
    /// * `reader` - a type which implements BufRead and which supplies the lines to be parsed. 
    ///
    /// # Returns
    /// A Result wrapping a unit if successful. Otherwise a JSPTemplateError. 
    /// 
    /// # Examples
    /// TBD
    pub fn load<R>(&mut self, reader: R) -> Result<(), JSPTemplateError> 
    where
        R: BufRead
    {
        let mut statemachine = StateMachine::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                match statemachine.parse(&line) {
                    Ok(v) => {
                        match v {
                            ParseResult::Empty => {}
                            ParseResult::Header(header) => {log::info!("Loader::load(...) line: {} {:?}", statemachine.line_number(), header)}

                            ParseResult::Comment(comment) =>{log::debug!("Loader::load(...) line: {} {}", statemachine.line_number(), comment)}

                            ParseResult::Node(node) => {
                                log::info!("Loader::load(...) line: {} {:?}", statemachine.line_number(), node);
                                self.process_node(node, line.as_str(), &statemachine)?;
                            }

                            ParseResult::Regex(regex) => {
                                log::info!("Loader::load(...) line: {} {:?}", statemachine.line_number(), regex);
                                self.process_regex(regex)?;
                            }
                            ParseResult::Edges(edges) => {
                                log::info!("Loader::load(...) line: {} {:?}", statemachine.line_number(), edges);
                                
                                // deal with root
                                // this is not where this belongs
                               // if edges.len() > 0 && edges[0].from != "root" {
                                //    let root = Edge::new(s!("root"), edges[0].from.clone());
                                //    self.process_edges(vec![root],line.as_str(), &statemachine)?;
                                //}
                                self.process_edges(edges, line.as_str(), &statemachine)?;
                            }
                        }
                    },
                    Err(e) => {
                        return Err(JSPTemplateError::from(e))
                    },
                }
            } 
        }
        Ok(())
    }

    // Process a vector of edges supplied by the parsing of a line of hte jsptemplate. 
    // We provide the line and statemachine for context in the case of failure. 
    fn process_edges(&mut self, edges: Vec<Edge>, line: &str, statemachine: &StateMachine) -> Result<(), JSPTemplateError> {
        for edge in edges {
            log::debug!("Loader::process_edges(...) Adding edge for {:?}", &edge);
            let from_node = self.keymap.get(&edge.from).ok_or_else(||
                JSPTemplateLineError::from((
                    statemachine.line_number(),
                    line.to_owned(),
                    statemachine.state().clone(),
                    JSPTemplateError::KeyMapLookupError(edge.from.clone())
                ))
            )?;
            let to_node = self.keymap.get(&edge.to).ok_or_else(||
                JSPTemplateLineError::from((
                    statemachine.line_number(),
                    line.to_owned(),
                    statemachine.state().clone(),
                    JSPTemplateError::KeyMapLookupError(edge.to.clone())
                ))
            )?;
            self.graph.extend_with_edges(&[(from_node.clone(), to_node.clone())]);
        }
        Ok(())
    }

    // Process a node, generated by the StateMachine's parsing of an appropriate line
    fn process_node(&mut self, node: SNode, line: &str, statemachine: &StateMachine) -> Result<(), JSPTemplateError> {
        match node {
            // `rd`
            SNode::Simple(ref name, ref metadata) => {
                //let entrytype = if is_volume(metadata) {EntryType::Volume} else {EntryType::Directory};
                let entrytype = if metadata.is_volume() {EntryType::Volume} else {EntryType::Directory};

                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            NodeType::Simple(name.clone()),
                            entrytype,
                            new_jsp_metadata(metadata)
                        )
                    )
                );
            }
            // `rd = RD`
            SNode::Pair{ref name, ref value, ref metadata} => {
                //let entrytype = if is_volume(metadata) {EntryType::Volume} else {EntryType::Directory};
                let entrytype = if metadata.is_volume() {EntryType::Volume} else {EntryType::Directory};

                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            NodeType::Simple(value.clone()),
                            entrytype,
                            new_jsp_metadata(metadata)
                        )
                    )
                );
            }
            // `rd = $rd_re`
            SNode::ReVar{ref name, ref variable, ref metadata} => {
                let var = self.regexmap.get(variable).ok_or_else(||
                    JSPTemplateLineError::from((
                        statemachine.line_number(),
                        line.to_owned(),
                        statemachine.state().clone(),
                        JSPTemplateError::RegexMapLookupError(variable.clone()
                    ))
                ))?;
                //let entrytype = if is_volume(metadata) {EntryType::Volume} else {EntryType::Directory};
                let entrytype = if metadata.is_volume() {EntryType::Volume} else {EntryType::Directory};
                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            var.clone(),
                            entrytype,
                            new_jsp_metadata(metadata)
                        )
                    )
                );
            } 
            // `rd = $$rd_re`
            SNode::EnvVar{ref name, ref variable, ref metadata} => {
                let var = std::env::var(variable).or_else(|_errval|
                    Err(JSPTemplateLineError::from((
                        statemachine.line_number(),
                        line.to_owned(),
                        statemachine.state().clone(),
                        JSPTemplateError::EnvVarLookupError(variable.clone()
                    )))
                ))?;
                log::trace!("Loader::process_node(...) Looked up EnvVar: {} and found {}", name, &var);
                let entrytype = if metadata.is_volume() {EntryType::Volume} else {EntryType::Directory};
                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            NodeType::Simple(var.clone()),
                            entrytype,
                            new_jsp_metadata(metadata)
                        )
                    )
                );
            } 
            // `rd = "[a-z]+"`
            SNode::RegexSimple{ref name, ref re, ref metadata} => {
                let regx = Regexp::new(format!("^{}$", re.as_str()).as_str())?;
                //let entrytype = if is_volume(metadata) {EntryType::Volume} else {EntryType::Directory};
                let entrytype = if metadata.is_volume() {EntryType::Volume} else {EntryType::Directory};

                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            NodeType::new_regex( name.clone(), regx, None),
                            entrytype,
                            new_jsp_metadata(metadata)
                        )
                    )
                );
            }
            // `rd = "[a-z]+" "(foo|bar)"`
            SNode::RegexComplex{ref name, ref pos, ref neg, ref metadata} => {
                let regx_pos = Regexp::new(format!("^{}$", pos.as_str()).as_str())?;
                let regx_neg = Regexp::new(format!("^{}$", neg.as_str()).as_str())?;
                //let entrytype = if is_volume(metadata) {EntryType::Volume} else {EntryType::Directory};
                let entrytype = if metadata.is_volume() {EntryType::Volume} else {EntryType::Directory};

                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            NodeType::new_regex( name.clone(), regx_pos, Some(regx_neg)),
                            entrytype,
                            new_jsp_metadata(metadata)
                        )
                    )
                );
            }
        };

        Ok(())
    }

    // Process a regular expression (Regex) generated by the StateMachine after parsing the current
    // line.
    // match against the various flavors or regex and construct Regex objects in the regexmap store
    // these will be used in node later.
    fn process_regex(&mut self, regex: JsptRegex)-> Result<(), JSPTemplateError> {
        match regex {

            JsptRegex::Simple{ ref name,  ref value} => {
                let re = Regexp::new(format!("^{}$", value.as_str()).as_str())?;
                self.regexmap.insert(name.clone(), NodeType::new_regex( name.clone(), re, None));
            }

            JsptRegex::Complex{ ref name, ref positive, ref negative} => {
                let pos_re = Regexp::new(format!("^{}$", positive.as_str()).as_str())?;
                let neg_re = Regexp::new(format!("^{}$", negative.as_str()).as_str())?;
                self.regexmap.insert(name.clone(), NodeType::new_regex(name.clone(), pos_re, Some(neg_re)));
            }
        }
        Ok(())
    }
}

/**
 * FUggly 
 * 
 * This bit of conversion will go away once I unify the two metadata representations
 */
fn new_jsp_metadata( meta: &Option<JsptMetadata> ) -> JspMetadata {
    let mut jspmeta = JspMetadata::new();

    if let Some(meta) = meta {

        if meta.owner().is_some() { 
            let owner = meta.owner().unwrap();
            jspmeta.set_owner(
                Some(
                    User::from(owner.to_string()) 
                )
            );
        }

        if meta.permissions().is_some() {
            let perms = meta.permissions().unwrap();
            jspmeta.set_perms(Some(perms.to_string()));
        }

        if meta.varname().is_some() {
            let varname = meta.varname().unwrap();
            jspmeta.set_varname(Some(varname.to_string()));
        }

        jspmeta.set_autocreate(meta.is_autocreate());
        
    }
    jspmeta
}

//
// Replaced with a trait that provides a bit of sugar, allowing
// us to call is_volume as a method on Option<Metadata>
//
// // check if metadata has option
// fn is_volume(meta: &Option<Metadata>) -> bool {
//     if let Some(meta) = meta {
//         meta.is_volume()
//     } else {
//         false
//     }
// }

