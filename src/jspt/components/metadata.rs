/// Potential JsptMetadata associated with a `Node` in the `JGraph`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MetadataComponent {
    Volume,
    Permissions(String),
    EnvVarName(String),
    Owner(String),
    // Nom requires that all branches of certain 
    // matches have the same type, so I added 
    // Separator, even though it isn't really a 
    // type that survives parsing. I may create a separate enum
    // to get around this in the future, as I don't like this leaking.
    Separator, 
    //NavAlias(String), 
    //AutoCreate
}

/// Tracks the supported metadata values in the template, delimited
/// in the Node section by square brackets. 
/// EG
/// `[ volume, owner:$me, perms: 777 ]`
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct JsptMetadata {
    volume: bool,
    permissions: Option<String>,
    varname: Option<String>,
    owner: Option<String>
}

impl JsptMetadata {
    /// new up an empty `JsptMetadata` instance. By default, JsptMetadata is not a volume,
    /// and all of its optional fields are set to None. 
    pub fn new() -> Self {
        Self {
            volume: false,
            permissions: None,
            varname: None,
            owner: None,
        }
    }

    /// Determine whether the JsptMetadata instance is empty, defined as the volume field being false, 
    /// and all of the optional terms being None. 
    pub fn is_empty(&self) -> bool {
        self.volume == false && self.permissions.is_none() && self.varname.is_none() && self.owner.is_none()
    }

    /// Set volume and get back moved self. This is designed to be used in 
    /// a fluent api. Otherwise, you must assign back. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jsp::jspt::JsptMetadata;
    /// 
    /// let metadata = JsptMetadata::new()
    ///                 .set_volume(true)
    ///                 .set_owner(Some("jgerber"));
    /// ```
    pub fn set_volume(mut self, is: bool) -> Self {
        self.volume = is;
        self
    }

    /// Test to see if the JsptMetadata represents a Volume. 
    /// 
    /// # Parameters
    /// None
    /// 
    /// # Returns
    /// bool 
    /// 
    /// # Examples 
    /// 
    /// ```
    /// use jsp::jspt::JsptMetadata;
    /// 
    /// let metadata = JsptMetadata::new()
    ///                 .set_volume(true)
    ///                 .set_owner(Some("jgerber"));
    /// 
    /// assert_eq!(metadata.is_volume(), true);
    /// ```
    pub fn is_volume(&self) -> bool {
        self.volume
    }

    /// Set permissions, passing in an Option of a type which we 
    /// can get a string from (via into). This method consumes and
    /// returns `self`, so it is convenient when using in a chained,
    /// fluent api, but requires reassignment if using "stand alone".
    /// 
    /// # Parameters
    /// 
    /// * `perms` - permissions of type Into<String>. 
    /// 
    /// # Examples
    /// 
    /// ## Fluent Style 
    /// ```
    /// use jsp::jspt::JsptMetadata;
    /// 
    /// let metadata = JsptMetadata::new()
    ///                 .set_permissions(Some("777"))
    ///                 .set_volume(true)
    ///                 .set_owner(Some("jgerber"));
    /// ```
    /// 
    /// ## Stand Alone
    /// ```
    /// use jsp::jspt::JsptMetadata;
    /// 
    /// let metadata = JsptMetadata::new();
    /// let metadata = metadata.set_permissions(Some("777"));
    /// ```
    /// Alternatively, we can make metadata mutable.
    /// ```
    /// use jsp::jspt::JsptMetadata;
    /// 
    /// let mut metadata = JsptMetadata::new();
    /// metadata = metadata.set_permissions(Some("777"));
    /// ```
    pub fn set_permissions<T>(mut self, perms: Option<T>) -> Self 
    where 
        T: Into<String> 
    {
        self.permissions = perms.map(|x| x.into());
        self
    }

    /// Retrieve permissions as an Option wrapped &str. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jsp::jspt::JsptMetadata;
    ///  
    /// let metadata = JsptMetadata::new()
    ///                 .set_permissions(Some("777"));
    /// if let Some(perms) = metadata.permissions() {
    ///     assert_eq!(perms, "777");
    /// }
    /// ```
    pub fn permissions(&self) -> Option<&str> {
        self.permissions.as_ref().map(|x| &**x)
    }

    /// Take the permissions as an Option<String> leaving None in its place.
    /// 
    /// # Parameters
    /// None
    /// 
    /// # Returns
    /// Permissions as Option<String>
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jsp::jspt::JsptMetadata;
    ///  
    /// let mut metadata = JsptMetadata::new()
    ///                 .set_permissions(Some("777"));
    /// let perms = metadata.take_permissions();
    /// assert_eq!(perms, Some("777".to_string()));
    /// ```
    pub fn take_permissions(&mut self) -> Option<String> {
        self.permissions.take()
    }

    /// Set varname given an Option wrapped type which implements Into<String>. 
    /// 
    /// Note that this method consumes and returns `self`. It is designed 
    /// to be optimal for fluent style api application. One must reassign if 
    /// used "stand alone".
    /// 
    /// # Parameters
    /// * `varname` - the variable name to set, as an Option wrapped type 
    /// that implements Into<String>. 
    /// 
    /// # Returns
    /// `self`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jsp::jspt::JsptMetadata;
    ///  
    /// let metadata = JsptMetadata::new()
    ///                 .set_varname(Some("JG_SHOW"));
    /// ```
    pub fn set_varname<T>(mut self, varname: Option<T>) -> Self 
    where 
        T: Into<String>
    {
        self.varname = varname.map(|x| x.into());
        self
    }

