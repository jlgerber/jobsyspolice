
/// The various headers found in the template, including the possibility of the
/// Unknown header, which would indicate that we are in an error state.
/// The Headers are particularly important, as the StateMachine defines state
/// transitions in relation to each header. In other words, when the Statemachine
/// identifies a header as a result of parsing the current line, it triggers a
/// subsequent state transition. These valid transitions are defined in the 
/// StateMachine.
#[derive(Debug, PartialEq, Eq)]
pub enum Header {
    Regex,
    Node,
    Edge,
    Unknown(String),
}

