#[derive(Debug, PartialEq, Eq)]
pub struct SearchTerm<'a> {
    key: &'a str,
    value: &'a str,
}

