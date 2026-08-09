//! Single source of truth for the configuratarr service registry.
//!
//! Holds *only* the table of known services. Each consumer supplies its own
//! callback macro that expands the table into the shape it needs — one list,
//! many shapes:
//!
//! - `core-config` expands it into the runtime [`ServiceInstance`] dispatch enum.
//! - `config-doc-gen` expands it into a list of rendered `<service>-config.md`.
//!
//! Adding a service is **one row here**, plus an optional-dep + feature line in
//! each consumer's manifest.
//!
//! Row syntax: `Variant => "type-tag" : crate::Type = "Human Title"`. The
//! `type-tag` doubles as the Cargo feature name that gates the service, so a
//! consumer can compile a single service with
//! `--no-default-features --features <tag>`.
//!
//! [`ServiceInstance`]: ../core_config/enum.ServiceInstance.html

/// Expand the service table through a caller-supplied callback macro `$cb`.
///
/// `$cb` is invoked once with the whole table; it pattern-matches rows of the
/// form `Variant => "tag" : path::Type = "Title"` and emits whatever shape it
/// needs (an enum, an array, statements, …).
#[macro_export]
macro_rules! service_registry {
    ($cb:ident) => {
        $cb! {
            RadarrV3 => "radarr-v3" : radarr_v3::RadarrV3 = "Radarr v3",
            SonarrV3 => "sonarr-v3" : sonarr_v3::SonarrV3 = "Sonarr v3",
            ProwlarrV1 => "prowlarr-v1" : prowlarr_v1::ProwlarrV1 = "Prowlarr v1",
            LidarrV1 => "lidarr-v1" : lidarr_v1::LidarrV1 = "Lidarr v1",
            JellyfinV11 => "jellyfin-v11" : jellyfin_v11::JellyfinV11 = "Jellyfin v11",
            BazarrV1 => "bazarr-v1" : bazarr_v1::BazarrV1 = "Bazarr v1",
            AutobrrV1 => "autobrr-v1" : autobrr_v1::AutobrrV1 = "Autobrr v1",
            LazyLibrarianV1 => "lazylibrarian-v1" : lazylibrarian_v1::LazyLibrarianV1 = "LazyLibrarian v1",
            CleanuparrV1 => "cleanuparr-v1" : cleanuparr_v1::CleanuparrV1 = "Cleanuparr v1",
        }
    };
}
