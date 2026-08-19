use core_macros::wire_enum;

/// How often Komga automatically rescans a library's `root` directory.
///
/// The derived `SCREAMING_SNAKE_CASE` rename would produce `EVERY6_H` /
/// `EVERY12_H` for `Every6H` / `Every12H` (the digit breaks the word
/// boundary the renamer looks for), so those two variants carry an explicit
/// `#[variant(...)]` to match the API's actual `EVERY_6H` / `EVERY_12H`.
#[wire_enum(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScanInterval {
    /// Automatic scanning is disabled.
    Disabled,
    /// Rescan every hour.
    Hourly,
    /// Rescan every 6 hours.
    #[variant("EVERY_6H")]
    Every6H,
    /// Rescan every 12 hours.
    #[variant("EVERY_12H")]
    Every12H,
    /// Rescan once a day.
    Daily,
    /// Rescan once a week.
    Weekly,
    /// Unknown or future interval not yet modelled by this version.
    #[fallback]
    Unknown,
}
