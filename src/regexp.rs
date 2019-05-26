use regex::*;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Regexp is a newtype wrapper around Regex that provides ordering
/// and equality tests against the types we are likely to encounter
/// including OsString.
///
/// In general, documentation for the various methods and trait
/// implementations may be found in the regex library documentation.
#[derive(Debug, Clone)]
pub struct Regexp(pub Regex);

impl PartialOrd for Regexp {
    fn partial_cmp(&self, other: &Regexp) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Regexp {
    fn cmp(&self, other: &Regexp) -> Ordering {
        self.0.as_str().cmp(&other.0.as_str())
    }
}

impl PartialEq for Regexp {
    fn eq(&self, other: &Regexp) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Display for Regexp {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Regexp {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Regexp, regex::Error> {
        Regexp::new(s)
    }
}

impl Eq for Regexp {}

impl Regexp {

    /// new up an Regexp given a str.
    ///
    /// # Parameters
    /// * `r` - The raw regex string. This should be anchored in
    ///         "^...$" as the rust regex library does not acnhor
    ///         matches.
    /// # Returns
    ///    Result wrapping a Regexp or a regex::Error
    pub fn new(r: &str) -> Result<Regexp, regex::Error> {
        let regx = Regex::new(r)?;
        Ok(Regexp(regx))
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.0.is_match(text)
    }

    pub fn find<'t>(&self, text: &'t str) -> Option<Match<'t>> {
        self.0.find(text)
    }

    pub fn find_iter<'r, 't>(&'r self, text: &'t str) -> Matches<'r, 't> {
        self.0.find_iter(text)
    }

    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {
        self.0.captures(text)
    }

    pub fn captures_iter<'r, 't>(&'r self, text: &'t str) -> CaptureMatches<'r, 't> {
        self.0.captures_iter(text)
    }

    pub fn split<'r, 't>(&'r self, text: &'t str) -> Split<'r, 't> {
        self.0.split(text)
    }

    pub fn splitn<'r, 't>(&'r self, text: &'t str, limit: usize) -> SplitN<'r, 't> {
        self.0.splitn(text, limit)
    }

    pub fn replace<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str> {
        self.0.replace(text, rep)
    }
    pub fn replace_all<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str> {
        self.0.replace_all(text, rep)
    }

    pub fn replacen<'t, R: Replacer>(&self, text: &'t str, limit: usize, rep: R) -> Cow<'t, str> {
        self.0.replacen(text, limit, rep)
    }
}

/// advanced impls
impl Regexp {
    pub fn shortest_match(&self, text: &str) -> Option<usize> {
        self.0.shortest_match(text)
    }

    pub fn shortest_match_at(&self, text: &str, start: usize) -> Option<usize> {
        self.0.shortest_match_at(text, start)
    }

    pub fn is_match_at(&self, text: &str, start: usize) -> bool {
        self.0.is_match_at(text, start)
    }

    pub fn find_at<'t>(&self, text: &'t str, start: usize) -> Option<Match<'t>> {
        self.0.find_at(text, start)
    }

    pub fn captures_read<'t>(
        &self,
        locs: &mut CaptureLocations,
        text: &'t str,
    ) -> Option<Match<'t>> {
        self.0.captures_read(locs, text)
    }

    pub fn captures_read_at<'t>(
        &self,
        locs: &mut CaptureLocations,
        text: &'t str,
        start: usize,
    ) -> Option<Match<'t>> {
        self.0.captures_read_at(locs, text, start)
    }
}

// axuilliary methods
impl Regexp {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn capture_names(&self) -> CaptureNames {
        self.0.capture_names()
    }

    pub fn captures_len(&self) -> usize {
        self.0.captures_len()
    }

    pub fn capture_locations(&self) -> CaptureLocations {
        self.0.capture_locations()
    }
}
