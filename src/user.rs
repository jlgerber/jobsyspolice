use crate::constants;
use log;
use serde::{self, Deserialize, Serialize};
use std::{
    env,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum User {
    Me,
    Named(String),
}
/// Looks up the default user
pub fn get_default_user() -> String {
    String::from(constants::DEFAULT_USER)
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            User::Me => match env::var(constants::USER_ENV_VAR) {
                Ok(u) => write!(f, "{}", u),
                Err(_) => {
                    log::warn!("unable to look up current user from environment!");
                    write!(f, "{}", get_default_user())
                }
            },

            User::Named(n) => write!(f, "{}", n),
        }
    }
}

impl User {
    /// New up a Named user
    pub fn new<S: Into<String>>(name: S) -> User {
        User::Named(name.into())
    }

    /// New up a user that is resolved from the environment
    /// to be the current USER.
    pub fn new_me() -> User {
        User::Me
    }
}


impl From<String> for User {
    fn from(name: String) -> Self {
        User::Named(name)
    }
}

impl From<&str> for User {
    fn from(name: &str) -> Self {
        User::Named(name.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_new_up_named_user() {
        let usr = User::new("fred");
        assert_eq!(usr, User::Named(String::from("fred")));
    }

    #[test]
    fn can_format_me() {
        let usr = User::Me;
        let usr_str = format!("{}", usr);
        println!("{}", usr);
        assert_ne!(usr_str, String::from(constants::DEFAULT_USER));
    }

    #[test]
    fn can_format_named_user() {
        let usr = User::new("fred");
        let usr_str = format!("{}", usr);
        assert_eq!(usr_str, String::from("fred"));
    }

    #[test]
    fn can_convert_from_string() {
        let name = String::from("fred");
        let usr = User::from(name);
        assert_eq!(usr, User::Named(String::from("fred")));
    }

    #[test]
    fn can_convert_from_string_into_user() {
        let name = String::from("fred");
        let usr: User = name.into();
        assert_eq!(usr, User::Named(String::from("fred")));
    }

    #[test]
    fn can_convert_from_str() {
        let usr = User::from("fred");
        assert_eq!(usr, User::Named(String::from("fred")));
    }

    #[test]
    fn can_convert_from_str_into_user() {
        let name = "fred";
        let usr: User = name.into();
        assert_eq!(usr, User::Named(String::from("fred")));
    }
}
