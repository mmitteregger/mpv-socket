use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};

use crate::Command;

impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let name = self.name();
        let params = self.params();

        let mut seq = serializer.serialize_seq(Some(1 + params.len()))?;
        seq.serialize_element(name)?;
        for param in params {
            seq.serialize_element(&param)?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{Event, PropertyChangeEvent};
    use crate::protocol::EventResponse;
    use crate::{CommandResponse, Property, Request, Value};

    #[test]
    fn serialize_request_client_name() {
        let request = Request {
            command: Command::ClientName,
            request_id: 1,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, r#"{"command":["client_name"],"request_id":1}"#);
    }

    #[test]
    fn serialize_request_get_property_volume() {
        let request = Request {
            command: Command::GetProperty(Property::Volume),
            request_id: 1,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(
            json,
            r#"{"command":["get_property","volume"],"request_id":1}"#
        );
    }

    #[test]
    fn serialize_request_set_property_pause() {
        let request = Request {
            command: Command::SetProperty(Property::Pause, Value::from(true)),
            request_id: 1,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(
            json,
            r#"{"command":["set_property","pause",true],"request_id":1}"#
        );
    }

    #[test]
    fn serialize_request_observe_property_volume() {
        let request = Request {
            command: Command::ObserveProperty(1, Property::Volume),
            request_id: 1,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(
            json,
            r#"{"command":["observe_property",1,"volume"],"request_id":1}"#
        );
    }

    #[test]
    fn deserialize_response() {
        let input = r#"{ "error": "success" }"#;
        let response: CommandResponse = serde_json::from_str(input).unwrap();
        assert_eq!(
            response,
            CommandResponse {
                request_id: None,
                error: Some(String::from("success")),
                data: Value::Null,
            }
        )
    }

    #[test]
    fn deserialize_response_with_request_id() {
        let input = r#"{ "request_id": 123, "error": "success" }"#;
        let response: CommandResponse = serde_json::from_str(input).unwrap();
        assert_eq!(
            response,
            CommandResponse {
                request_id: Some(123),
                error: Some(String::from("success")),
                data: Value::Null,
            }
        )
    }

    #[test]
    fn deserialize_response_with_data_double() {
        let input = r#"{ "data": 50.0, "error": "success" }"#;
        let response: CommandResponse = serde_json::from_str(input).unwrap();
        assert_eq!(
            response,
            CommandResponse {
                request_id: None,
                error: Some(String::from("success")),
                data: Value::from(50.0),
            }
        )
    }

    #[test]
    fn deserialize_response_with_data_bool() {
        let input = r#"{ "data": true, "error": "success" }"#;
        let response: CommandResponse = serde_json::from_str(input).unwrap();
        assert_eq!(
            response,
            CommandResponse {
                request_id: None,
                error: Some(String::from("success")),
                data: Value::Bool(true),
            }
        )
    }

    #[test]
    fn deserialize_response_with_data_str() {
        let input = r#"{ "data": "test", "error": "success" }"#;
        let response: CommandResponse = serde_json::from_str(input).unwrap();
        assert_eq!(
            response,
            CommandResponse {
                request_id: None,
                error: Some(String::from("success")),
                data: Value::String(String::from("test")),
            }
        )
    }

    #[test]
    fn deserialize_response_from_event() {
        let input = r#"{ "event": "property-change", "id": 1, "data": 52.0, "name": "volume" }"#;
        let response: CommandResponse = serde_json::from_str(input).unwrap();
        assert_eq!(
            response,
            CommandResponse {
                request_id: None,
                error: None,
                data: Value::from(52.0),
            }
        )
    }

    #[test]
    fn deserialize_event() {
        let input = r#"{ "event": "property-change", "id": 1, "data": 52.0, "name": "volume" }"#;
        let response: EventResponse = serde_json::from_str(input).unwrap();
        assert_eq!(
            response,
            EventResponse {
                event: Event::PropertyChange(PropertyChangeEvent {
                    data: Value::from(52.0),
                    name: Property::Volume
                }),
                id: Some(1),
                error: None,
            }
        )
    }
}
