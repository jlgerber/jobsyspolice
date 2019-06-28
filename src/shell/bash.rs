use crate::ShellEnvManager;

/// Struct which implements the `ShellEnvManager` trait for Bash.
pub struct Shell {}

impl std::default::Default for Shell {
    fn default() -> Shell {
        Shell {}
    }
}

impl Shell {
    /// New up an instance of Shell
    pub fn new() -> Self {
        Shell::default()
    }
}

impl ShellEnvManager for Shell {

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