use core_lib::SecretValue;
use core_macros::fields_blob;

/// Stash notification provider configuration — pushes scan/identify work to a
/// Stash instance after an import. Eros-only; the largest notification variant
/// (19 fields) and this fork's flagship integration.
#[fields_blob(implementation = "Stash", config_contract = "StashSettings")]
pub struct StashConfig {
    /// Stash hostname or IP address.
    pub host: String,
    /// Stash HTTP port (default 9999).
    pub port: i32,
    /// Connect to Stash over HTTPS instead of HTTP.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
    /// Stash API key for authentication.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Generate covers for new media during the Stash scan.
    #[wire(name = "generateCovers")]
    pub generate_covers: Option<bool>,
    /// Generate previews for new media during the Stash scan.
    #[wire(name = "generatePreviews")]
    pub generate_previews: Option<bool>,
    /// Generate image previews during the Stash scan. Requires
    /// `generate_previews` to also be enabled.
    #[wire(name = "generateImagePreviews")]
    pub generate_image_previews: Option<bool>,
    /// Generate sprites for new media during the Stash scan.
    #[wire(name = "generateSprites")]
    pub generate_sprites: Option<bool>,
    /// Generate perceptual hashes for new media during the Stash scan.
    #[wire(name = "generatePhashes")]
    pub generate_phashes: Option<bool>,
    /// Run the Stash metadata Identify task on newly scanned files.
    #[wire(name = "metadataIdentify")]
    pub metadata_identify: Option<bool>,
    /// Stash Box GraphQL endpoint used by the Identify task
    /// (default `https://stashdb.org/graphql`).
    #[wire(name = "stashBoxEndpoint")]
    pub stash_box_endpoint: Option<String>,
    /// Use Stash's builtin autotag source during the Identify task.
    #[wire(name = "builtinAutotag")]
    pub builtin_autotag: Option<bool>,
    /// Include male performers during the Identify task.
    #[wire(name = "includeMalePerformers")]
    pub include_male_performers: Option<bool>,
    /// Set the scene cover image during the Identify task.
    #[wire(name = "setCoverImage")]
    pub set_cover_image: Option<bool>,
    /// Skip matches that return more than one result.
    #[wire(name = "skipMultipleMatches")]
    pub skip_multiple_matches: Option<bool>,
    /// Stash tag id applied to scenes whose match was skipped.
    #[wire(name = "skipMultipleMatchTag")]
    pub skip_multiple_match_tag: Option<i32>,
    /// Mark scenes organized during the Identify task.
    #[wire(name = "setOrganized")]
    pub set_organized: Option<bool>,
    /// Whisparr-side path prefix to rewrite when Stash mounts the library at a
    /// different location.
    #[wire(name = "mapFrom")]
    pub map_from: Option<String>,
    /// Stash-side path prefix that `map_from` is rewritten to.
    #[wire(name = "mapTo")]
    pub map_to: Option<String>,
}
