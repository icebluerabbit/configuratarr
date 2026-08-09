use core_macros::fields_blob;

/// TMDb Popular import list — imports currently popular movies from TMDb with filter criteria.
#[fields_blob(
    implementation = "TMDbPopularImport",
    config_contract = "TMDbPopularSettings"
)]
pub struct TmdbPopularConfig {
    /// Category of popular movies to import (integer enum; e.g. popular, top-rated, upcoming).
    #[wire(name = "tMDbListType")]
    pub tmdb_list_type: Option<i32>,
    /// Language filter for returned movies (integer enum corresponding to a language code).
    #[wire(name = "languageCode")]
    pub language_code: Option<i32>,
    /// filterCriteria.minVoteAverage — minimum vote average filter.
    #[wire(name = "filterCriteria.minVoteAverage")]
    pub min_vote_average: Option<String>,
    /// filterCriteria.minVotes — minimum vote count filter.
    #[wire(name = "filterCriteria.minVotes")]
    pub min_votes: Option<String>,
    /// filterCriteria.certification — certification filter (e.g. "PG-13").
    #[wire(name = "filterCriteria.certification")]
    pub certification: Option<String>,
    /// filterCriteria.includeGenreIds — genre IDs to include.
    #[wire(name = "filterCriteria.includeGenreIds")]
    pub include_genre_ids: Option<String>,
    /// filterCriteria.excludeGenreIds — genre IDs to exclude.
    #[wire(name = "filterCriteria.excludeGenreIds")]
    pub exclude_genre_ids: Option<String>,
}
