use core_macros::fields_blob;

/// TMDb Keyword import list — imports movies tagged with a specific TMDb keyword.
#[fields_blob(
    implementation = "TMDbKeywordImport",
    config_contract = "TMDbKeywordSettings"
)]
pub struct TmdbKeywordConfig {
    /// TMDb keyword identifier used to filter movies.
    #[wire(name = "keywordId")]
    pub keyword_id: Option<String>,
}
