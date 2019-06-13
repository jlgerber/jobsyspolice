use crate::{constants};
use std::env;

/// CachedEnvVars is an iterator for variables cached in the environment 
/// by jspgo
#[derive(Debug)]
pub struct CachedEnvVars(Vec<String>);

impl CachedEnvVars {
    /// new up a CachedVar
    pub fn new() -> Self {
        let var = env::var(constants::JSP_TRACKING_VAR).unwrap_or(String::from(""));
        let varnames = var.split(":").map(|x| x.to_owned()).collect::<Vec<String>>();
        Self(varnames)
    }
    
    /// Return an iterator over cached vars
    pub fn iter<'a>(&'a self) -> IterCachedEnvVars<'a> {
        IterCachedEnvVars::new(self)
    }
}

impl IntoIterator for CachedEnvVars {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}


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
}



