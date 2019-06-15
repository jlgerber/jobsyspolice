use crate::{constants};
use std::env;
use crate::ShellEnvManager;
use log;
/// CachedEnvVars provides a means of looking up and iterating over the previously 
/// set JSPVars in the environment
#[derive(Debug)]
pub struct CachedEnvVars(Vec<String>);

impl CachedEnvVars {

    /// new up a CachedEnvVars
    pub fn new() -> Self {
        log::info!("CachedEnvVars new()");
        let var = env::var(constants::JSP_TRACKING_VAR).unwrap_or(String::from(""));
        let varnames = var.split(":").filter(|x| x.trim() != "").map(|x| x.to_owned()).collect::<Vec<String>>();
        Self(varnames)
    }
    
    /// Return an iterator over CachedEvnVars
    pub fn iter<'a>(&'a self) -> IterCachedEnvVars<'a> {
        IterCachedEnvVars::new(self)
    }

    /// Produce a string that, when eval'ed by a shell (eg bash or tcsh) compatible
    /// with the implementation of `ClearEnvVar by `clearer`, will blank out the
    /// settings the supplied variables.
    pub fn clear(&self, clearer: &Box<dyn ShellEnvManager>) -> String  {
        let mut result = String::new();
        for var in self.iter() {
            result.push_str( clearer.clear_env_var(var).as_str() );
        }
        result
    }
}

impl IntoIterator for CachedEnvVars {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Iterator for CachedEnvVars
pub struct IterCachedEnvVars<'a> {
    inner: &'a CachedEnvVars,
    pos: usize,
}

impl<'a> IterCachedEnvVars<'a> {
    fn new(inner: &'a CachedEnvVars) -> Self {
        Self {
            inner,
            pos: 0
        }
    }
}

impl<'a> Iterator for IterCachedEnvVars<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.inner.0.len() {
            // Obviously, there isn't any more data to read so let's stop here.
            None
        } else {
            // We increment the position of our iterator.
            self.pos += 1;
            // We return the current value pointed by our iterator.
            Some(self.inner.0[self.pos - 1].as_ref())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::bash;

    #[test]
    fn into_iter_collects_vec_of_strings() {
        // set the environment
        env::set_var(constants::JSP_TRACKING_VAR, "DD_SHOW:DD_SEQUENCE:DD_SHOT");
        let cache = CachedEnvVars::new();
        let vars = cache.into_iter().collect::<Vec<String>>();
        assert_eq!(vars, vec!["DD_SHOW".to_owned(), "DD_SEQUENCE".to_owned(), "DD_SHOT".to_owned()]);
    }

    #[test]
    fn iter_collects_vec_of_str_refs() {
        // set the environment
        env::set_var(constants::JSP_TRACKING_VAR, "DD_SHOW:DD_SEQUENCE:DD_SHOT");
        let cache = CachedEnvVars::new();
        let vars = cache.iter().collect::<Vec<&str>>();

        assert_eq!(vars, vec!["DD_SHOW", "DD_SEQUENCE", "DD_SHOT"]);
    }

    #[test]
    fn can_clear_env_vars() {
        // set the environment
        env::set_var(constants::JSP_TRACKING_VAR, "DD_SHOW:DD_SEQUENCE:DD_SHOT");
        
        let cache = CachedEnvVars::new();
        let bash: Box<dyn ShellEnvManager> = Box::new(bash::Shell::new());
        let clearstr = cache.clear(&bash);
        let expect = String::from("unset DD_SHOW;unset DD_SEQUENCE;unset DD_SHOT;");
        assert_eq!(clearstr, expect);
    }
}



