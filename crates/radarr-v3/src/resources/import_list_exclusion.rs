use core_macros::resource;

/// A movie excluded from all import lists, keyed by its TMDB id.
#[resource(
    sync = crud,
    list = get("/api/v3/exclusions"),
    create = post("/api/v3/exclusions"),
    update = put("/api/v3/exclusions/${self.id}"),
    delete = delete("/api/v3/exclusions/${self.id}"),
)]
pub struct ImportListExclusion {
    #[id]
    pub id: Option<i32>,
    /// Natural key — TMDB movie id, uniquely identifies the excluded movie.
    #[key]
    pub tmdb_id: i32,
    /// Title of the excluded movie, stored for display purposes.
    pub movie_title: Option<String>,
    /// Release year of the excluded movie.
    pub movie_year: i32,
}
