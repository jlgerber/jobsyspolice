use serde::{ Deserialize, Serialize, self };

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Navalias {
    Simple(String),
    Complex{name:String, value:String},
}

impl Navalias {
    /// Instantiate a new Navalias::Simple
    pub fn new_simple<S>(name: S) -> Self 
    where
        S: Into<String>
    {
        Navalias::Simple(name.into())
    } 

    /// Instantiate a new Navalias::Complex 
    pub fn new_complex<S>(name: S, value: S) -> Self 
    where
        S: Into<String> 
    {
        Navalias::Complex{name: name.into(), value: value.into()}
    }
}