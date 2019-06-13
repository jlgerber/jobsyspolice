//use crate::ClearEnvVar;
use crate::ShellEnvManager;

pub struct Bash {}

impl Bash {
    /// New up an instance of Bash
    pub fn new() -> Self {
        Self {}
    }
}

// impl ClearEnvVar for Bash {
//     fn clear_env_var(&self, varname: &str) -> String {
//         let  ret = format!("unset {};", varname); // could also be export {}='';
//         ret
//     }
// }

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