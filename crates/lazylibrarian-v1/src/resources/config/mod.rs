//! The LazyLibrarian configuration as a single custom singleton.
//!
//! LazyLibrarian has no bulk-config endpoint: a value is read with
//! `readCFG&name=<KEY>&group=<SECTION>` (which returns `[value]`) and written with
//! `writeCFG&name=<KEY>&group=<SECTION>&value=<v>`. So — like bazarr's `settings` —
//! this is a `sync = custom` singleton whose fields are `Option<Section>` (present =
//! manage that section, absent = leave it). Each section's fields carry their
//! uppercase LazyLibrarian `KEY` as the wire name, and the section field carries the
//! uppercase `SECTION` as *its* wire name, so encoding the config yields a nested
//! `{ SECTION: { KEY: value } }` — exactly the (group, name, value) triples the
//! reconcile feeds to `writeCFG`.
//!
//! Every non-runtime LazyLibrarian config section is modelled (see [`sections`],
//! generated from the spec `Config` schema); the reconcile is **section-generic**,
//! so it drives them all unchanged. Runtime-only sections (`TELEMETRY`, `TESTING`,
//! `GIT` version state) are excluded.

use core_lib::apply::Change;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, engine};
use core_macros::resource;
use serde_json::Value;

pub mod sections;

