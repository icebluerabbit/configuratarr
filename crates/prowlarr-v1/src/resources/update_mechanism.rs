use core_macros::wire_enum;

/// How Prowlarr installs updates when a new version is available.
#[wire_enum(rename_all = "camelCase")]
pub enum UpdateMechanism {
    /// Prowlarr uses its own built-in updater.
    BuiltIn,
    /// Updates are applied by an external script.
    Script,
    /// Updates are managed by an external process (e.g. package manager).
    External,
    /// Updates are applied via the system APT package manager.
    Apt,
    /// Running inside Docker; updates are handled by pulling a new image.
    Docker,
    /// Unknown or future update mechanism.
    #[fallback]
    Unknown,
}
