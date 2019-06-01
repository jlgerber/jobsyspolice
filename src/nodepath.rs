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
    /// let mut graph = JGraph::new();
    /// let node = Node::new_root();
    /// let idx = graph.add_node(node);
    /// let idxvec = vec![idx];
    /// let np = NodePath::new(&graph).append(np);
    ///
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
    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    /// New up a nodepath
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
    /// let mut graph = JGraph::new();
    /// let niv = vec![Node::new_root()];
    /// let mut np = NodePath::new(&graph).replace_nodes_unchecked(niv);
    /// ```
    pub fn replace_nodes_unchecked(mut self, n: Vec<NIndex>) -> Self {
        self.nodes = n;
        self
    }

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
    /// let graph = JGraph::new();
    /// let mut np = NodePath::new(&graph);
    /// let n = NIndex::new(0);
    /// assert_eq!(np.push(n), false);
    /// ```
    pub fn push(&mut self, node: NIndex) -> Result<(), JSPError> {
        if !self.graph.node_indices().any(|x| x == node) {
            return Err(JSPError::MissingIndex(node));
        }
        self.nodes.push(node);
        Ok(())
    }

    ///
    pub fn pop(&mut self) -> Option<NIndex> {
        self.nodes.pop()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn count(&self) -> usize {
        self.nodes.len()
    }
    // NodePathIntoIterator consumes NodePath
    pub fn into_iter(&'a mut self) -> NodePathIntoIterator<'a> {
        NodePathIntoIterator{nodepath: self, index: 0}
    }

    /// Returns true if the vector contains no elements.
    ///
    /// # Examples
    ///
    /// ```
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