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
}