    /// Retrieve a reference to `varname` as an Option<&str>
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jsp::jspt::JsptMetadata;
    ///  
    /// let metadata = JsptMetadata::new()
    ///                 .set_varname(Some("JG_SHOW"));
    /// 
    /// if let Some(varname) = metadata.varname() {
    ///     assert_eq!(varname, "JG_SHOW");
    /// }
    /// ```
    pub fn varname(&self) -> Option<&str> {
        self.varname.as_ref().map(|x| &**x)
    }
    
    /// Take `varname` as an Option<String>, leaving None in its place. 
    ///
    /// # Examples
    /// 
    /// ```
    /// use jsp::jspt::JsptMetadata;
    ///  
    /// let mut metadata = JsptMetadata::new()
    ///                 .set_varname(Some("JG_SHOW"));
    /// 
    /// let varname = metadata.take_varname(); 
    /// assert_eq!(varname, Some("JG_SHOW".to_string()));
    /// ```
    pub fn take_varname(&mut self) -> Option<String> {
        self.varname.take()
    }

    /// Set `owner` given an Option wrapped type which implements `Into<String>`.
    /// 
    /// # Parameters
    /// 
    /// * `owner` - An 
    pub fn set_owner<T>(mut self, owner: Option<T>) -> Self 
    where
        T: Into<String>
    {
        self.owner = owner.map(|x| x.into());
        self
    }

    /// Retrieve a reference to `owner` as an Option wrapped `&str`.
    pub fn owner(&self) -> Option<&str> {
        self.owner.as_ref().map(|x| &**x)
    }

    /// Retrieve the `owner` as an Option wrapped String, leaving 
    /// None in its place. 
    pub fn take_owner(&mut self) -> Option<String> {
        self.owner.take()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_metadata() {
        let md = JsptMetadata::new();
        let expect = JsptMetadata {
            volume: false,
            permissions: None,
            varname: None,
            owner: None,
        };
        assert_eq!(md, expect);
    }

     #[test]
    fn can_create_metadata_and_set_volume() {
        let md = JsptMetadata::new().set_volume(true);
        let expect = JsptMetadata {
            volume: true,
            permissions: None,
            varname: None,
            owner: None,
        };
        assert_eq!(md, expect);
    }

     #[test]
    fn can_create_metadata_and_set_owner() {
        let md = JsptMetadata::new().set_volume(true).set_owner(Some("jgerber"));
        let expect = JsptMetadata {
            volume: true,
            permissions: None,
            varname: None,
            owner: Some("jgerber".to_string()),
        };
        assert_eq!(md, expect);
    }

     #[test]
    fn can_create_metadata_and_set_varname() {
        let md = JsptMetadata::new().set_volume(true).set_owner(Some("jgerber")).set_varname(Some("jg_show"));
        let expect = JsptMetadata {
            volume: true,
            permissions: None,
            varname: Some("jg_show".to_string()),
            owner: Some("jgerber".to_string()),
        };
        assert_eq!(md, expect);
    }

     #[test]
    fn can_create_metadata_and_set_perms() {
        let md = JsptMetadata::new()
                    .set_volume(true)
                    .set_owner(Some("jgerber"))
                    .set_varname(Some("jg_show"))
                    .set_permissions(Some("777"));

        let expect = JsptMetadata {
            volume: true,
            permissions: Some("777".to_string()),
            varname: Some("jg_show".to_string()),
            owner: Some("jgerber".to_string()),
        };
        assert_eq!(md, expect);
    }

    #[test]
    fn can_get_volume() {
        let md = JsptMetadata::new().set_volume(true);
        assert_eq!(md.is_volume(), true);
    }

    #[test]
    fn can_get_owner() {
        let md = JsptMetadata::new().set_volume(true).set_owner(Some("jgerber"));
        assert_eq!(md.owner(), Some("jgerber"));
    }

    #[test]
    fn can_take_owner() {
        let mut md = JsptMetadata::new().set_volume(true).set_owner(Some("jgerber"));
        assert_eq!(md.take_owner(), Some("jgerber".to_string()));
    }

    #[test]
    fn can_get_varname() {
        let md = JsptMetadata::new()
                    .set_volume(true)
                    .set_owner(Some("jgerber"))
                    .set_varname(Some("jg_foo"));
        assert_eq!(md.varname(), Some("jg_foo"));
    }

    #[test]
    fn can_take_varname() {
        let mut md = JsptMetadata::new()
                    .set_volume(true)
                    .set_owner(Some("jgerber"))
                    .set_varname(Some("jg_foo"));
        assert_eq!(md.take_varname(), Some("jg_foo".to_string()));
    }

    #[test]
    fn can_get_permissions() {
        let md = JsptMetadata::new()
                    .set_volume(true)
                    .set_owner(Some("jgerber"))
                    .set_varname(Some("jg_foo"))
                    .set_permissions(Some("777"));
        assert_eq!(md.permissions(), Some("777"));
    }

    #[test]
    fn can_take_permissions() {
        let mut md = JsptMetadata::new()
                    .set_volume(true)
                    .set_owner(Some("jgerber"))
                    .set_varname(Some("jg_foo"))
                    .set_permissions(Some("777"));
        assert_eq!(md.take_permissions(), Some("777".to_string()));
    }
}