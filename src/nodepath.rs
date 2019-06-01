use crate::Node;
use crate::NIndex;
use crate::JGraph;
use crate::JSPError;

/// The NodePath stores a path of nodes in the JGraph. The nodes
/// are represented internally as `NIndex`s. One may generate an
/// iterator from the NodePath.
#[derive(Debug)]
pub struct NodePath<'a> {
    pub graph: &'a JGraph,
    pub nodes: Vec<NIndex>

}

impl<'a> NodePath<'a> {

    /// Moves all the elements of other into Self, leaving other empty.
    /// `append_unchecked` does this without checking the internal `JGraph`
    /// to ensure that the `NIndex`s are known to the `JGraph` instance.
    /// Usage of this method should be done with care, as subsequent iteration
    /// over the NodePath will panic if we encounter an unknown NIndex. For
    /// untrusted cases, there is `append`.
    ///
    /// # Parameters
    ///
    /// * `other` - A mutable reference to a vector of `NIndex` which we wish to
    ///             append to our NodePath's existing `NIndex`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsp::{ JGraph, Node, jspnode, NodePath };
    /// let mut graph = JGraph::new();
    /// let node = Node::new_root();
    /// let idx = graph.add_node(node);
    /// let mut idxvec = vec![idx];
    /// let np = NodePath::new(&graph).append_unchecked(&mut idxvec);
    ///```
    pub fn append_unchecked(&mut self, other: &mut Vec<NIndex>) {
        self.nodes.append(other);
    }

    /// Moves all the elements of other into Self, leaving other empty, assuming
    /// all the NIndexes in other exist in self.graph. If that isn't the case,
    /// append bails early.
    ///
    /// # Parameters
    /// * `other` - a mutable reference to a vector of NIndex which we wish to append
    ///             to self.
    /// # Returns
    ///   bool indicating success / failure
    ///
    /// # Examples
    /// ```
    /// use jsp::{ JGraph, Node, NodePath };
    /// let mut graph = JGraph::new();
    /// let node = Node::new_root();
    /// let idx = graph.add_node(node);
    /// let mut idxvec = vec![idx];
    /// let np = NodePath::new(&graph).append(&mut idxvec).unwrap();
    /// ```
    pub fn append(mut self, other: &mut Vec<NIndex>) -> Result<Self, JSPError> {
        for nd in other.iter() {
            if !self.graph.node_indices().any(|x| &x == nd) {
                return Err(JSPError::MissingIndex(nd.clone()));
            }
        }
        self.nodes.append(other);
        Ok(self)
    }

    /// Removes all nodes from the NodePath
    ///
    /// # Examples
    ///
    /// ```
    /// use jsp::{ JGraph, Node, jspnode, NodePath };
    /// let mut graph = JGraph::new();
    /// let node = Node::new_root();
    /// let idx = graph.add_node(node);
    /// let mut idxvec = vec![idx];
    /// let mut np = NodePath::new(&graph).append(&mut idxvec).unwrap();
    /// assert_eq!(np.count(), 1);
    ///
    /// np.clear();
    /// assert_eq!(np.count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    /// New up a nodepath
    ///
    /// # Parameters
    ///
    /// * `graph` - reference to a `JGraph`
    ///
    /// # Examples
    ///
    /// ```
    /// use jsp::{ JGraph, NodePath };
    /// let mut graph = JGraph::new();
    /// let np = NodePath::new(&graph);
    /// ```
    pub fn new(graph: &'a JGraph) -> Self {
        Self {
            graph,
            nodes: Vec::new()
        }
    }

    /// Replace the interally stored nodes with this new
    /// vector of `NIndex`s
    ///
    /// # Returns
    /// Self, so that the call may be chained.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsp::{ JGraph, Node, NIndex, jspnode, NodeType, EntryType, NodePath };
    ///
    /// let mut graph = JGraph::new();
    /// let mut niv = vec![Node::new_root(),jspnode!("FOO"), jspnode!("BAR")];
    /// let mut idx = niv.drain(0..niv.len()).map(|x| graph.add_node(x)).collect::<Vec<NIndex>>();
    /// idx.pop();
    ///
    /// let np = NodePath::new(&graph).replace_nodes_unchecked(idx);
    /// ```
    pub fn replace_nodes_unchecked(mut self, n: Vec<NIndex>) -> Self {
        self.nodes = n;
        self
    }

