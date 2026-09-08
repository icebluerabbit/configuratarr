use core_lib::SecretValue;
use core_macros::fields_blob;

/// StashDB Favorites import list — imports scenes from the StashDB entities
/// (performers and/or studios) the authenticated account has favorited.
///
/// Whisparr-only: StashDB has no Radarr counterpart.
#[fields_blob(
    implementation = "StashDBFavoriteImport",
    config_contract = "StashDBFavoriteSettings"
)]
pub struct StashDbFavoriteConfig {
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
    /// Which kind of favorited entity to follow.
    /// One of `all`, `performer`, `studio`.
    pub filter: Option<String>,
    /// Tag StashIDs to filter on, comma-separated. Optional.
    // Renamed from `tags`: the shared `Provider` envelope already flattens a
    // `tags` field (tag-id references) into the same config namespace, so a
    // second `tags` here would be unreachable. Wire name is unchanged.
    #[wire(name = "tags")]
    pub stash_tags: Option<String>,
    /// How `tags` is applied. One of `includes`, `excludes`.
    #[wire(name = "tagsFilter")]
    pub tags_filter: Option<String>,
}
