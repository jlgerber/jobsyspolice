use crate::ShellEnvManager;

/// Struct which implements the `ShellEnvManager` trait for Bash.
pub struct Bash {}

impl Bash {
    /// New up an instance of Bash
    pub fn new() -> Self {
        Self {}
    }
}

impl ShellEnvManager for Bash {

    fn set_env_var(&self, varname: &str, value: &str) -> String {
        format!("export {}={};", varname, value)
    }

    fn unset_env_var(&self, varname: &str) -> String {
        format!("unset {};", varname)
    }

    fn clear_env_var(&self, varname: &str) -> String {
        let  ret = format!("unset {};", varname); // could also be export {}='';
        ret
    }

}