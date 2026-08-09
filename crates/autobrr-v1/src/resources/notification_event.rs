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

#[cfg(test)]
mod tests {
    use super::*;
    use core_lib::engine;
    use serde_json::json;

    /// Every wire value decodes to a real variant and re-encodes to the same
    /// string — including `RELEASE_NEW` / `TEST`, which previously fell through
    /// to `Unknown` and could not round-trip.
    #[test]
    fn all_events_round_trip() {
        for s in [
            "PUSH_APPROVED",
            "PUSH_REJECTED",
            "PUSH_ERROR",
            "IRC_DISCONNECTED",
            "IRC_RECONNECTED",
            "APP_UPDATE_AVAILABLE",
            "RELEASE_NEW",
            "TEST",
        ] {
            let ev: NotificationEvent = engine::decode(&json!(s)).unwrap();
            assert!(
                !matches!(ev, NotificationEvent::Unknown),
                "`{s}` must map to a real variant, not the fallback"
            );
            assert_eq!(engine::encode(&ev).unwrap(), json!(s), "round-trip `{s}`");
        }
    }

    /// A genuinely unknown value still lands on the fallback.
    #[test]
    fn unknown_event_is_fallback() {
        let ev: NotificationEvent = engine::decode(&json!("SOMETHING_FUTURE")).unwrap();
        assert!(matches!(ev, NotificationEvent::Unknown));
    }
}
