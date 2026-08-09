//! LazyLibrarian provider families.
//!
//! Each family is a config.ini array section (`Newznab_<n>`, `Torznab_<n>`,
//! `RSS_<n>`, `IRC_<n>`, `GEN_<n>`), keyed by `DISPNAME`, listed via
//! `listProviders`. All families are full **upsert** (`sync = custom`): create if
//! absent, update to match, no prune (no `Change`-expressible delete).
//!
//! Write path — `addProvider` is buggy: it drops `ENABLED` (raw assignment instead
//! of `set_bool`), so a provider created with it lands **disabled** regardless of
//! how the flag is passed. `changeProvider` sets fields correctly (`set_bool`). So
//! every provider is written in two steps: first `addProvider` a **stub** (`type` +
//! `name`=DISPNAME + host) just to create the config slot, then
//! `changeProvider&name=<internal NAME>` with the full field set, including
//! `ENABLED`. Matching the update by the provider's **internal `NAME`** (`RSS_0`,
//! `Newznab_2`) rather than its DISPNAME is what makes this work for *every* family
//! — a DISPNAME-matched `changeProvider` fails for rss/irc/gen (`Invalid parameter:
//! name`). One apply lands a provider fully configured and enabled.
//!
//! Apprise providers (`APPRISE_<n>`) are **not** modelled: `addProvider` accepts
//! only `newznab/torznab/rss/gen/irc` and `listProviders` returns no apprise array,
//! so the API cannot manage them.

use core_lib::apply::Change;
use core_lib::{Described, HttpClient, engine};
use serde_json::Value;

pub mod direct;
pub mod irc;
pub mod newznab;
pub mod rss;
pub mod torznab;

/// Render a JSON scalar as a query-string value. Objects/arrays never occur in a
/// provider's flat field set.
fn scalar(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Bool(b) => (if *b { "1" } else { "0" }).into(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Compare a desired value against a live one as strings, tolerating
/// LazyLibrarian's interchangeable boolean spellings: `1`/`true` = true;
/// `0`/`false`/`""` (empty) = false. `listProviders` returns **every** field as a
/// string (`"0"`, `""`, `"1"`) and a *disabled* provider reads its `ENABLED` back
/// as `""`, so a raw typed `==` never matches — normalise both sides first.
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

/// Idempotency predicate: every declared (wire) field already matches the live
/// provider (compared as normalised strings). `listProviders` returns the full
/// record (including fields we don't manage), so a subset match means "in sync".
fn in_sync(wire: &Value, live: &Value) -> bool {
    wire.as_object()
        .map(|o| {
            o.iter().all(|(k, v)| {
                let have = live.get(k).map(scalar).unwrap_or_default();
                same(&scalar(v), &have)
            })
        })
        .unwrap_or(false)
}

/// `addProvider` stub params — the minimum to create the slot: `type`, `name`
/// (DISPNAME), and a host. `addProvider` requires a `HOST` (or `SERVER` for IRC)
/// and drops everything else meaningfully, so the real values are set afterwards
/// via `changeProvider`.
fn stub_params(add_type: &str, wire: &Value) -> anyhow::Result<Vec<(String, String)>> {
    let obj = wire
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("provider wire is not an object"))?;
    let dispname = obj
        .get("DISPNAME")
        .map(scalar)
        .ok_or_else(|| anyhow::anyhow!("provider is missing DISPNAME"))?;
    let (host_key, host_val) = obj
        .get("HOST")
        .map(|h| ("HOST", scalar(h)))
        .or_else(|| obj.get("SERVER").map(|s| ("SERVER", scalar(s))))
        .ok_or_else(|| anyhow::anyhow!("provider {dispname} needs HOST or SERVER to create"))?;
    Ok(vec![
        ("cmd".into(), "addProvider".into()),
        ("type".into(), add_type.into()),
        ("name".into(), dispname),
        (host_key.into(), host_val),
    ])
}

/// `changeProvider` params — matched by the provider's internal `NAME`, setting
/// every managed field (incl. `ENABLED`, which `changeProvider` writes via
/// `set_bool`). `DISPNAME` is the identity (already set on create) and is not
/// re-sent.
fn change_params(name: &str, wire: &Value) -> Vec<(String, String)> {
    let mut pairs = vec![
        ("cmd".into(), "changeProvider".into()),
        ("name".into(), name.to_string()),
    ];
    if let Some(obj) = wire.as_object() {
        for (k, v) in obj {
            if k == "DISPNAME" {
                continue;
            }
            pairs.push((k.clone(), scalar(v)));
        }
    }
    pairs
}

/// The `[family]` array from `listProviders` (e.g. `newznab`, `direct`).
async fn list_family(client: &HttpClient, family: &str) -> anyhow::Result<Vec<Value>> {
    let all = crate::http::get(client, &[("cmd", "listProviders")]).await?;
    Ok(all
        .get(family)
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default())
}

/// Issue an `add`/`changeProvider` call and surface its JSON envelope result:
/// these commands return `{Success, Data, Error}`, so a `Success: false` (e.g.
/// `Missing parameter: HOST`) must become a real error, not a silent no-op.
async fn call(client: &HttpClient, pairs: &[(String, String)]) -> anyhow::Result<()> {
    let refs: Vec<(&str, &str)> = pairs
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    let resp = crate::http::get(client, &refs).await?;
    if resp.get("Success") == Some(&Value::Bool(false)) {
        let msg = resp
            .pointer("/Error/Message")
            .and_then(Value::as_str)
            .unwrap_or("request failed");
        anyhow::bail!("lazylibrarian: {msg}");
    }
    Ok(())
}

/// Look up a provider's internal `NAME` by its `DISPNAME` in a live list.
fn name_of<'a>(live: &'a [Value], dispname: &str) -> Option<&'a str> {
    live.iter()
        .find(|l| l.get("DISPNAME").and_then(Value::as_str) == Some(dispname))
        .and_then(|l| l.get("NAME"))
        .and_then(Value::as_str)
}

