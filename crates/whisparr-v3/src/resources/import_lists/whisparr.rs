use core_lib::SecretValue;
use core_macros::fields_blob;

/// Whisparr import list — imports the library of another Whisparr instance.
///
/// The id-valued filters below address the **remote** instance's quality
/// profiles, tags and root folders, so they are plain integers/strings and
/// carry no `#[reference]` to this instance's resources.
#[fields_blob(
    implementation = "WhisparrImport",
    config_contract = "WhisparrSettings"
)]
pub struct WhisparrConfig {
    /// Base URL of the source Whisparr instance, e.g. `http://localhost:6969`.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// API key of the source Whisparr instance.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Only import items whose quality profile on the source instance is one of
    /// these ids. Empty imports every profile.
    #[wire(name = "profileIds")]
    pub profile_ids: Vec<i32>,
    /// Only import items carrying one of these tag ids on the source instance.
    /// Empty imports regardless of tags.
    #[wire(name = "tagIds")]
    pub tag_ids: Vec<i32>,
    /// Only import items stored under one of these root folder paths on the
    /// source instance. Empty imports every root folder.
    #[wire(name = "rootFolderPaths")]
    pub root_folder_paths: Vec<String>,
}
