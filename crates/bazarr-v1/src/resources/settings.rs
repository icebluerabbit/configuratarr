//! Bazarr's entire configuration lives behind one endpoint,
//! `/api/system/settings`:
//!
//! * **GET** returns the settings as **nested snake_case JSON** —
//!   `{ "general": { "use_sonarr": … }, "sonarr": { … }, … }`.
//! * **POST** takes a flat `application/x-www-form-urlencoded` body whose keys
//!   are `settings-<section>-<field>` (e.g. `settings-general-use_sonarr`), and
//!   saves **only the keys present** — a natural sparse update. Bazarr normalises
//!   some values server-side and returns computed keys (`plex.encryption_key`,
//!   migration flags), so echoing the whole object back (a `singleton` PUT) would
//!   break idempotency — the write must be sparse.
//!
//! No stock sync fits that contract, so [`Settings`] is `sync = custom`. But the
//! encode is **not** hand-rolled: the hook runs the declared config through the
//! engine (`engine::config_present_to_wire::<Settings>`), which decodes it into
//! these typed structs (validating + dropping unknown keys), presence-masks it to
//! what the user wrote, and — since the resource is `case = snake` — emits keys
//! that already match bazarr's wire. The custom hook then only owns the parts
//! that are genuinely bazarr's contract: the form flattening, the sparse diff,
//! and `auth.password`'s md5 (bazarr stores it hashed). Language profiles + the
//! enabled-language set write through this same endpoint but as their own
//! side-channel form fields — [`crate::resources::languages::Languages`].

