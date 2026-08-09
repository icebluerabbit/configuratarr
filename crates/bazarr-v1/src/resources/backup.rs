use core_macros::nested;

/// Automatic-backup settings (`settings-backup-*`).
#[nested(case = snake)]
pub struct Backup {
    /// Directory backups are written to.
    pub folder: Option<String>,
    /// How many days of backups to retain.
    pub retention: Option<i32>,
    /// Backup cadence (`Manually` / `Daily` / `Weekly`).
    pub frequency: Option<String>,
    /// Day of week for a weekly backup (0–6).
    pub day: Option<i32>,
    /// Hour of day for the backup (0–23).
    pub hour: Option<i32>,
}
