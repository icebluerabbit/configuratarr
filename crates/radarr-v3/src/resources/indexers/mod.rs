//! Tagged-enum for indexer providers (`#[tagged]`). Each `#[variant("...")]`
//! binds an `implementation` string to a typed fields-blob variant. `#[fallback]`
//! catches any implementation we don't model and preserves it via `RawProvider`.

pub mod filelist;
pub mod hdbits;
pub mod iptorrents;
pub mod newznab;
pub mod nyaa;
pub mod pass_the_popcorn;
pub mod torrent_potato;
pub mod torrent_rss;
pub mod torznab;

pub use filelist::FileListConfig;
pub use hdbits::HdBitsConfig;
pub use iptorrents::IpTorrentsConfig;
pub use newznab::NewznabConfig;
pub use nyaa::NyaaConfig;
pub use pass_the_popcorn::PassThePopcornConfig;
pub use torrent_potato::TorrentPotatoConfig;
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
    #[variant("Nyaa")]
    Nyaa(NyaaConfig),
    #[variant("PassThePopcorn")]
    PassThePopcorn(PassThePopcornConfig),
    #[variant("TorrentPotato")]
    TorrentPotato(TorrentPotatoConfig),
    #[variant("TorrentRssIndexer")]
    TorrentRss(TorrentRssConfig),
    #[variant("Torznab")]
    Torznab(TorznabConfig),
    #[fallback]
    Unknown(RawProvider),
}
