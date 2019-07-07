use crate::ShellEnvManager;
use std::path::Path;

/// Struct which implements the `ShellEnvManager` trait for Bash.
pub struct Shell {}


impl std::default::Default for Shell {
    fn default() -> Shell {
        Shell {}
    }
}

impl Shell {
    /// New up an instance of Tcsh
    pub fn new() -> Self {
        Self::default()
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

    fn set_alias(&self, name: &str, value: &Path) -> String {
        let  ret = format!("alias {} 'cd {}';", name, value.display()); 
        ret
    }

    fn unset_alias(&self, name: &str) -> String {
        let  ret = format!("unalias {};", name); 
        ret
    }
}