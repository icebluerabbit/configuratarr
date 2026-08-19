//! Collection resource — a Komga library (a scanned root directory).
//!
//! Modelled from `LibraryDto` (read) / `LibraryCreationDto` (create) /
//! `LibraryUpdateDto` (update) in the spec. The three DTOs share one field
//! set with two deliberate differences, both handled here:
//! - `unavailable` is server-computed — present on read, absent from both
//!   write DTOs — so it's `#[wire(read_only)]`.
//! - Every other setting is optional on write (`LibraryUpdateDto` marks
//!   nothing required, and the "required" booleans on `LibraryCreationDto`
//!   are Kotlin default-parameter artifacts, not user-mandatory input). The
//!   spec gives no default values for them, so rather than guess and risk
//!   forcing the wrong value onto every apply (a non-`Option` field is
//!   always sent, clobbering the live value via `merge` even when the user
//!   never mentioned it — see `core-architecture`), they're modelled as
//!   `Option<T>`: omitted in config = left alone on the server.

use core_macros::resource;

use crate::resources::scan_interval::ScanInterval;
use crate::resources::series_cover::SeriesCover;

/// A Komga library: a scanned root directory plus its import/scan behavior.
#[resource(
    sync = crud,
    list = get("/api/v1/libraries"),
    create = post("/api/v1/libraries"),
    update = patch("/api/v1/libraries/${self.id}"),
    delete = delete("/api/v1/libraries/${self.id}"),
)]
pub struct Library {
    /// Server-assigned id (Komga ids are opaque strings, not integers).
    #[id]
    pub id: Option<String>,
    /// Library name — its identity (`${ref.library.<name>}`).
    #[key]
    pub name: String,
    /// Filesystem path Komga scans for this library. Always required —
    /// there is no meaningful default for where to look.
    pub root: String,
    /// Directory name (relative to `root`) treated as containing oneshots.
    pub oneshots_directory: Option<String>,
    /// Whether Komga can currently reach `root`. Server-computed; absent
    /// from create/update.
    #[wire(read_only)]
    pub unavailable: Option<bool>,

    /// Scan this library on Komga startup.
    pub scan_on_startup: Option<bool>,
    /// Automatic rescan interval.
    pub scan_interval: Option<ScanInterval>,
    /// Scan `.cbr`/`.cbz` comic archives.
    pub scan_cbx: Option<bool>,
    /// Scan `.pdf` files.
    pub scan_pdf: Option<bool>,
    /// Scan `.epub` files.
    pub scan_epub: Option<bool>,
    /// Force re-checking file modification times during a scan, instead of
    /// trusting the last recorded value.
    pub scan_force_modified_time: Option<bool>,
    /// Directory names to exclude from scanning. Omit the key to leave the
    /// server's current list alone.
    pub scan_directory_exclusions: Option<Vec<String>>,
    /// Empty the trash (remove bookkeeping for files no longer on disk)
    /// after each scan.
    pub empty_trash_after_scan: Option<bool>,

    /// Read `ComicInfo.xml` metadata for books.
    pub import_comic_info_book: Option<bool>,
    /// Read `ComicInfo.xml` metadata for series.
    pub import_comic_info_series: Option<bool>,
    /// Read `ComicInfo.xml` metadata for collections.
    pub import_comic_info_collection: Option<bool>,
    /// Read `ComicInfo.xml` metadata for read lists.
    pub import_comic_info_read_list: Option<bool>,
    /// Append the volume number from `ComicInfo.xml` to the series title.
    pub import_comic_info_series_append_volume: Option<bool>,
    /// Read epub metadata for books.
    pub import_epub_book: Option<bool>,
    /// Read epub metadata for series.
    pub import_epub_series: Option<bool>,
    /// Import Mylar-style `series.json` metadata.
    pub import_mylar_series: Option<bool>,
    /// Import local artwork files (e.g. `cover.jpg`) found alongside books.
    pub import_local_artwork: Option<bool>,
    /// Read an ISBN from a barcode found in a book's pages.
    pub import_barcode_isbn: Option<bool>,

    /// Repair file extensions that don't match their actual content type.
    pub repair_extensions: Option<bool>,
    /// Automatically convert applicable archives to `.cbz`.
    pub convert_to_cbz: Option<bool>,
    /// Which book's cover to use as the series cover.
    pub series_cover: Option<SeriesCover>,

    /// Compute a hash of each file for duplicate detection.
    pub hash_files: Option<bool>,
    /// Compute a hash of each page within a file.
    pub hash_pages: Option<bool>,
    /// Compute a KOReader-compatible hash for each file.
    pub hash_koreader: Option<bool>,
    /// Analyze page dimensions (width/height) during a scan.
    pub analyze_dimensions: Option<bool>,
}
