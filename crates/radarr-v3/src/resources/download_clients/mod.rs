//! Tagged-enum template (`#[tagged]`). The discriminator is the `implementation`
//! string in the wire object; each `#[variant("...")]` binds that value to a
//! typed fields-blob variant. `#[fallback]` catches any implementation we don't
//! model and preserves it via `RawProvider`.
//!
//! Subagents: add `pub mod <name>;` + a `#[variant("<Impl>")] <Name>(<Name>Config),`
//! line per download-client implementation in the spec. One file per variant,
//! following `qbittorrent.rs`.

pub mod aria2;
pub mod deluge;
pub mod flood;
pub mod freebox;
pub mod hadouken;
pub mod nzbget;
pub mod nzbvortex;
pub mod pneumatic;
pub mod qbittorrent;
pub mod rtorrent;
pub mod sabnzbd;
pub mod torrent_blackhole;
pub mod torrent_download_station;
pub mod transmission;
pub mod usenet_blackhole;
pub mod usenet_download_station;
pub mod utorrent;
pub mod vuze;

pub use aria2::Aria2Config;
pub use deluge::DelugeConfig;
pub use flood::FloodConfig;
pub use freebox::FreeboxConfig;
pub use hadouken::HadoukenConfig;
pub use nzbget::NzbgetConfig;
pub use nzbvortex::NzbvortexConfig;
pub use pneumatic::PneumaticConfig;
pub use qbittorrent::QBittorrentConfig;
pub use rtorrent::RTorrentConfig;
pub use sabnzbd::SabnzbdConfig;
pub use torrent_blackhole::TorrentBlackholeConfig;
pub use torrent_download_station::TorrentDownloadStationConfig;
pub use transmission::TransmissionConfig;
pub use usenet_blackhole::UsenetBlackholeConfig;
pub use usenet_download_station::UsenetDownloadStationConfig;
pub use utorrent::UTorrentConfig;
pub use vuze::VuzeConfig;

use core_macros::tagged;

use crate::resources::raw_provider::RawProvider;

#[tagged(by = "implementation")]
pub enum DownloadClientProvider {
    #[variant("Aria2")]
    Aria2(Aria2Config),
    #[variant("Deluge")]
    Deluge(DelugeConfig),
    #[variant("Flood")]
    Flood(FloodConfig),
    #[variant("TorrentFreeboxDownload")]
    Freebox(FreeboxConfig),
    #[variant("Hadouken")]
    Hadouken(HadoukenConfig),
    #[variant("Nzbget")]
    Nzbget(NzbgetConfig),
    #[variant("Nzbvortex")]
    Nzbvortex(NzbvortexConfig),
    #[variant("Pneumatic")]
    Pneumatic(PneumaticConfig),
    #[variant("QBittorrent")]
    QBittorrent(QBittorrentConfig),
    #[variant("RTorrent")]
    RTorrent(RTorrentConfig),
    #[variant("Sabnzbd")]
    Sabnzbd(SabnzbdConfig),
    #[variant("TorrentBlackhole")]
    TorrentBlackhole(TorrentBlackholeConfig),
    #[variant("TorrentDownloadStation")]
    TorrentDownloadStation(TorrentDownloadStationConfig),
    #[variant("Transmission")]
    Transmission(TransmissionConfig),
    #[variant("UsenetBlackhole")]
    UsenetBlackhole(UsenetBlackholeConfig),
    #[variant("UsenetDownloadStation")]
    UsenetDownloadStation(UsenetDownloadStationConfig),
    #[variant("UTorrent")]
    UTorrent(UTorrentConfig),
    #[variant("Vuze")]
    Vuze(VuzeConfig),
    #[fallback]
    Unknown(RawProvider),
}
