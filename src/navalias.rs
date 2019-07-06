use serde::{ Deserialize, Serialize, self };

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Navalias {
    Simple(String),
    Complex{name:String, value:String},
}
