use core_macros::{nested, wire_enum};

/// Whether `blocklist_path` entries are treated as a blacklist (blocked) or a
/// whitelist (only these allowed).
#[wire_enum]
pub enum BlocklistType {
    /// Items matching `blocklist_path` are blocked.
    Blacklist,
    /// Only items matching `blocklist_path` are allowed; everything else is
    /// blocked.
    Whitelist,
    /// Unknown or future blocklist type not yet modelled by this version.
    #[fallback]
    Unknown,
}

/// Malware blocklist settings for a single *arr instance.
///
/// Embedded in [`crate::resources::malware_blocker::MalwareBlocker`] under
/// the `sonarr`/`radarr`/`lidarr`/`readarr`/`whisparr` keys.
#[nested]
pub struct BlocklistSettings {
    /// Enables malware blocking for this *arr instance.
    pub enabled: Option<bool>,
    /// Whether `blocklist_path` is a blacklist or a whitelist.
    pub blocklist_type: Option<BlocklistType>,
    /// http(s) URL or an existing local file path.
    pub blocklist_path: Option<String>,
}
