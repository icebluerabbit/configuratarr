use core_macros::resource;

/// `/api/v1/config/downloadclient` — download client handling settings.
///
/// Prowlarr's download client config exposes only the server-assigned `id`;
/// there are no user-configurable fields at this endpoint.
#[resource(
    sync = singleton,
    read = get("/api/v1/config/downloadclient"),
    update = put("/api/v1/config/downloadclient/${self.id}"),
)]
pub struct DownloadClientConfig {
    #[id]
    pub id: Option<i32>,
}
