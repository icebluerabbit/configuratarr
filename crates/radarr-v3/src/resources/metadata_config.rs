use core_macros::resource;

/// `/api/v3/config/metadata` — certification country for metadata lookups.
#[resource(
    sync = singleton,
    read = get("/api/v3/config/metadata"),
    update = put("/api/v3/config/metadata/${self.id}"),
)]
pub struct MetadataConfig {
    #[id]
    pub id: Option<i32>,
    /// ISO country code used to select the content rating system (e.g. `us` for MPAA, `gb` for BBFC).
    pub certification_country: Option<String>,
}
