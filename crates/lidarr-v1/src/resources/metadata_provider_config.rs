use core_macros::resource;

/// `/api/v1/config/metadataprovider` — music metadata source and audio tag write settings.
#[resource(
    sync = singleton,
    read = get("/api/v1/config/metadataprovider"),
    update = put("/api/v1/config/metadataprovider/${self.id}"),
)]
pub struct MetadataProviderConfig {
    #[id]
    pub id: Option<i32>,
    /// The metadata source Lidarr uses to look up artist and album information (e.g. `lastfm`).
    pub metadata_source: Option<String>,
    /// Controls when Lidarr writes audio tags to imported track files: `no`, `newFiles`, `allFiles`, or `sync`.
    #[default("no")]
    pub write_audio_tags: String,
    /// Removes embedded audio tags that are not managed by Lidarr from imported track files.
    pub scrub_audio_tags: bool,
    /// Embeds cover art into imported track files as an ID3/APE tag.
    pub embed_cover_art: bool,
}
