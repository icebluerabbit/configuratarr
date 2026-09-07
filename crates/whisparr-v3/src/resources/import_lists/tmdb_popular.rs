use core_macros::fields_blob;

/// TMDb Popular import list — imports currently popular titles from TMDb,
/// narrowed by a set of filter criteria.
///
/// The `filterCriteria.*` wire names are not a typo: the C# settings class nests
/// a `TMDbFilterSettings` object, and the *arr schema builder flattens a nested
/// settings object into the `fields[]` blob under a dotted `<property>.<field>`
/// name. Whisparr's filter block carries two company-id filters and the language
/// filter that Radarr keeps at the top level.
#[fields_blob(
    implementation = "TMDbPopularImport",
    config_contract = "TMDbPopularSettings"
)]
pub struct TmdbPopularConfig {
    /// Category of popular titles to import. Integer enum:
    /// 1 = In Theaters, 2 = Popular, 3 = Top Rated, 4 = Upcoming.
    ///
    /// The wire name derives from the C# property `TMDbListType` — only the
    /// first character is lower-cased, giving `tMDbListType`.
    #[wire(name = "tMDbListType")]
    pub tmdb_list_type: Option<i32>,
    /// `filterCriteria.minVoteAverage` — minimum TMDb vote average (0.0–10.0).
    #[wire(name = "filterCriteria.minVoteAverage")]
    pub min_vote_average: Option<String>,
    /// `filterCriteria.minVotes` — minimum number of TMDb votes.
    #[wire(name = "filterCriteria.minVotes")]
    pub min_votes: Option<String>,
    /// `filterCriteria.certification` — single certification filter
    /// (`NR`, `G`, `PG`, `PG-13`, `R`, `NC-17`).
    #[wire(name = "filterCriteria.certification")]
    pub certification: Option<String>,
    /// `filterCriteria.includeGenreIds` — TMDb genre ids to include,
    /// comma- or pipe-separated.
    #[wire(name = "filterCriteria.includeGenreIds")]
    pub include_genre_ids: Option<String>,
    /// `filterCriteria.excludeGenreIds` — TMDb genre ids to exclude,
    /// comma- or pipe-separated.
    #[wire(name = "filterCriteria.excludeGenreIds")]
    pub exclude_genre_ids: Option<String>,
    /// `filterCriteria.includeCompanyIds` — TMDb company ids to include,
    /// comma- or pipe-separated.
    #[wire(name = "filterCriteria.includeCompanyIds")]
    pub include_company_ids: Option<String>,
    /// `filterCriteria.excludeCompanyIds` — TMDb company ids to exclude,
    /// comma- or pipe-separated.
    #[wire(name = "filterCriteria.excludeCompanyIds")]
    pub exclude_company_ids: Option<String>,
    /// `filterCriteria.languageCode` — original-language filter (integer enum
    /// corresponding to a TMDb language code).
    #[wire(name = "filterCriteria.languageCode")]
    pub language_code: Option<i32>,
}
