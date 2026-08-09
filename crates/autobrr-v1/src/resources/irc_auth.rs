use core_lib::SecretValue;
use core_macros::nested;

/// NickServ / SASL authentication for an [`IrcNetwork`](crate::resources::irc_network::IrcNetwork).
#[nested(case = snake)]
pub struct IrcAuth {
    /// Auth mechanism: `NONE`, `SASL_PLAIN`, or `NICKSERV`.
    pub mechanism: Option<String>,
    /// Account / login name.
    pub account: Option<String>,
    /// Account password (write-only; returned `<redacted>` on read).
    pub password: Option<SecretValue>,
}
