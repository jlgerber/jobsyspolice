use crate::constants;
use log;
use serde::{self, Deserialize, Serialize};
use std::{
    env,
    fmt::{self, Display, Formatter},
};

/// The User enum encompasses both static and dynamic users (specifically the current user as
/// recorded in the environment).
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Clone)]
//#[serde(tag = "type")]
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
    /// New up a Named or Me user. If name is $USER or $user, then new
    /// returns User::Me. Otherwise, new returns Named(name)
    pub fn new<S: Into<User>>(name: S) -> User {
        name.into()
    }

    pub fn to_string(&self) -> String {
        match self {
            User::Me => match env::var(constants::USER_ENV_VAR) {
                Ok(u) => {
                    log::debug!("looked up user from env: {}", u);
                    u
                },
                Err(_) => {
                    log::warn!("unable to look up current user from environment!");
                    get_default_user()
                }
            },

            User::Named(n) =>  n.clone(),
        }
    }
}


impl From<String> for User {
    fn from(name: String) -> Self {
        if &name == "$user" || &name == "$USER" {
            User::Me
        } else {
            User::Named(name)
        }
    }
}

impl From<&str> for User {
    fn from(name: &str) -> Self {
        User::from(name.to_string())
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
    fn can_convert_from_string_to_me() {
        let name = String::from("$user");
        let usr = User::from(name);
        assert_eq!(usr, User::Me);
        let name = String::from("$USER");
        let usr = User::from(name);
        assert_eq!(usr, User::Me);
    }

    #[test]
    fn can_convert_from_string_into_user() {
        let name = String::from("fred");
        let usr: User = name.into();
        assert_eq!(usr, User::Named(String::from("fred")));
    }

    #[test]
    fn can_convert_from_string_into_user_me() {
        let name = String::from("$USER");
        let usr: User = name.into();
        assert_eq!(usr, User::Me);
    }

    #[test]
    fn can_convert_from_str() {
        let usr = User::from("fred");
        assert_eq!(usr, User::Named(String::from("fred")));
    }

    #[test]
    fn can_convert_from_str_to_user() {
        let usr = User::from("$USER");
        assert_eq!(usr, User::Me);
        let usr = User::from("$user");
        assert_eq!(usr, User::Me);
    }

    #[test]
    fn can_convert_from_str_into_user() {
        let name = "fred";
        let usr: User = name.into();
        assert_eq!(usr, User::Named(String::from("fred")));
    }

    #[test]
    fn can_convert_from_str_into_user_me() {
        let name = "$user";
        let usr: User = name.into();
        assert_eq!(usr, User::Me);
    }
}
