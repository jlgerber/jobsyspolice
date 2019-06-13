
// /// A Trait designed to produce a string, that when evaluated by a shell, clears 
// /// the variable with the supplied name (`varname`).
// pub trait ClearEnvVar {
//     fn clear_env_var(&self, varname: &str) -> String;
// }

pub mod bash;

pub trait ShellEnvManager {
    fn set_env_var(&self, varname: &str, value: &str) -> String;

    fn unset_env_var(&self, varname: &str) -> String;

    fn clear_env_var(&self, varname: &str) -> String;

}