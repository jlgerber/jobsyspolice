
/// Represents the two cases of Regexs found in the template. 
/// 
/// The first type is Simple, having a name and a value. EG
/// show = "([A-Z][A-Z0-9]+)"
/// The second type is Complex, having a name, as well as 
/// a positive regular expression and a negative regular 
/// expression. EG
/// show = "([A-Z][A-Z0-9]+)" "(SHARED|COLOR|OUTSOURCE)"
#[derive(Debug, PartialEq, Eq)]
pub enum JsptRegex {
    Simple{
        name: String, 
        value: String
    },
    Complex{
        name: String, 
        positive: String, 
        negative: String
    },
}

