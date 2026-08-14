use core_macros::wire_enum;

/// An event that triggers a notification.
#[wire_enum(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationEvent {
    /// A release was pushed to a download client / action.
    PushApproved,
    /// A release was rejected by filter rules.
    PushRejected,
    /// An error occurred while pushing a release.
    PushError,
    /// autobrr lost its IRC connection.
    IrcDisconnected,
    /// autobrr re-established its IRC connection.
    IrcReconnected,
    /// A new autobrr version is available.
    AppUpdateAvailable,
    /// A new release was announced (feeds/IRC).
    ReleaseNew,
    /// A test event, emitted by the notification "Test" button.
    Test,
    /// Unknown or future event.
    #[fallback]
    Unknown,
}
