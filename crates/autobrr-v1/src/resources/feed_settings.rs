use core_macros::nested;

/// Per-feed settings blob on a [`Feed`](crate::resources::feed::Feed).
#[nested(case = snake)]
pub struct FeedSettings {
    /// What to hand the download client for each item: `TORRENT` (default),
    /// `MAGNET`, or `NZB` (autobrr 1.76+, for usenet RSS feeds).
    pub download_type: Option<String>,
}
