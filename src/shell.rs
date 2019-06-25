use std::str::FromStr;
use crate::JSPError;

pub mod bash;
pub mod tcsh;

#[derive(Debug, PartialEq, Eq)]
pub enum SupportedShell {
    Bash,
    Tcsh
}

impl SupportedShell {

    pub fn get(&self) -> Box<dyn ShellEnvManager> {
        match self {
            SupportedShell::Bash => Box::new(bash::Shell::new()),
            SupportedShell::Tcsh => Box::new(tcsh::Shell::new()),
        }
    }
}

impl FromStr for SupportedShell {
    type Err = JSPError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bash" => Ok(SupportedShell::Bash),
            "tcsh" => Ok(SupportedShell::Tcsh),
            _ => Err(JSPError::UnknownShell(s.to_string())),
        }
    }
}

/// Basic methods for setting environment variables for a target Shell. These
/// are used by the `jsp go` to print commands to stdout, which will later be 
/// `eval`ed in the shell in order to mutate the existing environment. 
pub trait ShellEnvManager {
    /// Generate a string that sets an environment variable named `varname` to  `value`
    /// in a specific Shell, as dictated by the concrete implementation of the trait.
    fn set_env_var(&self, varname: &str, value: &str) -> String;

    /// Generate a string that unsets an environment variable for specific Shell,
    /// as dictated by the concrete implementation of the traii.
    fn unset_env_var(&self, varname: &str) -> String;

    /// Generate a string which clears out the variable named `varname` for a given
    /// Shell as dictated by the trait's concrete implementation. 
    fn clear_env_var(&self, varname: &str) -> String;

}