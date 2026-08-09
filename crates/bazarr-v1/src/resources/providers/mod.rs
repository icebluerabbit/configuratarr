//! Bazarr subtitle-provider config sections (`settings-<provider>-*`), one
//! struct per provider.
//!
//! Every field is `Option`/`Vec` (sparse: presence = manage the key). Provider
//! credentials are stored plaintext server-side (unlike `auth.password`), so the
//! generic reconcile diff makes them idempotent. Enable a provider by adding its
//! id to `general.enabled_providers`. Field names/types mirror bazarr's
//! `app/config.py` validators exactly.

pub mod addic7ed;
pub mod anidb;
pub mod animetosho;
pub mod anticaptcha;
pub mod assrt;
pub mod avistaz;
pub mod betaseries;
pub mod captchaai;
pub mod cinemaz;
pub mod deathbycaptcha;
pub mod embeddedsubtitles;
pub mod hdbits;
pub mod jimaku;
pub mod karagarga;
pub mod ktuvit;
pub mod legendasdivx;
pub mod legendasnet;
pub mod napiprojekt;
pub mod napisy24;
pub mod opensubtitlescom;
pub mod pipocas;
pub mod subdl;
pub mod subf2m;
pub mod subsarr;
pub mod subsource;
pub mod subsro;
pub mod subx;
pub mod titlovi;
pub mod titulky;
pub mod turkcealtyaziorg;
pub mod whisperai;
pub mod xsubs;
