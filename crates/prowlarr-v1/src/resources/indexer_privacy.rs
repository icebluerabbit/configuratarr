use core_macros::wire_enum;

/// Privacy level of a Prowlarr indexer.
#[wire_enum(rename_all = "camelCase")]
pub enum IndexerPrivacy {
    /// Indexer is publicly accessible without registration.
    Public,
    /// Indexer requires registration or an invite but is not fully private.
    SemiPrivate,
    /// Indexer is private; access is restricted to invited members.
    Private,
    /// Unknown or future privacy level.
    #[fallback]
    Unknown,
}
