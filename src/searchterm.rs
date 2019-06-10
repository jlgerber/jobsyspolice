use std::collections::VecDeque;

/// Struct which owns a search term used by find
#[derive(Debug, PartialEq, Eq)]
pub struct SearchTerm {
    key: String,
    value: String,
}

impl SearchTerm {
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

#[derive(Debug)]
pub struct Search {
    terms: VecDeque<SearchTerm>
}

impl Search {
    pub fn new() -> Self {
        Self {
            terms: VecDeque::new()
        }
    }

    pub fn push_front(&mut self, term: SearchTerm) {
        self.terms.push_front(term);
    }

    pub fn push_back(&mut self, term: SearchTerm) {
        self.terms.push_back(term);
    }
}



