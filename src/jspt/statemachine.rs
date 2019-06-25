use nom::{IResult};
use crate::jspt::{ParseResult, Header, start_parser, regex_parser, node_parser, edge_parser, JSPTemplateError, JSPTemplateLineError};
use std::cell::Cell;
use std::fmt;

/// The states that the StateMachine may transition through, from the start (Start)
/// to the two possible terminal states (Done, Error). 
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum State {
    Start,
    RegexParsing,
    NodeParsing,
    EdgeParsing,
    Done,
    Error
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Start => write!(f, "Start"),
            State::RegexParsing => write!(f, "RegexParsing"),
            State::NodeParsing => write!(f, "NodeParsing"),
            State::EdgeParsing => write!(f, "EdgeParsing"),
            State::Done => write!(f, "Done"),
            State::Error => write!(f, "Error"),
        }
    }
}

/// Responsible for Manging state transitions and for parsing 
/// the jsptemplate file appropriately given the current state, 
/// on a per line basis.
pub struct StateMachine {
    // The current state of the StateMachine
    state: State,
    // The current line number.
    line: Cell<usize>,
    // a tuple of parsers corresponding with the states
    // that we will be passing through. This can be a tuple
    // as the transitions are well defined. 
    parsers: (
        fn(&str)->IResult<&str, ParseResult>,
        fn(&str)->IResult<&str, ParseResult>,
        fn(&str)->IResult<&str, ParseResult>,
        fn(&str)->IResult<&str, ParseResult>  
    )
}

impl StateMachine {
    /// New up a StateMachine instance
    pub fn new() -> StateMachine {
        StateMachine {
            state: State::Start,
            line: Cell::new(0),
            parsers: (start_parser, regex_parser, node_parser, edge_parser),
        }
    }

    /// Get the current line number being parsed. The line number is managed
    /// by the parse method, and is incremented immediately upon invocation.
    pub fn line_number(&self) -> usize {
        self.line.get()
    }

    /// Retrieve a reference to the current state. 
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Parse the current line of input, possibly transitioning to the next 
    /// state, depending upon the current line. 
    /// In this case, if the input is a Header, transition
    /// the statemachine to the next valid state, as defined internally.
    ///
    /// The state graph should look something like: 
    /// `Start -> RegexParsing -> NodeParsing -> EdgeParseing -> Done`
    /// assuming that the headers appear in order. One may transition back and forth, 
    /// using the headers. However one may not depend upon another state's contents before
    /// said contents has been processed. 
    /// IE if one cannot reference a `regex` from the `node` state before the aforementioned
    /// regex has been parsed. 
    pub fn parse(&mut self, input: &str) -> Result<ParseResult, JSPTemplateLineError> {
        self.line.set(self.line.get() + 1);
        // parse current line if the statemachine is in a state that has a parser
        // associated with it. If the state doesnt have an associated parser, set
        // the appropriate error.
        let parsed_line = match self.state {
            State::Start        => Ok(self.parsers.0(input)),
            State::RegexParsing => Ok(self.parsers.1(input)),
            State::NodeParsing  => Ok(self.parsers.2(input)),
            State::EdgeParsing  => Ok(self.parsers.3(input)),
            State::Done  => Err(JSPTemplateError::DoneState),
            State::Error => Err(JSPTemplateError::ErrorState),
        };
        // Outer result determines whether the statemachine is in a state 
        // that can parse lines. Neither the Done state nor the Error state 
        // qualify as such.
        match parsed_line {
            Ok(result) => {
                // inner Result determines whether parsing of line is ok
                match result {
                    Ok((_, value)) => {
                        // If we encounter a header, we transition to the state
                        // associated with the header. We only allow valid transitions as 
                        // dictated by the next_state method.
                        if let ParseResult::Header(ref header) = value {
                            let current_state = self.state.clone();

                            // get the next allowed state from the statemachine
                            let next_valid_state = match self.next_valid_state(){
                                Ok(a) => a,
                                Err(e) => return Err(JSPTemplateLineError::from((self.line.get(), input.to_owned(), self.state.clone(), e))),
                            };

                            // get the state assocated with the header
                            let new_state = match header {
                                Header::Node  =>  State::NodeParsing,
                                Header::Edge  =>  State::EdgeParsing,
                                Header::Regex =>  State::RegexParsing,
                                Header::Unknown(_) =>  State::Error,
                            };

                            // make sure that the new state matches the next valid state in the 
                            // statemachine
                            if next_valid_state != new_state {
                                return Err(
                                    JSPTemplateLineError::from(
                                        (self.line.get(),
                                        input.to_owned(),
                                        self.state.clone(),
                                        JSPTemplateError::InvalidStateTransition(current_state, new_state))
                                        )
                                    )   
                            }

                            // set the new state if the transition is a valid one to make
                            self.state = new_state;
                        }

                        return Ok(value);
                    },  
                    Err(e) => {
                        return Err(
                            JSPTemplateLineError::from(
                                ( self.line.get(), input.to_owned(), self.state.clone(), JSPTemplateError::from(e)) )
                            );
                    },
                }
            }, 
            Err(e) =>    Err(JSPTemplateLineError::from((self.line.get(), input.to_owned(), self.state.clone(), e))),
        }
    }

    // Retrieve the next state in the statemachine given the current state
    fn next_valid_state(&self) -> Result<State, JSPTemplateError> {
        match self.state {
            State::Start        => Ok(State::RegexParsing),
            State::RegexParsing => Ok(State::NodeParsing),
            State::NodeParsing  => Ok(State::EdgeParsing),
            State::EdgeParsing  => Ok(State::Done),
            State::Done         => Err(JSPTemplateError::NoValidNextState(State::Done)),
            State::Error        => Err(JSPTemplateError::NoValidNextState(State::Error))
        }
    }
}