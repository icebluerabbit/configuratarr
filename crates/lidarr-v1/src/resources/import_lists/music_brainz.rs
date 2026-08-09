use core_macros::fields_blob;

/// MusicBrainz Series import list — imports artists from a MusicBrainz series.
#[fields_blob(
    implementation = "MusicBrainzSeries",
    config_contract = "MusicBrainzSeriesSettings"
)]
pub struct MusicBrainzConfig {
    /// MusicBrainz series MBID to import artists from.
    #[wire(name = "seriesId")]
    pub series_id: Option<String>,
}
