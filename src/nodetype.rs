
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum NodeType {
   Directory,
   Volume,
   Root,
}