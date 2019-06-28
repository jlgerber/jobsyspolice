use petgraph::graph::{ DefaultIx, NodeIndex };
use std::ffi::OsString;
use std::cmp::Ordering;
use std::cell::RefCell;
use std::rc::Rc;
pub type NIndex = NodeIndex<DefaultIx>;

/// Used to capture the success or failure of a comparison
/// between a candidate path and the template. It provides
/// enough information on failure to present to the user
/// the root cause of a failure, including the requested
/// directory or file in the request which could not satisfy
/// the constraints imposed by the template, a well as the specific
/// nodes which it conflicted with.
///
/// # Parameters
///
/// * `entry` - OsString capturing the directory or file which
///             failed to validate against the template.
/// * `node` - NIndex of the last template node to successfully
///            match. This is the parent of the failed nodes
/// * `depth` - The depth of the failure, in terms of the
///             requested path.
#[derive(Debug, PartialEq, Eq)]
pub enum ReturnValue {
    Success(Rc<RefCell<Vec<NIndex>>>),
    Failure{ entry: OsString, node: NIndex, depth: u8 }
}

impl ReturnValue {
    pub fn new_success(val: Vec<NIndex>) -> ReturnValue {
        ReturnValue::Success(Rc::new(RefCell::new(val)))
    }

    /// Is the current ReturnValue instance a Success?
    pub fn is_success(&self) -> bool {
        if let ReturnValue::Success(_) = self {
            true
        } else {
            false
        }
        //return self == &ReturnValue::Success(_)
    }

    /// Is the current ReturnValue instance a failure?
    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }

    /// Return the depth of the ReturnValue. If the ReturnValue
    /// instance is Success, always return 0. Otherwise, return
    /// the value captured in the Failure case.
    pub fn depth(&self) -> u8 {
        match *self {
            ReturnValue::Success(_) => 0,
            ReturnValue::Failure{ depth:d, .. } => d,
        }
    }
}

impl Ord for ReturnValue {
    fn cmp(&self, other: &Self) -> Ordering {
        // success always equals success regardless of the internals
        if self.is_success() && other.is_success() {
            return Ordering::Equal;
        }
        if self.is_success() {
            return Ordering::Greater;
        }

        if other.is_success() {
            return Ordering::Less;
        }

        if let ReturnValue::Failure{ entry: _selfentry, node: _selfnode, depth: selfdepth } = self {
            if let ReturnValue::Failure{ entry: _oentry, node: _onode, depth: odepth } = other {
                if selfdepth == odepth { return Ordering::Equal; }
                else if selfdepth > odepth { return Ordering::Greater; }
                else { return Ordering::Less; }
            }
        }
        panic!("Should not reach here");
    }
}

impl PartialOrd for ReturnValue {
    fn partial_cmp( &self, other: &Self ) -> Option<Ordering> {
        Some( self.cmp(other) )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_test_success() {
        let v = Vec::new();
        let v2 = Vec::new();
        let rv1 = ReturnValue::new_success(v);
        let rv2 = ReturnValue::new_success(v2);
        assert_eq!(rv1, rv2);
    }

    #[test]
    fn equality_test_failure() {
        let n1  = NIndex::new(1);
        let entry = OsString::from("foob");
        let rv1 = ReturnValue::Failure{ entry, node: n1, depth: 10 };
        let v2 =Vec::new();
        let rv2 = ReturnValue::new_success(v2);
        assert_ne!(rv1, rv2);
    }

    #[test]
    fn inequality_test_same_node_and_entry() {
        let n1  =NIndex::new(2);

        let entry = OsString::from("foob");
        let rv1 = ReturnValue::Failure{ entry: entry.clone(), node: n1.clone(), depth: 10 };
        let rv2 = ReturnValue::Failure{ entry, node: n1, depth:9};
        assert!(rv1 > rv2);
    }

    #[test]
    fn inequality_test_different_node_and_entry() {
        let n1  = NIndex::new(1);
        let n2  = NIndex::new(2);

        let entry = OsString::from("foob");
        let entry2 = OsString::from("bla");

        let rv1 = ReturnValue::Failure{ entry, node: n1, depth: 10 };
        let rv2 = ReturnValue::Failure{ entry: entry2, node: n2, depth: 9 };
        assert!(rv1 > rv2);
    }
}