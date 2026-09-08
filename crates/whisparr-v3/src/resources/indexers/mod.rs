//! Tagged-enum for indexer providers (`#[tagged]`). Each `#[variant("...")]`
//! binds an `implementation` string to a typed fields-blob variant. `#[fallback]`
//! catches any implementation we don't model and preserves it via `RawProvider`.
//!
//! eros (Whisparr V3) ships 6 indexer implementations — a subset of radarr's:
//! no `Nyaa`, `PassThePopcorn`, or `TorrentPotato`. Every torrent variant here
//! nests `seedRatio`/`seedTime` under a dotted `seedCriteria.*` wire name (see
//! `Indexers/ITorrentIndexerSettings.cs` — eros replaced radarr's flat fields
//! with a `SeedCriteriaSettings` sub-object) and adds
//! `rejectBlocklistedTorrentHashesWhileGrabbing`. All 6 (including `Newznab`)
//! add `failDownloads`.

pub mod filelist;
pub mod hdbits;
pub mod iptorrents;
pub mod newznab;
pub mod torrent_rss;
pub mod torznab;

pub use filelist::FileListConfig;
pub use hdbits::HdBitsConfig;
pub use iptorrents::IpTorrentsConfig;
pub use newznab::NewznabConfig;
pub use torrent_rss::TorrentRssConfig;
pub use torznab::TorznabConfig;

use core_macros::tagged;

use crate::resources::raw_provider::RawProvider;

#[tagged(by = "implementation")]
pub enum IndexerProvider {
    #[variant("FileList")]
    FileList(FileListConfig),
    #[variant("HDBits")]
    HdBits(HdBitsConfig),
    #[variant("IPTorrents")]
    IpTorrents(IpTorrentsConfig),
    #[variant("Newznab")]
    Newznab(NewznabConfig),
    #[variant("TorrentRssIndexer")]
    TorrentRss(TorrentRssConfig),
    #[variant("Torznab")]
    Torznab(TorznabConfig),
    #[fallback]
    Unknown(RawProvider),
}
