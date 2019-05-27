use crate::Node;
use std::ffi::OsString;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ReturnValue {
    Success,
    Failure{ entry: OsString, node: Node, depth: u8 }
}

impl ReturnValue {
    pub fn is_success(&self) -> bool {
        return self == &ReturnValue::Success
    }

    pub fn is_failure(&self) -> bool {
        return !self.is_success()
    }
}

impl Ord for ReturnValue {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_success() && other.is_success() {
            return Ordering::Equal;
        }
        if self.is_success() {
            return Ordering::Greater;
        }

        if other.is_success() {
            return Ordering::Less;
        }

        if let ReturnValue::Failure{entry: _selfentry, node: _selfnode, depth: selfdepth } = self {
            if let ReturnValue::Failure{entry: _oentry, node: _onode, depth: odepth } = other {
                if selfdepth == odepth { return Ordering::Equal; }
                else if selfdepth > odepth { return Ordering::Greater; }
                else { return Ordering::Less; }
            }
        }
        panic!("Should not reach here");
    }
}

impl PartialOrd for ReturnValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ NodeType, EntryType };

    #[test]
    fn equality_test_success() {
        let rv1 = ReturnValue::Success;
        let rv2 = ReturnValue::Success;
        assert_eq!(rv1, rv2);
    }

    #[test]
    fn equality_test_failure() {
        let n1  = Node::new(
            NodeType::Simple("foobar".to_string()),
            EntryType::Directory
        );
        let entry = OsString::from("foob");
        let rv1 = ReturnValue::Failure{ entry: entry, node: n1, depth: 10 };
        let rv2 = ReturnValue::Success;
        assert_ne!(rv1, rv2);
    }

    #[test]
    fn inequality_test_same_node_and_entry() {
        let n1  = Node::new(
            NodeType::Simple("foobar".to_string()),
            EntryType::Directory
        );

        let entry = OsString::from("foob");
        let rv1 = ReturnValue::Failure{ entry: entry.clone(), node: n1.clone(), depth: 10 };
        let rv2 = ReturnValue::Failure{ entry: entry, node: n1, depth:9};
        assert!(rv1 > rv2);
    }

    #[test]
    fn inequality_test_different_node_and_entry() {
        let n1  = Node::new(
            NodeType::Simple("foobar".to_string()),
            EntryType::Directory
        );
        let n2  = Node::new(
            NodeType::Simple("bla".to_string()),
            EntryType::Directory
        );
        let entry = OsString::from("foob");
        let entry2 = OsString::from("bla");

        let rv1 = ReturnValue::Failure{ entry: entry, node: n1, depth: 10 };
        let rv2 = ReturnValue::Failure{ entry: entry2, node: n2, depth: 9 };
        assert!(rv1 > rv2);
    }
}