use core_lib::engine;
use core_lib::form;
use core_lib::{Change, CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use md5::{Digest, Md5};
use serde_json::Value;

use crate::resources::auth::Auth;
use crate::resources::backup::Backup;
use crate::resources::general::General;
use crate::resources::jellyfin::Jellyfin;
use crate::resources::log::Log;
use crate::resources::plex::Plex;
use crate::resources::postgresql::Postgresql;
use crate::resources::providers::addic7ed::Addic7ed;
use crate::resources::providers::anidb::AniDb;
use crate::resources::providers::animetosho::AnimeTosho;
use crate::resources::providers::anticaptcha::AntiCaptcha;
use crate::resources::providers::assrt::Assrt;
use crate::resources::providers::avistaz::AvistaZ;
use crate::resources::providers::betaseries::BetaSeries;
use crate::resources::providers::captchaai::CaptchaAi;
use crate::resources::providers::cinemaz::CinemaZ;
use crate::resources::providers::deathbycaptcha::DeathByCaptcha;
use crate::resources::providers::embeddedsubtitles::EmbeddedSubtitles;
use crate::resources::providers::hdbits::HdBits;
use crate::resources::providers::jimaku::Jimaku;
use crate::resources::providers::karagarga::KaraGarga;
use crate::resources::providers::ktuvit::Ktuvit;
use crate::resources::providers::legendasdivx::LegendasDivx;
use crate::resources::providers::legendasnet::LegendasNet;
use crate::resources::providers::napiprojekt::NapiProjekt;
use crate::resources::providers::napisy24::Napisy24;
use crate::resources::providers::opensubtitlescom::OpenSubtitlesCom;
use crate::resources::providers::pipocas::Pipocas;
use crate::resources::providers::subdl::SubDl;
use crate::resources::providers::subf2m::Subf2m;
use crate::resources::providers::subsarr::Subsarr;
use crate::resources::providers::subsource::SubSource;
use crate::resources::providers::subsro::SubsRo;
use crate::resources::providers::subx::SubX;
use crate::resources::providers::titlovi::Titlovi;
use crate::resources::providers::titulky::Titulky;
use crate::resources::providers::turkcealtyaziorg::TurkceAltyaziOrg;
use crate::resources::providers::whisperai::WhisperAi;
use crate::resources::providers::xsubs::XSubs;
use crate::resources::proxy::Proxy;
use crate::resources::radarr::Radarr;
use crate::resources::sonarr::Sonarr;
use crate::resources::subsync::Subsync;
use crate::resources::translator::Translator;

const SETTINGS_PATH: &str = "/api/system/settings";

/// `/api/system/settings` — bazarr's whole configuration. `case = snake` because
/// bazarr's JSON keys are the snake field names verbatim; each section is
/// `Option`: present = manage it, absent = leave bazarr's current values.
#[resource(sync = custom, case = snake, list = get("/api/system/settings"))]
pub struct Settings {
    /// General instance behaviour and downstream-app toggles.
    pub general: Option<General>,
    /// Sonarr connection.
    pub sonarr: Option<Sonarr>,
    /// Radarr connection.
    pub radarr: Option<Radarr>,
    /// Jellyfin connection.
    pub jellyfin: Option<Jellyfin>,
    /// Outbound proxy.
    pub proxy: Option<Proxy>,
    /// Automatic backups.
    pub backup: Option<Backup>,
    /// Subtitle synchronisation.
    pub subsync: Option<Subsync>,
    /// Web-UI authentication.
    pub auth: Option<Auth>,
    /// Plex integration.
    pub plex: Option<Plex>,
    /// PostgreSQL backend.
    pub postgresql: Option<Postgresql>,
    /// Machine-translation engine.
    pub translator: Option<Translator>,
    /// Log filtering.
    pub log: Option<Log>,
    /// Addic7ed provider settings.
    pub addic7ed: Option<Addic7ed>,
    /// AniDB provider settings.
    pub anidb: Option<AniDb>,
    /// AnimeTosho provider settings.
    pub animetosho: Option<AnimeTosho>,
    /// Anti-Captcha provider settings.
    pub anticaptcha: Option<AntiCaptcha>,
    /// Assrt provider settings.
    pub assrt: Option<Assrt>,
    /// AvistaZ provider settings.
    pub avistaz: Option<AvistaZ>,
    /// BetaSeries provider settings.
    pub betaseries: Option<BetaSeries>,
    /// CaptchaAI provider settings.
    pub captchaai: Option<CaptchaAi>,
    /// CinemaZ provider settings.
    pub cinemaz: Option<CinemaZ>,
    /// Death by Captcha provider settings.
    pub deathbycaptcha: Option<DeathByCaptcha>,
    /// Embedded subtitles provider settings.
    pub embeddedsubtitles: Option<EmbeddedSubtitles>,
    /// HDBits provider settings.
    pub hdbits: Option<HdBits>,
    /// Jimaku provider settings.
    pub jimaku: Option<Jimaku>,
    /// KaraGarga provider settings.
    pub karagarga: Option<KaraGarga>,
    /// Ktuvit provider settings.
    pub ktuvit: Option<Ktuvit>,
    /// LegendasDivx provider settings.
    pub legendasdivx: Option<LegendasDivx>,
    /// Legendas.net provider settings.
    pub legendasnet: Option<LegendasNet>,
    /// NapiProjekt provider settings.
    pub napiprojekt: Option<NapiProjekt>,
    /// Napisy24 provider settings.
    pub napisy24: Option<Napisy24>,
    /// OpenSubtitles.com provider settings.
    pub opensubtitlescom: Option<OpenSubtitlesCom>,
    /// Pipocas provider settings.
    pub pipocas: Option<Pipocas>,
    /// SubDL provider settings.
    pub subdl: Option<SubDl>,
    /// Subf2m provider settings.
    pub subf2m: Option<Subf2m>,
    /// Subsarr provider settings.
    pub subsarr: Option<Subsarr>,
    /// SubSource provider settings.
    pub subsource: Option<SubSource>,
    /// Subs.ro provider settings.
    pub subsro: Option<SubsRo>,
    /// SubX provider settings.
    pub subx: Option<SubX>,
    /// Titlovi provider settings.
    pub titlovi: Option<Titlovi>,
    /// Titulky provider settings.
    pub titulky: Option<Titulky>,
    /// Turkcealtyazi.org provider settings.
    pub turkcealtyaziorg: Option<TurkceAltyaziOrg>,
    /// Whisper AI provider settings.
    pub whisperai: Option<WhisperAi>,
    /// XSubs provider settings.
    pub xsubs: Option<XSubs>,
}

/// md5 hex digest, matching bazarr's `hashlib.md5(...).hexdigest()`.
fn md5_hex(s: &str) -> String {
    // md-5 0.11's digest output no longer implements `LowerHex`, so hex-encode
    // the bytes directly (works across the `GenericArray`/`Array` change).
    Md5::digest(s.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// `auth.password` is stored md5-hashed, so a plaintext-vs-hash compare never
/// matches. Compare the hash of the declared plaintext to the live hash (empty ↔
/// empty, which bazarr never hashes).
fn password_synced(want: &Value, have: Option<&Value>) -> bool {
    let want = want.as_str().unwrap_or("");
    let have = have.and_then(Value::as_str).unwrap_or("");
    if want.is_empty() {
        have.is_empty()
    } else {
        md5_hex(want) == have
    }
}

/// One field's declared wire value vs live — numeric-insensitive, with the
/// md5-hashed `auth.password` special-cased.
fn field_synced(section: &str, field: &str, want: &Value, have: Option<&Value>) -> bool {
    if section == "auth" && field == "password" {
        return password_synced(want, have);
    }
    match have {
        None => false,
        Some(h) => match (want.as_f64(), h.as_f64()) {
            (Some(a), Some(b)) => a == b,
            _ => want == h,
        },
    }
}

/// Every declared key in the wire object already matches the live settings.
fn in_sync(wire: &Value, live: &Value) -> bool {
    let Some(obj) = wire.as_object() else {
        return true;
    };
    obj.iter().all(|(section, sec_val)| {
        let Some(sec_obj) = sec_val.as_object() else {
            return true;
        };
        let live_sec = live.get(section);
        sec_obj.iter().all(|(field, want)| {
            field_synced(section, field, want, live_sec.and_then(|s| s.get(field)))
        })
    })
}

impl CustomSync for Settings {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let Some(cfg) = desired.first() else {
                return Ok(Vec::new());
            };

            // Encode through the descriptor: decode into the typed structs
            // (validate, drop unknown keys), presence-mask to declared keys, emit
            // snake wire that already matches bazarr's shape.
            let wire = engine::config_present_to_wire::<Self>(cfg)?;

            let live: Value = client.get(SETTINGS_PATH).await?;
            if in_sync(&wire, &live) {
                return Ok(vec![Change::unchanged("settings")]);
            }

            // `flatten` (core) renders the snake wire object into bazarr's
            // `settings-<section>-<field>` form pairs.
            let pairs = form::flatten(&wire, "settings");
            if execute {
                let _: Value = client.post_form(SETTINGS_PATH, &pairs).await?;
            }

            // Surface only the section names (never a secret value) for the plan view.
            let sections = wire
                .as_object()
                .map(|o| o.keys().cloned().collect::<Vec<_>>().join(", "))
                .unwrap_or_default();
            Ok(vec![Change::updated("settings").with("sections", sections)])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn auth_password_diffs_against_md5_hash() {
        let hash = json!("5ebe2294ecd0e0f08eab7690d2a6ee69"); // md5("secret")
        assert!(password_synced(&json!("secret"), Some(&hash)));
        assert!(!password_synced(&json!("wrong"), Some(&hash)));
        assert!(password_synced(&json!(""), Some(&json!(""))));
        assert!(!password_synced(&json!("secret"), Some(&json!(""))));
    }

    #[test]
    fn only_auth_password_is_hash_compared() {
        let hash = "5ebe2294ecd0e0f08eab7690d2a6ee69";
        assert!(field_synced(
            "auth",
            "password",
            &json!("secret"),
            Some(&json!(hash))
        ));
        assert!(!field_synced(
            "auth",
            "username",
            &json!("secret"),
            Some(&json!(hash))
        ));
    }

    #[test]
    fn in_sync_only_checks_declared_keys() {
        let live = json!({
            "general": { "use_sonarr": true, "minimum_score": 90, "theme": "auto" },
            "sonarr": { "port": 8989 },
        });
        assert!(in_sync(
            &json!({ "general": { "use_sonarr": true } }),
            &live
        ));
        assert!(in_sync(&json!({ "sonarr": { "port": 8989 } }), &live));
        assert!(!in_sync(
            &json!({ "general": { "use_sonarr": false } }),
            &live
        ));
        assert!(!in_sync(
            &json!({ "general": { "single_language": true } }),
            &live
        ));
    }
}