/// Upsert reconcile for one provider family.
///
/// `family` is the key under `listProviders`' result; `add_type` is the
/// `addProvider&type=` value. `desired` are wire-encoded records (UPPERCASE keys
/// incl. `DISPNAME`). For each: unchanged if the live record already matches; else
/// `changeProvider` (by internal NAME) to converge; a fresh one is `addProvider`'d
/// as a stub first, then re-listed to learn its NAME. No prune.
async fn reconcile_family(
    client: &HttpClient,
    desired: &[Value],
    execute: bool,
    family: &'static str,
    add_type: &'static str,
) -> anyhow::Result<Vec<Change>> {
    let mut live = list_family(client, family).await?;
    let mut changes = Vec::with_capacity(desired.len());

    for wire in desired {
        let dispname = wire
            .get("DISPNAME")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("provider is missing DISPNAME"))?;

        let existing = live
            .iter()
            .find(|l| l.get("DISPNAME").and_then(Value::as_str) == Some(dispname))
            .cloned();

        let change = match existing {
            Some(l) if in_sync(wire, &l) => Change::unchanged(dispname),
            Some(l) => {
                if execute {
                    let name = l.get("NAME").and_then(Value::as_str).ok_or_else(|| {
                        anyhow::anyhow!("provider {dispname} has no internal NAME")
                    })?;
                    call(client, &change_params(name, wire)).await?;
                }
                Change::updated(dispname)
            }
            None => {
                if execute {
                    // 1) stub-create the slot, 2) re-list to learn its NAME,
                    // 3) set every field (incl. ENABLED) via changeProvider.
                    call(client, &stub_params(add_type, wire)?).await?;
                    live = list_family(client, family).await?;
                    let name = name_of(&live, dispname)
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "created provider {dispname} not found in listProviders"
                            )
                        })?
                        .to_string();
                    call(client, &change_params(&name, wire)).await?;
                }
                Change::created(dispname)
            }
        };
        changes.push(change);
    }
    Ok(changes)
}