/// LazyLibrarian configuration — every managed section (all optional).
#[resource(sync = custom, case = snake, list = get("/api?cmd=getVersion"))]
pub struct Config {
    /// The `[General]` section.
    #[wire(name = "GENERAL")]
    pub general: Option<sections::General>,
    /// The `[Api]` section.
    #[wire(name = "API")]
    pub api: Option<sections::Api>,
    /// The `[Proxy]` section.
    #[wire(name = "PROXY")]
    pub proxy: Option<sections::Proxy>,
    /// The `[Postprocess]` section.
    #[wire(name = "POSTPROCESS")]
    pub postprocess: Option<sections::Postprocess>,
    /// The `[Webserver]` section.
    #[wire(name = "WEBSERVER")]
    pub web_server: Option<sections::WebServer>,
    /// The `[Logging]` section.
    #[wire(name = "LOGGING")]
    pub logging: Option<sections::Logging>,
    /// The `[Importer]` section.
    #[wire(name = "IMPORTER")]
    pub importer: Option<sections::Importer>,
    /// The `[Calibre]` section.
    #[wire(name = "CALIBRE")]
    pub calibre: Option<sections::Calibre>,
    /// The `[Sabnzbd]` section.
    #[wire(name = "SABNZBD")]
    pub sabnzbd: Option<sections::Sabnzbd>,
    /// The `[Nzbget]` section.
    #[wire(name = "NZBGET")]
    pub nzbget: Option<sections::Nzbget>,
    /// The `[Usenet]` section.
    #[wire(name = "USENET")]
    pub usenet: Option<sections::Usenet>,
    /// The `[Nzbmatrix]` section.
    #[wire(name = "NZBMATRIX")]
    pub nzbmatrix: Option<sections::Nzbmatrix>,
    /// The `[Torrent]` section.
    #[wire(name = "TORRENT")]
    pub torrent: Option<sections::Torrent>,
    /// The `[Rtorrent]` section.
    #[wire(name = "RTORRENT")]
    pub rtorrent: Option<sections::Rtorrent>,
    /// The `[Utorrent]` section.
    #[wire(name = "UTORRENT")]
    pub utorrent: Option<sections::Utorrent>,
    /// The `[Qbittorrent]` section.
    #[wire(name = "QBITTORRENT")]
    pub qbittorrent: Option<sections::Qbittorrent>,
    /// The `[Transmission]` section.
    #[wire(name = "TRANSMISSION")]
    pub transmission: Option<sections::Transmission>,
    /// The `[Deluge]` section.
    #[wire(name = "DELUGE")]
    pub deluge: Option<sections::Deluge>,
    /// The `[Synology]` section.
    #[wire(name = "SYNOLOGY")]
    pub synology: Option<sections::Synology>,
    /// The `[Abb]` section.
    #[wire(name = "ABB")]
    pub abb: Option<sections::Abb>,
    /// The `[Kat]` section.
    #[wire(name = "KAT")]
    pub kat: Option<sections::Kat>,
    /// The `[Tpb]` section.
    #[wire(name = "TPB")]
    pub tpb: Option<sections::Tpb>,
    /// The `[Tdl]` section.
    #[wire(name = "TDL")]
    pub tdl: Option<sections::Tdl>,
    /// The `[Bok]` section.
    #[wire(name = "BOK")]
    pub bok: Option<sections::Bok>,
    /// The `[Slsk]` section.
    #[wire(name = "SLSK")]
    pub slsk: Option<sections::Slsk>,
    /// The `[Anna]` section.
    #[wire(name = "ANNA")]
    pub anna: Option<sections::Anna>,
    /// The `[Lime]` section.
    #[wire(name = "LIME")]
    pub lime: Option<sections::Lime>,
    /// The `[Newzbin]` section.
    #[wire(name = "NEWZBIN")]
    pub newzbin: Option<sections::Newzbin>,
    /// The `[Searchscan]` section.
    #[wire(name = "SEARCHSCAN")]
    pub searchscan: Option<sections::Searchscan>,
    /// The `[Libraryscan]` section.
    #[wire(name = "LIBRARYSCAN")]
    pub libraryscan: Option<sections::Libraryscan>,
    /// The `[Comics]` section.
    #[wire(name = "COMICS")]
    pub comics: Option<sections::Comics>,
    /// The `[Magazines]` section.
    #[wire(name = "MAGAZINES")]
    pub magazines: Option<sections::Magazines>,
    /// The `[Twitter]` section.
    #[wire(name = "TWITTER")]
    pub twitter: Option<sections::Twitter>,
    /// The `[Boxcar]` section.
    #[wire(name = "BOXCAR")]
    pub boxcar: Option<sections::Boxcar>,
    /// The `[Pushbullet]` section.
    #[wire(name = "PUSHBULLET")]
    pub pushbullet: Option<sections::Pushbullet>,
    /// The `[Pushover]` section.
    #[wire(name = "PUSHOVER")]
    pub pushover: Option<sections::Pushover>,
    /// The `[Androidpn]` section.
    #[wire(name = "ANDROIDPN")]
    pub androidpn: Option<sections::AndroidPn>,
    /// The `[Telegram]` section.
    #[wire(name = "TELEGRAM")]
    pub telegram: Option<sections::Telegram>,
    /// The `[Prowl]` section.
    #[wire(name = "PROWL")]
    pub prowl: Option<sections::Prowl>,
    /// The `[Growl]` section.
    #[wire(name = "GROWL")]
    pub growl: Option<sections::Growl>,
    /// The `[Slack]` section.
    #[wire(name = "SLACK")]
    pub slack: Option<sections::Slack>,
    /// The `[Custom]` section.
    #[wire(name = "CUSTOM")]
    pub custom: Option<sections::Custom>,
    /// The `[Email]` section.
    #[wire(name = "EMAIL")]
    pub email: Option<sections::Email>,
    /// The `[Fmt]` section.
    #[wire(name = "FMT")]
    pub fmt: Option<sections::Fmt>,
    /// The `[Opds]` section.
    #[wire(name = "OPDS")]
    pub opds: Option<sections::Opds>,
    /// The `[Rss]` section.
    #[wire(name = "RSS")]
    pub rss_config: Option<sections::RssConfig>,
    /// The `[Preprocess]` section.
    #[wire(name = "PREPROCESS")]
    pub preprocess: Option<sections::Preprocess>,
}

