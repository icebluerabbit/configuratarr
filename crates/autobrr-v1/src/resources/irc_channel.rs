use core_lib::SecretValue;
use core_macros::nested;

/// One channel an [`IrcNetwork`](crate::resources::irc_network::IrcNetwork) joins.
/// Runtime state autobrr attaches on read (`id`, `monitoring`, …) is not
/// modelled — only the declared identity, key and enablement.
///
/// `detached` is deliberately *not* modelled: autobrr hardcodes it to `true`
/// when it (re)inserts a network's channel rows, so a declared value could never
/// converge — every apply would diff and re-write forever.
#[nested(case = snake)]
pub struct IrcChannel {
    /// Channel name (e.g. `#announce`).
    pub name: String,
    /// Channel key/password, where the channel is protected (write-only).
    pub password: Option<SecretValue>,
    /// Whether autobrr joins this channel. Must always be written: autobrr's
    /// channel struct decodes an absent `enabled` key as `false`, and its join
    /// workflow skips disabled channels — so omitting it silently stops
    /// announces. Non-optional with a `true` default for that reason.
    #[default(true)]
    pub enabled: bool,
}
