use crate::Node;
use crate::NIndex;
use crate::JGraph;

#[derive(Debug, PartialEq, Eq)]
pub struct NodePath {
    pub nodes: Vec<Node>
}

impl NodePath {

    /// New up a nodepath
    pub fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    pub fn push(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn pop(&mut self) -> Option<Node> {
        self.nodes.pop()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    // NodePathIntoIterator consumes NodePath
    pub fn into_iter(&mut self) -> NodePathIntoIterator {
        NodePathIntoIterator{nodepath: self, index: 0}
    }

    // NodePathIter returns reference to nodes
    pub fn iter(&self) -> NodePathIterator {
        NodePathIterator{nodepath: self, index: 0}
    }
}

pub struct NodePathIntoIterator<'a> {
    nodepath: &'a mut NodePath,
    index: usize,
}

impl<'a> Iterator for NodePathIntoIterator<'a> {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        if self.index >= self.nodepath.len() {
            return None;
        }
        let result = self.nodepath.nodes.pop();
        self.index += 1;
        result
    }
}


pub struct NodePathIterator<'a> {
    nodepath: &'a  NodePath,
    index: usize,
}

impl<'a> Iterator for NodePathIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.nodepath.len() {
            return None;
        }
        let result = &self.nodepath.nodes[self.index];
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