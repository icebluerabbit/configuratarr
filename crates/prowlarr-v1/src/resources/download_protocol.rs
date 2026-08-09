//! Unit-enum template (`#[wire_enum]`): a string scalar on the wire. Each
//! variant renders to its name under the chosen casing; `#[fallback]` catches
//! any value the API adds later that we don't model yet.

use core_macros::wire_enum;

/// Wire protocol used by a download client or indexer.
#[wire_enum(rename_all = "lowercase")]
pub enum DownloadProtocol {
    Usenet,
    Torrent,
    #[fallback]
    Unknown,
}
