use core_macros::resource;

/// `/System/Configuration/metadata` — metadata storage behaviour.
#[resource(
    sync = singleton,
    case = pascal,
    read = get("/System/Configuration/metadata"),
    update = post("/System/Configuration/metadata"),
)]
pub struct MetadataConfiguration {
    /// Use File Creation Time For Date Added
    pub use_file_creation_time_for_date_added: bool,
}
