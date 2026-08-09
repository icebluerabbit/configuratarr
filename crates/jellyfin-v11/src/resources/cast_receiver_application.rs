use core_macros::nested;

/// A Google Cast receiver application entry.
#[nested(case = pascal)]
pub struct CastReceiverApplication {
    /// Id
    pub id: Option<String>,
    /// Name
    pub name: Option<String>,
}
