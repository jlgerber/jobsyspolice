#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum EntryType {
    Directory,
    Volume,
    Root,
}
