use crate::Node;

pub enum ReturnValue<'a> {
    Success,
    Fail{entry: &'a str, last_success: &'a Node}
}