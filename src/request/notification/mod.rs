//! The `aps` notification content builders

mod localized;
mod plain;
mod silent;
mod options;

pub use self::localized::{LocalizedAlert, LocalizedNotificationBuilder};
pub use self::plain::PlainNotificationBuilder;
pub use self::silent::SilentNotificationBuilder;
pub use self::options::{CollapseId, NotificationOptions, Priority};

use request::payload::Payload;

pub trait NotificationBuilder {
    /// Generates the request payload to be send with the `Client`.
    fn build<'a>(self, device_token: &'a str, options: NotificationOptions) -> Payload<'a>;
}
