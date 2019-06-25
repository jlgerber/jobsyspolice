//! Formerly a separate crate - jspcompile - this code was merged when
//! it was discovered that there was no nead to precompile the file into
//! a json struct. The parsing was fast enough. 
//! 
//! The code remains distinct however.
pub mod parser;
pub use parser::{start_parser, regex_parser, node_parser, edge_parser, parse_components, parse_metadata};

pub mod helpers;

pub mod components;
pub use components::*;

pub mod statemachine;
pub use statemachine::{State, StateMachine};

pub mod errors;
pub use errors::{JSPTemplateError, JSPTemplateLineError};

pub mod loader;
pub use loader::{Loader, JGraphKeyMap, RegexMap};
