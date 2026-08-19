use core_macros::wire_enum;

/// Which book's cover Komga uses as the series' cover image.
#[wire_enum(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SeriesCover {
    /// Always use the first book's cover.
    First,
    /// Use the first unread book's cover, falling back to the first book.
    FirstUnreadOrFirst,
    /// Use the first unread book's cover, falling back to the last book.
    FirstUnreadOrLast,
    /// Always use the last book's cover.
    Last,
    /// Unknown or future option not yet modelled by this version.
    #[fallback]
    Unknown,
}
