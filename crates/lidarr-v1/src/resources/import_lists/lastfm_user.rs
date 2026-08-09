use core_macros::fields_blob;

/// Last.fm User import list — imports artists from a Last.fm user's library.
#[fields_blob(implementation = "LastFMUser", config_contract = "LastFMUserSettings")]
pub struct LastFmUserConfig {
    /// Last.fm username whose library to import.
    #[wire(name = "userId")]
    pub user_id: Option<String>,
    /// Number of artists to pull from the user's library.
    pub count: Option<i64>,
}
