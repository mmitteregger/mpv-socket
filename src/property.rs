use serde::Deserialize;
pub use serde_json::Value;

use crate::Result;

#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Property {
    Volume,
    Pause,
    /// Returns yes if the player is currently seeking, or otherwise trying to restart playback.
    ///
    /// (It's possible that it returns yes while a file is loaded.
    /// This is because the same underlying code is used for seeking and resyncing.)
    Seeking,
    PlaybackTime,
    TimeRemaining,
    PercentPos,
    StreamPos,
}

impl<'a> From<&'a Property> for Value {
    fn from(property: &'a Property) -> Self {
        let value = match property {
            Property::Volume => "volume",
            Property::Pause => "pause",
            Property::Seeking => "seeking",
            Property::PlaybackTime => "playback-time",
            Property::TimeRemaining => "time-remaining",
            Property::PercentPos => "percent-pos",
            Property::StreamPos => "stream-pos",
        };
        Value::from(value)
    }
}

pub(crate) trait TryFromValue: Sized {
    fn try_from(value: Value) -> Result<Self>;
}

impl TryFromValue for String {
    fn try_from(value: Value) -> Result<String> {
        value
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| format!("expected string, but got: {:?}", value).into())
    }
}

impl TryFromValue for i64 {
    fn try_from(value: Value) -> Result<i64> {
        value
            .as_i64()
            .ok_or_else(|| format!("expected i64, but got: {:?}", value).into())
    }
}
