/// The `Edge` connects two `Node`s in the `JGraph`. 
/// It stores the labels of two nodes that it connects, allowing for
///  later retrieval. 
#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    pub from: String,
    pub to: String,
}

impl Edge {

    /// New up an `Edge` instance providing the names of the `from` and `to` 
    /// `Node`s that it connects.
    /// 
    /// # Parameters
    /// 
    /// * `from` - The name of the upstream `Node` that this `Edge` connects. 
    /// * `to`   - The name of the downstream `Node` that this `Edge` connects. 
    /// 
    /// # Returns 
    /// A new `Node` instance
    pub fn new<I>(from: I, to: I) -> Edge 
    where
        I: Into<String>
    {
        Edge{
            from: from.into(),
            to: to.into()
        }
    }
}