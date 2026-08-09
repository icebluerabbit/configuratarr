//! `/api/proxy` — SOCKS5/HTTP proxies that indexers and IRC networks route through.
//!
//! `sync = crud`, keyed by `name`: GET lists, POST creates, PUT/{id} updates,
//! DELETE/{id} removes (so prune works). The `pass` credential is write-only —
//! autobrr returns it `<redacted>` on read, so a declared `pass` always differs
//! from live and re-applies as an update (the standard write-only-secret wart).

use core_lib::SecretValue;
use core_macros::resource;

/// `/api/proxy` — a proxy autobrr can route indexer/IRC traffic through.
#[resource(
    sync = crud,
    case = snake,
    list = get("/api/proxy"),
    create = post("/api/proxy"),
    update = put("/api/proxy/${self.id}"),
    delete = delete("/api/proxy/${self.id}"),
)]
pub struct Proxy {
    /// Server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// Display name — its identity (`${ref.proxy.<name>}`).
    #[key]
    pub name: String,
    /// Whether the proxy is active.
    pub enabled: bool,
    /// Proxy kind: `SOCKS5` or `HTTP`.
    #[wire(name = "type")]
    pub proxy_type: String,
    /// Proxy URL, including scheme (e.g. `socks5://127.0.0.1:1080`).
    pub addr: String,
    /// Username, where the proxy authenticates.
    pub user: Option<String>,
    /// Password, where the proxy authenticates.
    pub pass: Option<SecretValue>,
    /// Connection timeout in seconds (`0` = client default).
    pub timeout: Option<i32>,
}