/// Render a JSON scalar as the string LazyLibrarian's `writeCFG` expects.
///
/// **Booleans**: `writeCFG` treats any non-empty string as true (even `"0"`), so
/// `false` must be written as the empty string — and `readCFG` returns `""` for a
/// false bool. So `true` → `"1"`, `false` → `""`.
fn scalar(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Bool(b) => (if *b { "1" } else { "" }).into(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// `readCFG` returns the value wrapped as `[value]`. [`crate::http::get`] parses
/// the body, so a numeric value like `[77]` arrives as a one-element JSON
/// **array**, while a non-JSON value like `[slate]` stays a string — handle both,
/// plus the empty `[]` case.
fn unwrap_read(v: &Value) -> String {
    match v {
        Value::Array(a) if a.len() == 1 => scalar(&a[0]),
        Value::Array(_) => String::new(),
        Value::String(s) => s
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .unwrap_or(s)
            .to_string(),
        other => scalar(other),
    }
}

/// Compare a desired value against the current one, tolerating LazyLibrarian's
/// interchangeable boolean spellings: `1`/`true` = true; `0`/`false`/`""` (empty)
/// = false (a false bool reads back as the empty string).
fn same(want: &str, have: &str) -> bool {
    if want == have {
        return true;
    }
    let norm = |s: &str| match s.trim().to_ascii_lowercase().as_str() {
        "1" | "true" => Some(true),
        "0" | "false" | "" => Some(false),
        _ => None,
    };
    matches!((norm(want), norm(have)), (Some(a), Some(b)) if a == b)
}

impl CustomSync for Config {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let Some(cfg) = desired.first() else {
                return Ok(vec![]);
            };
            let wire = engine::encode_config::<Self>(cfg)?;
            let mut changes = Vec::new();
            let Some(sections) = wire.as_object() else {
                return Ok(changes);
            };
            for (section, obj) in sections {
                let Some(fields) = obj.as_object() else {
                    continue;
                };
                let mut wrote: Vec<String> = Vec::new();
                for (key, val) in fields {
                    let want = scalar(val);
                    let current = crate::http::get(
                        client,
                        &[("cmd", "readCFG"), ("name", key), ("group", section)],
                    )
                    .await?;
                    if same(&want, &unwrap_read(&current)) {
                        continue;
                    }
                    if execute {
                        crate::http::get(
                            client,
                            &[
                                ("cmd", "writeCFG"),
                                ("name", key),
                                ("group", section),
                                ("value", &want),
                            ],
                        )
                        .await?;
                    }
                    wrote.push(key.clone());
                }
                changes.push(if wrote.is_empty() {
                    Change::unchanged(section.clone())
                } else {
                    Change::updated(section.clone()).with("keys", wrote.join(", "))
                });
            }
            Ok(changes)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn scalar_writes_false_as_empty() {
        // writeCFG treats any non-empty string as true, so false must be "".
        assert_eq!(scalar(&json!(true)), "1");
        assert_eq!(scalar(&json!(false)), "");
        assert_eq!(scalar(&json!("GoogleBooks")), "GoogleBooks");
        assert_eq!(scalar(&json!(77)), "77");
        assert_eq!(scalar(&json!("$Author/$Title")), "$Author/$Title");
        assert_eq!(scalar(&json!(null)), "");
    }

    #[test]
    fn unwrap_read_handles_array_string_and_empty() {
        // numeric readCFG "[77]" is JSON-parsed by get_query into a 1-elem array
        assert_eq!(unwrap_read(&json!([77])), "77");
        // non-JSON "[slate]" stays a bracketed string
        assert_eq!(unwrap_read(&json!("[slate]")), "slate");
        // empty value
        assert_eq!(unwrap_read(&json!([])), "");
        assert_eq!(unwrap_read(&json!("[]")), "");
        // a plain string (no brackets) passes through
        assert_eq!(unwrap_read(&json!("plain")), "plain");
        // a true bool reads back as "1", false as ""
        assert_eq!(unwrap_read(&json!(["1"])), "1");
    }

    #[test]
    fn same_treats_empty_as_false() {
        assert!(same("1", "1"));
        assert!(same("77", "77"));
        assert!(same("1", "true"));
        assert!(same("", "false")); // false reads back empty
        assert!(same("", "0"));
        assert!(same("0", ""));
        assert!(!same("1", "")); // true != false
        assert!(!same("77", "80"));
        assert!(same("GoogleBooks", "GoogleBooks"));
        assert!(!same("GoogleBooks", "OpenLibrary"));
    }
}
