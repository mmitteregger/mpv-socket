use serde::{Deserialize, Serialize};

use crate::event::Event;
use crate::{Property, Value};

#[derive(Serialize)]
pub(crate) struct Request {
    pub command: Command,
    pub request_id: i64,
}

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct CommandResponse {
    pub request_id: Option<i64>,
    pub error: Option<String>,
    #[serde(default = "Value::none")]
    pub data: Value,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct EventResponse {
    #[serde(flatten)]
    pub event: Event,
    pub id: Option<i64>,
    pub error: Option<String>,
}

// impl<'a> From<&'a Event> for Value {
//     fn from(event: &'a Event) -> Self {
//         let value = match event {
//             Event::PropertyChange(..) => "property-change",
//             Event::Seek => "seek",
//         };
//         Value::from(value)
//     }
// }

pub(crate) enum Command {
    ClientName,
    GetTimeUs,
    GetProperty(Property),
    SetProperty(Property, Value),
    ObserveProperty(i64, Property),
    UnobserveProperty(i64),
    RequestLogMessages,
    // EnableEvent(EventType),
    // DisableEvent(EventType),
    GetVersion,
}

impl Command {
    pub fn name(&self) -> &'static str {
        match self {
            Command::ClientName => "client_name",
            Command::GetTimeUs => "get_time_us",
            Command::GetProperty(..) => "get_property",
            Command::SetProperty(..) => "set_property",
            Command::ObserveProperty(..) => "observe_property",
            Command::UnobserveProperty(..) => "unobserve_property",
            Command::RequestLogMessages => "request_log_messages",
            // Command::EnableEvent(..) => "enable_event",
            // Command::DisableEvent(..) => "disable_event",
            Command::GetVersion => "get_version",
        }
    }

    pub fn params(&self) -> Vec<Value> {
        match self {
            Command::ClientName => vec![],
            Command::GetTimeUs => vec![],
            Command::GetProperty(property) => vec![property.into()],
            Command::SetProperty(property, value) => vec![property.into(), value.clone()],
            Command::ObserveProperty(id, property) => vec![(*id).into(), property.into()],
            Command::UnobserveProperty(id) => vec![(*id).into()],
            Command::RequestLogMessages => vec![],
            // Command::EnableEvent(event) => vec![event.into()],
            // Command::DisableEvent(event) => vec![event.into()],
            Command::GetVersion => vec![],
        }
    }
}