    /// Replace internal NIndex nodes with vector of new nodes. This is useful
    /// for instance when updating the path represented by NodePath. This
    /// method validates that the `NIndex`s are all known by the internal `JGraph`.
    ///
    /// # Parameters
    ///
    /// * `n` - a vector of `NIndex`s
    ///
    /// # Returns
    ///
    /// A Result wrapping Self if all `NIndex`s are known to the `JGraph`, or
    /// JSPError::MissingIndex if any `NIndex`s are not known to the `JGraph`
    ///
    /// # Examples
    ///
    /// ```
    /// use jsp::{ JGraph, Node, jspnode, NIndex, NodePath, EntryType, NodeType };
    /// let mut graph = JGraph::new();
    /// let mut niv = vec![Node::new_root(), jspnode!("grb"), jspnode!("shows"), jspnode!("FLUF"), jspnode!("FLARG")];
    /// let ids = niv.drain(0..niv.len()).map(|x| graph.add_node(x)).collect::<Vec<NIndex>>();
    /// let mut np = NodePath::new(&graph).replace_nodes(ids);
    /// ```
    pub fn replace_nodes(mut self, n: Vec<NIndex>) -> Result<Self, JSPError> {
        for nd in &n {
            if !self.graph.node_indices().any(|x| &x == nd) {
                return Err(JSPError::MissingIndex(nd.clone()));
            }
        }
        self.nodes = n;
        Ok(self)
    }

    /// Add an NIndex into the nodeindex. This method does not
    /// check to see if the node matches one in self.graph, and should
    /// only be used in cases where one is certain that this holds true.
    /// Otherwise, iteration could panic.
    ///
    /// # Examples
    /// ```
    /// use jsp::{ JGraph, Node, NodePath };
    /// let mut graph = JGraph::new();
    /// let ni = Node::new_root();
    /// let idx = graph.add_node(ni);
    /// let mut np = NodePath::new(&graph);
    ///
    /// np.push(idx);
    /// ```
    pub fn push_unchecked(&mut self, node: NIndex) {
        self.nodes.push(node);
    }

    /// Push a node into the NodePath if it is contained within
    /// the graph. Note that this operation is O(n) as it uses
    /// a graph iterator to test if any NIndex node matches the
    /// provided one
    ///
    /// # Examples
    ///
    /// ```
    /// use jsp::{ JGraph, jspnode, Node, NodeType, EntryType, NodePath };
    /// let mut graph = JGraph::new();
    /// let node = jspnode!("FOO");
    /// let idx = graph.add_node(node);
    ///
    /// let mut np = NodePath::new(&graph);
    /// let result = np.push(idx);
    /// assert_eq!(result.is_ok(), true);
    /// ```
    pub fn push(&mut self, node: NIndex) -> Result<(), JSPError> {
        if !self.graph.node_indices().any(|x| x == node) {
            return Err(JSPError::MissingIndex(node));
        }
        self.nodes.push(node);
        Ok(())
    }

    /// Remove the last node index from the NodePath and
    /// return it as an Option. If NodePath is empty, return None.
    pub fn pop(&mut self) -> Option<NIndex> {
        self.nodes.pop()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of nodes in the NodePath
    pub fn count(&self) -> usize {
        self.nodes.len()
    }

    /// NodePathIntoIterator consumes NodePath
    pub fn into_iter(&'a mut self) -> NodePathIntoIterator<'a> {
        NodePathIntoIterator{nodepath: self, index: 0}
    }

    /// Returns true if the vector contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsp::{JGraph, NodePath};
    ///
    /// let graph = JGraph::new();
    /// let np = NodePath::new(&graph);
    /// assert!(np.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    // NodePathIter returns reference to nodes
    pub fn iter(&self) -> NodePathIterator {
        NodePathIterator{nodepath: self, index: 0}
    }

    /// Validate that the node indices contained within
    /// this NodePath instance are all recognized by the internal JGraph
    ///
    /// # Returns
    ///
    /// bool indicating the validity of the NodeGraph
    pub fn validate(&self) -> bool {
        for ni in self.nodes.iter() {
            if !self.graph.node_indices().any(|x| &x == ni) {
                return false;
            }
        }
        true
    }
}

pub struct NodePathIntoIterator<'a> {
    nodepath: &'a mut NodePath<'a>,
    index: usize,
}

impl<'a> Iterator for NodePathIntoIterator<'a> {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        if self.index >= self.nodepath.len() {
            return None;
        }
        let idx = self.nodepath.nodes[self.index];
        let result = self.nodepath.graph[idx].clone();
        self.index += 1;
        Some(result)
    }
}


pub struct NodePathIterator<'a> {
    nodepath: &'a  NodePath<'a>,
    index: usize,
}

impl<'a> Iterator for NodePathIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.nodepath.len() {
            return None;
        }
        let idx = self.nodepath.nodes[self.index];
        let result = &self.nodepath.graph[idx];
        self.index += 1;
        Some(result)
    }
}

// NEED TO FIGURE THIS OUT
// pub struct NodePathMutIterator<'a> {
//     nodepath: &'a  mut NodePath,
//     index: usize,
// }

