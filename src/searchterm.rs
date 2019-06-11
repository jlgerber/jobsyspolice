use std::collections::VecDeque;

/// Struct which owns a search term used by find to match  
/// a Node of `NodeType::Regex` within the JGraph, and suggest
/// a valid solution for the Node's regular expression. For example,
/// we may define a search term that is meant to find a show within the 
/// graph: 
/// ```
/// let show_searchterm = 
/// SearchTerm {
///     key: String::from("show"),
///     value: String::from("DEV01"),
/// };
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct SearchTerm {
    key: String,
    value: String,
}

impl SearchTerm {
    /// New up a SearchTerm, provided a key and value which must implement
    /// Into<String> and Debug. The key corresponds with the name parameter
    /// of the NodeType::Regex, and the value is the intended value
    /// 
    /// # Parameters
    /// * `key` the name of the node which we are interested in
    /// * `value` the value of the Regexp which we are interested in
    /// 
    /// # Returns
    /// A new instance of SearchTerm
    pub fn new<I>(key: I, value: I) -> Self 
    where
        I: Into<String> + std::fmt::Debug 
    {
        Self {
            key: key.into(),
            value: value.into()
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

/// The Search struct holds an ordered list of `SearchTerm`s representing a 
/// search through the jobsystem graph (JGraph). The search is intended to
///  match `Node`s of type `NodeType::Regex`.
/// 
/// # Example
/// 
/// ```
/// let mut search = Search::new();
/// search.push_front(SearchTerm::new("show", "DEV01"));
/// search.push_front(SearchTerm::new("sequence", "RD"));
/// search.push_front(SearchTerm::new("shot", "0001"));
/// search.push_front(SearchTerm::new("IMG", "jgerber"));
/// ```
#[derive(Debug)]
pub struct Search {
    terms: VecDeque<SearchTerm>
}

impl Search {
    /// New up the Search entity
    pub fn new() -> Self {
        Self {
            terms: VecDeque::new()
        }
    }

    /// Add a term to the front of the Search
    pub fn push_front(&mut self, term: SearchTerm) {
        self.terms.push_front(term);
    }
    /// Add a term to the back of the Search
    pub fn push_back(&mut self, term: SearchTerm) {
        self.terms.push_back(term);
    }

    /// Retrieve the keys as a `Vec` of `&str`s
    pub fn keys(&self) -> Vec<&str> {
        self.terms.iter().map(|x| x.key()).collect::<Vec<&str>>()
    }

    /// Retrieve the values as a `Vec` of `&str`s
    pub fn values(&self) -> Vec<&str> {
        self.terms.iter().map(|x| x.value()).collect::<Vec<&str>>()
    }

    /// Retrieve the keys as a `Vec` of `String`s
    pub fn keys_owned(&self) -> Vec<String> {
        self.terms.iter().map(|x| x.key().to_string()).collect::<Vec<String>>()
    }

    /// Retrieve the values as a `Vec` of `String`s
    pub fn values_owned(&self) -> Vec<String> {
        self.terms.iter().map(|x| x.value().to_string()).collect::<Vec<String>>()
    }

    pub fn len(&self) -> usize {
        self.terms.len()
    }
}

impl std::ops::Index<usize> for Search {
    type Output = SearchTerm;

    fn index(&self, index: usize) -> &Self::Output {
        &self.terms[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_searchterms_may_be_tested_for_equality() {
        let searchterm = SearchTerm::new("show", "DEV01");
        let expect = SearchTerm {
            key: "show".to_owned(),
            value: "DEV01".to_owned()
        };
        assert_eq!(searchterm, expect);
    }


    #[test]
    fn nonequal_searchterms_may_be_tested_for_equality() {
        let searchterm = SearchTerm::new("show", "DEV01");
        let expect = SearchTerm {
            key: "show".to_owned(),
            value: "DEV02".to_owned()
        };
        assert_ne!(searchterm, expect);
    }

    #[test]
    fn search_can_push_front() {
        let mut search = Search::new();
        search.push_front(SearchTerm::new("show", "DEV01"));
        search.push_front(SearchTerm::new("sequence", "RD"));
        assert_eq!(&search[0], &SearchTerm::new("sequence", "RD"));
    }


    #[test]
    fn search_can_push_back() {
        let mut search = Search::new();
        search.push_back(SearchTerm::new("show", "DEV01"));
        search.push_back(SearchTerm::new("sequence", "RD"));
        assert_eq!(&search[0], &SearchTerm::new("show", "DEV01 "));
    }
}





