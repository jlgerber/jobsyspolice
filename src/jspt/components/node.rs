use crate::jspt::JsptMetadata;

/// Represents a Node in the JGraph as defined in the Node section of the
/// template. 
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    /// Represents a single identifier on a line in the node section of 
    /// the template. 
    /// EG
    /// `rd`
    Simple(String, Option<JsptMetadata>),

    /// Represents a named assignment on a line in the node section
    /// of the template. 
    /// EG
    ///  `rd = RD`
    Pair{name: String, value: String, metadata: Option<JsptMetadata>}, 

    /// Represents a variable assignment on a line in the node section
    /// of the template. The variable refers to a named regular expression 
    /// captured in the regex section of the template.
    /// EG
    /// `rd = $rd_re`
    ReVar{name: String, variable: String, metadata: Option<JsptMetadata>}, 

    /// Represents a simple inline regular expression on a line in the 
    /// node section of the template. 
    /// EG
    /// `rd = "[a-z]+"`
    RegexSimple{name: String, re: String, metadata: Option<JsptMetadata> },

    /// Represents a complex inline regular expression on a line in the 
    /// node section of the template. By complex, we mean that the regex
    /// has both a positive and negative regular expression. A match will
    /// only be valid if it both matches the positive and does not match 
    /// the netative regular expression. 
    /// EG
    /// `rd = "[a-z]+" "(foo|bar)"`
    RegexComplex{name:String, pos: String, neg: String, metadata: Option<JsptMetadata>}, 
}

impl Node {
    /// New up a Node::Simple instance, give a name and, optionally, metadata. 
    /// 
    /// # Parameters
    /// * `name` - The name of the simple Node, of a type that implements 
    ///            Into<String> 
    /// * `metadata` - A Some wrapped instance of Metadata, or None. 
    /// 
    /// # Returns 
    /// An instance of Node
    pub fn new_simple<I>(name: I, metadata: Option<JsptMetadata>) -> Node 
    where  
        I: Into<String>
    {
        Node::Simple(name.into(), metadata)
    }

    /// New up a Node::Pair, given a name, value, and, optionally, metadata. 
    /// 
    /// # Parameters
    /// 
    /// * `name` - The variable name of the Node, provided by a type that 
    ///            implements `Into<String>`. This will be the name of the 
    ///            Node as stored in the KeyMap.
    /// * `value` - The textual value of the node, provided by a type that 
    ///             implements `Into<String>`. This will be the value in 
    ///             the template.
    /// * `metadata` - a Some wrapped Metadata instance, or None. 
    /// 
    /// # Returns
    /// A Node instance. 
    pub fn new_pair<I>(name: I, value: I, metadata: Option<JsptMetadata>) -> Node 
    where
        I:Into<String> 
    {
        Node::Pair{
            name: name.into(),
            value: value.into(),
            metadata,
        }
    }

    /// New up a Node::ReVar, given a name, variable, and optionally, metadata.
    /// 
    /// # Parameters
    /// * `name` - The name of the Node, provided by a type that implements
    ///            `Into<String>`. 
    /// * `variable` - The regex variable name, provided by a type that implements
    ///                `Into<String>`.
    /// * `metadata` - a Some wrapped Metadata instance, or None. 
    /// 
    /// # Returns
    /// A `Node` instance.
    pub fn new_revar<I>(name: I, variable: I, metadata: Option<JsptMetadata>) -> Node 
    where 
        I:Into<String> 
    {
        Node::ReVar {
            name: name.into(),
            variable: variable.into(),
            metadata
        }
    }

    /// New up a Node::RegexSimple, given a name, a regular expression, and optionally,
    /// a Metadata instance.
    /// 
    /// # Parameters
    /// 
    /// * `name` - The name of the Node::RegexSimple, requiring a type that implements
    ///            `Into<String>`. 
    /// * `re`   - A regular expression, requiring a type that implements `Into<String>`. 
    /// * 1metadata` - A Some wrapped Metadata instance or None.
    /// 
    /// # Returns
    /// A `Node` instance.
    pub fn new_regexsimple<I>(name: I, re: I, metadata: Option<JsptMetadata>) -> Node 
    where 
        I:Into<String> 
    {
        Node::RegexSimple {
            name: name.into(),
            re: re.into(),
            metadata
        }
    }

    /// New up a Node::RegexComplex, given a name, a positive regex, a negative regex
    /// and optionally, a Metadata instance. 
    /// 
    /// # Parameters
    /// 
    /// * `name` - The name of the Node::RegexSimple, requiring a type that implements
    ///            `Into<String>`. 
    /// * `pos`  - A regular expression, requiring a type that implements `Into<String>`. 
    /// * `neg`  - A regular expression, requiring a type that implements `Into<String>`. 
    /// * 1metadata` - A Some wrapped Metadata instance or None.
    /// 
    /// # Returns
    /// A `Node` instance.
    pub fn new_regexcomplex<I>(name: I, pos: I, neg: I, metadata: Option<JsptMetadata>) -> Node 
    where 
        I:Into<String> 
    {
        Node::RegexComplex {
            name: name.into(),
            pos: pos.into(),
            neg: neg.into(),
            metadata
        }
    }

}
