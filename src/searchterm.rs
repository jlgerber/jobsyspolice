use std::collections::VecDeque;
use crate::JSPError;

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

    /// Retrieve a reference to the key str, which represents the 
    /// name of a NodeType::Regex Node within the JGraph instance. 
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Retrieve a reference to the value str, which is intended to 
    /// represeent a valid (passing) value for the NodeType::Regex Node
    /// whose name is the `key` of the SearchTerm.
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::str::FromStr for SearchTerm {
    type Err = JSPError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(":").map(|x| x.to_owned()).collect::<Vec<String>>();
        if pieces.len() != 2 {
            return Err(JSPError::SearchTermError(format!("Cannot construct SearchTerm from {}", s)));
        }
        let value = pieces.pop().unwrap();
        let key = pieces.pop().unwrap();
        Ok(SearchTerm::new(key, value))
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

    /// pop a term off of the front of the Search and
    /// return it wrapped in an Option, meaning If Search 
    /// is empty, pop_front returns None. Otherwise, it 
    /// returns a Some(SearchTerm).
    pub fn pop_front(&mut self) -> Option<SearchTerm> {
        self.terms.pop_front()
    }

    /// pop off a term from back of the Search and
    /// return it wrapped in an Option, meaning if Search
    /// is empty, pop_back returns None. Otherwise it 
    /// returns a Some(SearchTerm).
    pub fn pop_back(&mut self) -> Option<SearchTerm> {
        self.terms.pop_back()
    }

    /// Retrieve the keys as a `Vec` of `&str`s
    pub fn keys(&self) -> VecDeque<&str> {
        self.terms.iter().map(|x| x.key()).collect::<VecDeque<&str>>()
    }

    /// Retrieve the values as a `Vec` of `&str`s
    pub fn values(&self) -> VecDeque<&str> {
        self.terms.iter().map(|x| x.value()).collect::<VecDeque<&str>>()
    }

    /// Retrieve the keys as a `Vec` of `String`s
    pub fn keys_owned(&self) -> VecDeque<String> {
        self.terms.iter().map(|x| x.key().to_string()).collect::<VecDeque<String>>()
    }

    /// Retrieve the values as a `Vec` of `String`s
    pub fn values_owned(&self) -> VecDeque<String> {
        self.terms.iter().map(|x| x.value().to_string()).collect::<VecDeque<String>>()
    }

    /// Return the number of SearchTerms within the Search.
    pub fn len(&self) -> usize {
        self.terms.len()
    }

    /// Retrieve an Option wrapped reference to a SearchTerm, given
    /// an index. Unlike using square bracket notation, this 
    /// method will not crash when supplied with an out of bounds
    /// index. Rather, it will simply return None.
    pub fn get(&self, index: usize) -> Option<&SearchTerm> {
        if index == self.len() {
            None
        } else {
            Some(&self[index])
        }
    }
}

/// SearchTerm may be accessed by index. SearchTerm will
/// panic if an index outside of its range is supplied.
impl std::ops::Index<usize> for Search {
    type Output = SearchTerm;

    fn index(&self, index: usize) -> &Self::Output {
         &self.terms[index]
    }
}

#[cfg(test)]
mod searchterm_tests {
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
    fn nonequal_searchterms_may_be_tested_for_inequality() {
        let searchterm = SearchTerm::new("show", "DEV01");
        let expect = SearchTerm {
            key: "show".to_owned(),
            value: "DEV02".to_owned()
        };
        assert_ne!(searchterm, expect);
    }

    #[test]
    fn can_be_constructed_fromstr() {
        use std::str::FromStr;
        let st = SearchTerm::from_str("foo:bar").unwrap();
        let expect = SearchTerm::new("foo", "bar");
        assert_eq!(st, expect);
    }
}

#[cfg(test)]
mod search_tests {
    use super::*;

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
        assert_eq!(&search[0], &SearchTerm::new("show", "DEV01"));
    }

    #[test]
    fn search_can_pop_front() {
        let mut search = Search::new();
        search.push_front(SearchTerm::new("show", "DEV01"));
        search.push_front(SearchTerm::new("sequence", "RD"));
        assert_eq!(search.pop_front().unwrap(), SearchTerm::new("sequence", "RD"));
    }

    #[test]
    fn search_can_pop_back() {
        let mut search = Search::new();
        search.push_back(SearchTerm::new("show", "DEV01"));
        search.push_back(SearchTerm::new("sequence", "RD"));
        assert_eq!(search.pop_back().unwrap(), SearchTerm::new("sequence", "RD"));
    }

    #[test]
    fn can_retrieve_keys() {
        let mut search = Search::new();
        search.push_back(SearchTerm::new("show", "DEV01"));
        search.push_back(SearchTerm::new("sequence", "RD"));
        let keys = search.keys();
        assert_eq!(keys,  VecDeque::from(vec!["show", "sequence"]));
    }

    #[test]
    fn can_retrieve_values() {
        let mut search = Search::new();
        search.push_back(SearchTerm::new("show", "DEV01"));
        search.push_back(SearchTerm::new("sequence", "RD"));
        let values = search.values();
        assert_eq!(values,  VecDeque::from(vec!["DEV01", "RD"]));
    }

    #[test]
    fn can_retrieve_owned_keys() {
        let mut search = Search::new();
        search.push_back(SearchTerm::new("show", "DEV01"));
        search.push_back(SearchTerm::new("sequence", "RD"));
        let keys = search.keys_owned();
        assert_eq!(keys,  VecDeque::from(vec!["show".to_string(), "sequence".to_string()]));
    }

    #[test]
    fn can_retrieve_owned_values() {
        let mut search = Search::new();
        search.push_back(SearchTerm::new("show", "DEV01"));
        search.push_back(SearchTerm::new("sequence", "RD"));
        let values = search.values_owned();
        assert_eq!(values, VecDeque::from(vec!["DEV01".to_owned(), "RD".to_owned()]));
    }

    #[test]
    fn can_retrieve_len_of_search() {
        let mut search = Search::new();
        search.push_back(SearchTerm::new("show", "DEV01"));
        search.push_back(SearchTerm::new("sequence", "RD"));
        search.push_back(SearchTerm::new("shot", "0001"));
        assert_eq!(search.len(), 3);
    }

    #[test]
    fn can_get_searchterm_from_search() {
        let mut search = Search::new();
        search.push_back(SearchTerm::new("show", "DEV01"));
        search.push_back(SearchTerm::new("sequence", "RD"));
        search.push_back(SearchTerm::new("shot", "0001"));
        assert_eq!(search.get(2), Some(&SearchTerm::new("shot", "0001")));
    }

    #[test]
    fn can_index_searchterm_from_search() {
        let mut search = Search::new();
        search.push_back(SearchTerm::new("show", "DEV01"));
        search.push_back(SearchTerm::new("sequence", "RD"));
        search.push_back(SearchTerm::new("shot", "0001"));
        assert_eq!(search[2], SearchTerm::new("shot", "0001"));
        assert_eq!(search.len() , 3);
    }

}





