use core_lib::SecretValue;
use core_macros::fields_blob;

/// StashDB Performer import list — imports scenes featuring the given
/// performers, optionally narrowed by studio and tag.
///
/// Whisparr-only: StashDB has no Radarr counterpart.
#[fields_blob(
    implementation = "StashDBPerformerImport",
    config_contract = "StashDBPerformerSettings"
)]
pub struct StashDbPerformerConfig {
    /// StashDB API key. Required; the API returns it masked once saved.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Maximum number of scenes to fetch per sync. Must be greater than 0;
    /// StashDB caps a page at 100.
    pub limit: Option<i32>,
    /// Sort order for the fetched scenes (always descending).
    /// One of `released`, `created`, `trending`.
    pub sort: Option<String>,
    /// Only import scenes released on or after this date (`YYYY-MM-DD`).
    #[wire(name = "afterDate")]
    pub after_date: Option<String>,
    /// Performer StashIDs to follow, comma-separated. Required.
    pub performers: Option<String>,
    /// Studio StashIDs to filter on, comma-separated. Optional.
    pub studios: Option<String>,
    /// How `studios` is applied. One of `includes`, `excludes`.
    #[wire(name = "studiosFilter")]
    pub studios_filter: Option<String>,
    /// Tag StashIDs to filter on, comma-separated. Optional.
    // Renamed from `tags`: the shared `Provider` envelope already flattens a
    // `tags` field (tag-id references) into the same config namespace, so a
    // second `tags` here would be unreachable. Wire name is unchanged.
    #[wire(name = "tags")]
    pub stash_tags: Option<String>,
    /// How `tags` is applied. One of `includes`, `excludes`.
    #[wire(name = "tagsFilter")]
    pub tags_filter: Option<String>,
    /// Restrict results to studios the authenticated account has favorited.
    #[wire(name = "onlyFavoriteStudios")]
    pub only_favorite_studios: Option<bool>,
}
