use std::convert::TryFrom;

use serde::Deserialize;

use crate::{Error, Result};

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

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    None,
    Bool(bool),
    String(String),
    Double(f64),
    Number(i64),
}

impl Value {
    pub(crate) fn none() -> Value {
        Value::None
    }
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

impl<'a> From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl<'a> From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Double(value)
    }
}

impl<'a> From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Number(value)
    }
}

impl TryFrom<Value> for String {
    type Error = Error;

    fn try_from(value: Value) -> Result<String> {
        match value {
            Value::String(value) => Ok(value),
            _ => Err(format!("expected string, but got: {:?}", value).into()),
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = Error;

    fn try_from(value: Value) -> Result<i64> {
        match value {
            Value::Number(value) => Ok(value),
            _ => Err(format!("expected number, but got: {:?}", value).into()),
        }
    }
}

// impl From<EventResponse> for PropertyEvent {
//     fn from(event: EventResponse) -> PropertyEvent {
//         debug_assert_eq!(event.event, Event::PropertyChange);
//         PropertyEvent {
//             id: event.id.unwrap_or(0),
//             name: event.name,
//             data: event.data,
//         }
//     }
// }
