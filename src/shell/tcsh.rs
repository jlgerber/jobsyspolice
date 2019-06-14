use crate::ShellEnvManager;

/// Struct which implements the `ShellEnvManager` trait for Bash.
pub struct Shell {}

impl Shell {
    /// New up an instance of Tcsh
    pub fn new() -> Self {
        Self {}
    }
}

impl ShellEnvManager for Shell {

    fn set_env_var(&self, varname: &str, value: &str) -> String {
        format!("setenv {} {};", varname, value)
    }

    fn unset_env_var(&self, varname: &str) -> String {
        format!("setenv {} \"\";", varname)
    }

    fn clear_env_var(&self, varname: &str) -> String {
        let  ret = format!("unsetenv {};", varname); // could also be export {}='';
        ret
    }

}