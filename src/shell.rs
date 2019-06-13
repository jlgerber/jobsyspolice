
/// A Trait designed to produce a string, that when evaluated by a shell, clears 
/// the variable with the supplied name (`varname`).
pub trait ClearEnvVar {
    fn clear_env_var(&self, varname: &str) -> String;
}

pub mod bash;

