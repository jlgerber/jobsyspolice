use crate::ClearEnvVar;

pub struct Bash {}

impl Bash {
    /// New up an instance of Bash
    pub fn new() -> Self {
        Self {}
    }
}

impl ClearEnvVar for Bash {
    fn clear_env_var(&self, varname: &str) -> String {
        let  ret = format!("unset {};", varname); // could also be export {}='';
        ret
    }
}