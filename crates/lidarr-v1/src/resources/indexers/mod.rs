//! Tagged-enum for indexer providers (`#[tagged]`). Each `#[variant("...")]`
//! binds an `implementation` string to a typed fields-blob variant. `#[fallback]`
//! catches any implementation we don't model and preserves it via `RawProvider`.

pub mod filelist;
pub mod gazelle;
pub mod headphones;
pub mod iptorrents;
pub mod newznab;
pub mod nyaa;
pub mod redacted;
pub mod torrent_rss;
pub mod torrentleech;
pub mod torznab;

pub use filelist::FileListConfig;
pub use gazelle::GazelleConfig;
pub use headphones::HeadphonesConfig;
pub use iptorrents::IpTorrentsConfig;
pub use newznab::NewznabConfig;
pub use nyaa::NyaaConfig;
pub use redacted::RedactedConfig;
pub use torrent_rss::TorrentRssConfig;
pub use torrentleech::TorrentLeechConfig;
pub use torznab::TorznabConfig;

use core_macros::tagged;

use crate::resources::raw_provider::RawProvider;

/// Discriminated union of all modelled Lidarr indexer implementations.
/// The `implementation` field on the wire selects the variant; `#[fallback]`
/// preserves any implementation not listed here as a passthrough blob.
#[tagged(by = "implementation")]
pub enum IndexerProvider {
    #[variant("FileList")]
    FileList(FileListConfig),
    #[variant("Gazelle")]
    Gazelle(GazelleConfig),
    #[variant("Headphones")]
    Headphones(HeadphonesConfig),
    #[variant("IPTorrents")]
    IpTorrents(IpTorrentsConfig),
    #[variant("Newznab")]
    Newznab(NewznabConfig),
    #[variant("Nyaa")]
    Nyaa(NyaaConfig),
    #[variant("Redacted")]
    Redacted(RedactedConfig),
    #[variant("TorrentRssIndexer")]
    TorrentRss(TorrentRssConfig),
    #[variant("Torrentleech")]
    TorrentLeech(TorrentLeechConfig),
    #[variant("Torznab")]
    Torznab(TorznabConfig),
    #[fallback]
    Unknown(RawProvider),
}
