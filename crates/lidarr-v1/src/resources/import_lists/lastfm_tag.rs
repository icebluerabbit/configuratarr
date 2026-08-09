use core_macros::fields_blob;

/// Last.fm Tag import list — imports artists from a Last.fm tag.
#[fields_blob(implementation = "LastFmTag", config_contract = "LastFmTagSettings")]
pub struct LastFmTagConfig {
    /// Last.fm tag identifier to pull artists from.
    #[wire(name = "tagId")]
    pub tag_id: Option<String>,
    /// Number of artists to pull from the tag list.
    pub count: Option<i64>,
}
