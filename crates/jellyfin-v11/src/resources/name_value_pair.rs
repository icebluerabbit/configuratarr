use core_macros::nested;

/// A generic name/value pair.
#[nested(case = pascal)]
pub struct NameValuePair {
    /// Name
    pub name: Option<String>,
    /// Value
    pub value: Option<String>,
}