// impl<'a> Iterator for NodePathMutIterator<'a> {
//     type Item = &'a mut Node;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.index >= self.nodepath.len() {
//             return None;
//         }
//         let result: &'a mut Node = &'a mut self.nodepath.nodes[self.index];
//         self.index += 1;
//         Some(result)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::testdata::build_graph;

    #[test]
    fn can_replace_node_unchecked() {
        let graph = JGraph::new();
        let niv = vec![NIndex::new(0), NIndex::new(1)];
        let np = NodePath::new(&graph).replace_nodes_unchecked(niv);
        assert_eq!(np.count(), 2);
    }

    #[test]
    fn can_replace_node() {
        let mut graph = JGraph::new();
        let n1 = Node::new_root();
        let n1idx = graph.add_node(n1);
        let niv = vec![n1idx];
        let np = NodePath::new(&graph).replace_nodes(niv).unwrap();
        assert_eq!(np.count(), 1);
        assert_eq!(np.validate(), true);
    }

    #[test]
    fn can_push_unchecked() {
        let mut graph = JGraph::new();
        let ni = Node::new_root();
        let idx = graph.add_node(ni);
        let mut np = NodePath::new(&graph);

         np.push_unchecked(idx);
         assert_eq!(np.count(),1);
    }

    #[test]
    fn can_push() {
        let mut graph = JGraph::new();
        let ni = Node::new_root();
        let idx = graph.add_node(ni);
        let mut np = NodePath::new(&graph);

        assert_eq!(np.push(idx).is_ok(), true);
    }

    #[test]
    fn can_pop() {
        let mut graph = JGraph::new();
        let ni = Node::new_root();
        let idx = graph.add_node(ni);
        let mut np = NodePath::new(&graph);
        np.push_unchecked(idx);
        let result = np.pop();
        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn can_pop_empty() {
        let mut graph = JGraph::new();
        let ni = Node::new_root();
        let _idx = graph.add_node(ni);
        let mut np = NodePath::new(&graph);
        let result = np.pop();
        assert_eq!(result.is_some(), false);
    }

    #[test]
    fn can_replace_nodes_unchecked() {
        use crate::{ jspnode, EntryType, NodeType };
        let mut graph = JGraph::new();
        let mut niv = vec![Node::new_root(), jspnode!("grb"), jspnode!("shows"), jspnode!("FLUF"), jspnode!("FLARG")];
        let ids = niv.drain(0..niv.len()).map(|x| graph.add_node(x)).collect::<Vec<NIndex>>();
        let np = NodePath::new(&graph).replace_nodes_unchecked(ids);
        assert_eq!(np.count(), 5);
    }

    #[test]
    fn can_replace_nodes() {
        use crate::{ jspnode, EntryType, NodeType };
        let mut graph = JGraph::new();
        let mut niv = vec![Node::new_root(), jspnode!("grb"), jspnode!("shows"), jspnode!("FLUF"), jspnode!("FLARG")];
        let ids = niv.drain(0..niv.len()).map(|x| graph.add_node(x)).collect::<Vec<NIndex>>();
        let np = NodePath::new(&graph).replace_nodes_unchecked(ids);
        assert_eq!(np.count(), 5);
        assert_eq!(np.validate(), true);
    }

    #[test]
    fn can_measure_count() {
        use crate::{ jspnode, EntryType, NodeType };
        let mut graph = JGraph::new();
        let mut niv = vec![Node::new_root(), jspnode!("grb"), jspnode!("shows"), jspnode!("FLUF"), jspnode!("FLARG")];
        let ids = niv.drain(0..niv.len()).map(|x| graph.add_node(x)).collect::<Vec<NIndex>>();
        let np = NodePath::new(&graph).replace_nodes_unchecked(ids);
        assert_eq!(np.count(), 5);
    }

    #[test]
    fn is_empty_works() {
        use crate::{ jspnode, EntryType, NodeType };
        let mut graph = JGraph::new();
        let mut niv = vec![Node::new_root(), jspnode!("grb"), jspnode!("shows"), jspnode!("FLUF"), jspnode!("FLARG")];
        let ids = niv.drain(0..niv.len()).map(|x| graph.add_node(x)).collect::<Vec<NIndex>>();
        let np = NodePath::new(&graph);
        assert_eq!(np.is_empty(), true);
        let np = np.replace_nodes_unchecked(ids);
        assert_eq!(np.is_empty(), false);
    }


    #[test]
    fn into_iter_works() {
        let graph = build_graph();
        let mut np = NodePath::new(&graph);
        for x in graph.node_indices() {
            np.push(x).unwrap();
        }
        for x in np.iter() {
            println!("{:?}", x);
        }
        assert!(true);
    }
}