/// Encode a family's desired config items to wire form, then run the shared
/// reconcile. Called from each provider's `CustomSync::reconcile`.
async fn reconcile_encoded<T: Described>(
    client: &HttpClient,
    desired: &[Value],
    execute: bool,
    family: &'static str,
    add_type: &'static str,
) -> anyhow::Result<Vec<Change>> {
    let wire: Vec<Value> = desired
        .iter()
        .map(|c| engine::encode_config::<T>(c))
        .collect::<anyhow::Result<_>>()?;
    reconcile_family(client, &wire, execute, family, add_type).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn scalar_renders_query_values() {
        assert_eq!(scalar(&json!(true)), "1");
        assert_eq!(scalar(&json!(false)), "0");
        assert_eq!(scalar(&json!("host")), "host");
        assert_eq!(scalar(&json!(8080)), "8080");
        assert_eq!(scalar(&json!(null)), "");
    }

    #[test]
    fn same_normalises_bool_spellings_and_empty() {
        assert!(same("1", "1"));
        assert!(same("1", "true"));
        assert!(same("0", "false"));
        assert!(same("0", "")); // disabled reads back as ""
        assert!(same("", "false"));
        assert!(!same("1", "")); // enabled != disabled
        assert!(!same("1", "0"));
        assert!(same("https://x", "https://x"));
        assert!(!same("https://x", "https://y"));
    }

    #[test]
    fn in_sync_is_a_string_normalised_subset() {
        // live returns EVERY field as a string, plus fields we don't manage.
        let wire = json!({ "DISPNAME": "x", "ENABLED": true, "DLPRIORITY": 0, "HOST": "h" });
        let live = json!({ "DISPNAME": "x", "ENABLED": "1", "DLPRIORITY": "0",
                           "HOST": "h", "APICOUNT": "3" });
        assert!(in_sync(&wire, &live), "typed wire should match string live");

        // enabled true vs disabled ("") → drift
        let disabled = json!({ "DISPNAME": "x", "ENABLED": "", "DLPRIORITY": "0", "HOST": "h" });
        assert!(!in_sync(&wire, &disabled));

        // a declared field with a different value → drift
        let moved = json!({ "DISPNAME": "x", "ENABLED": "1", "DLPRIORITY": "0", "HOST": "other" });
        assert!(!in_sync(&wire, &moved));

        // a *disabled* provider we want disabled → in sync (0 == "")
        let want_off = json!({ "DISPNAME": "x", "ENABLED": false });
        assert!(in_sync(&want_off, &disabled));
    }

    #[test]
    fn stub_params_are_minimal_and_pick_host_or_server() {
        // newznab/etc use HOST; the stub carries only type/name/host.
        let nz = json!({ "DISPNAME": "x", "HOST": "h", "API": "k", "ENABLED": true });
        let p = stub_params("newznab", &nz).unwrap();
        assert!(p.contains(&("type".into(), "newznab".into())));
        assert!(p.contains(&("name".into(), "x".into())));
        assert!(p.contains(&("HOST".into(), "h".into())));
        assert!(!p.iter().any(|(k, _)| k == "ENABLED" || k == "API"));

        // IRC uses SERVER as its host key.
        let irc = json!({ "DISPNAME": "y", "SERVER": "irc.example.com" });
        let p = stub_params("irc", &irc).unwrap();
        assert!(p.contains(&("SERVER".into(), "irc.example.com".into())));

        // no host at all → error (addProvider would bail).
        assert!(stub_params("rss", &json!({ "DISPNAME": "z" })).is_err());
    }

    #[test]
    fn change_params_set_all_fields_including_enabled_but_not_dispname() {
        let wire = json!({ "DISPNAME": "x", "ENABLED": true, "HOST": "h", "DLPRIORITY": 0 });
        let p = change_params("RSS_0", &wire);
        assert!(p.contains(&("cmd".into(), "changeProvider".into())));
        assert!(p.contains(&("name".into(), "RSS_0".into())));
        assert!(p.contains(&("ENABLED".into(), "1".into()))); // set_bool via "1"
        assert!(p.contains(&("HOST".into(), "h".into())));
        assert!(p.contains(&("DLPRIORITY".into(), "0".into())));
        // DISPNAME is the identity — never re-sent as a field.
        assert!(!p.iter().any(|(k, _)| k == "DISPNAME"));
    }

    #[test]
    fn name_of_maps_dispname_to_internal_name() {
        let live = vec![
            json!({ "NAME": "RSS_0", "DISPNAME": "mine" }),
            json!({ "NAME": "RSS_1", "DISPNAME": "" }),
        ];
        assert_eq!(name_of(&live, "mine"), Some("RSS_0"));
        assert_eq!(name_of(&live, "absent"), None);
    }
}
