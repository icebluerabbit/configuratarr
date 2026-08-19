//! Per-library settings blob (`Library.settings`), embedded in
//! [`crate::resources::library::Library`].
//!
//! The live API gates this field set by the owning library's `media_type`: a
//! **book** library's default template carries the book-only fields below
//! plus the shared ones; a **podcast** library's template carries
//! `podcast_search_region` plus the shared ones and none of the book-only
//! fields. Sending a key that doesn't belong to the current `media_type`'s
//! template is silently dropped by the server, not rejected — so this one
//! struct models the union of both templates rather than splitting into two
//! types, and getting the `media_type` wrong just means a book-only field
//! quietly does nothing on a podcast library (and vice versa).
//!
//! Every field is `Option<T>` and **none carry `#[default(...)]`**, on
//! purpose: `Library`'s reconcile hook sends only the settings keys present
//! in the user's config (`engine::config_present_to_wire`, which recurses
//! into this nested struct too), because `PATCH /api/libraries/{id}` merges
//! `settings` key-by-key over the library's current settings — an omitted
//! key must mean "leave the live value alone", not "reset to this struct's
//! default". A `#[default(...)]` literal would only ever apply to the
//! *unmasked* encode path (unused here), so it would be pure noise; server
//! defaults are documented in prose below instead.

use core_macros::nested;

/// Per-library scan/matching/display settings.
#[nested]
pub struct LibrarySettings {
    /// Cover thumbnail aspect ratio: `0` = standard (2:3), `1` = square.
    /// Shared by both media types. Server default: `1`.
    pub cover_aspect_ratio: Option<i32>,
    /// Disable the filesystem watcher for this library. Shared. Toggling
    /// this restarts the watcher. Server default: `false`.
    pub disable_watcher: Option<bool>,
    /// Standard 6-field (seconds-first) cron expression for the scheduled
    /// scan (e.g. `0 0 0 * * *`); absent disables the scheduled scan. Shared.
    /// Changing this on update reprograms the scan cron job. **Known
    /// limitation:** the live API accepts an explicit `null` on update to
    /// *clear* an existing schedule, but this crate's config codec treats an
    /// explicit YAML `null` identically to omitting the key — so this
    /// resource can declare "leave unset" or "set a value" but has no way to
    /// declare "explicitly clear a previously-set schedule"; clearing it
    /// today requires an out-of-band call.
    pub auto_scan_cron_expression: Option<String>,
    /// Book libraries only. Skip quick-match for books that already have an
    /// ASIN. Server default: `false`.
    pub skip_matching_media_with_asin: Option<bool>,
    /// Book libraries only. Skip quick-match for books that already have an
    /// ISBN. Server default: `false`.
    pub skip_matching_media_with_isbn: Option<bool>,
    /// Book libraries only. Ignore ebook files entirely except as
    /// supplementary files to an audiobook. Server default: `false`.
    pub audiobooks_only: Option<bool>,
    /// Book libraries only. Allow scripted (JS) content inside served
    /// epubs. Server default: `false`.
    pub epubs_allow_scripted_content: Option<bool>,
    /// Book libraries only. Hide series that contain only one book from
    /// series views. Server default: `false`.
    pub hide_single_book_series: Option<bool>,
    /// Book libraries only. "Continue series" shelves skip books at or
    /// before the highest sequence number already read. Server default:
    /// `false`.
    pub only_show_later_books_in_continue_series: Option<bool>,
    /// Book libraries only. Ordered precedence of metadata sources used
    /// when scanning/matching (e.g. `folderStructure`, `audioMetatags`,
    /// `nfoFile`, `txtFiles`, `opfFile`, `absMetadata`). Sent value replaces
    /// the array wholesale — the server does not merge it element-wise.
    pub metadata_precedence: Vec<String>,
    /// Podcast libraries only. Region code used when searching for
    /// podcasts to add. Server default: `"us"`.
    pub podcast_search_region: Option<String>,
    /// Shared. Percent (0-100) of playback progress at which an item is
    /// auto-marked finished; when set, takes precedence over
    /// `mark_as_finished_time_remaining`. Server default: unset (disabled).
    pub mark_as_finished_percent_complete: Option<f64>,
    /// Shared. Seconds of playback remaining at which an item is
    /// auto-marked finished. Ignored when `mark_as_finished_percent_complete`
    /// is set. Server default: `10`.
    pub mark_as_finished_time_remaining: Option<f64>,
}
