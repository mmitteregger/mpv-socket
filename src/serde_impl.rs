use std::fmt;

use serde::de::{self, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Command, Request, Value};

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

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::None => serializer.serialize_none(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::String(s) => serializer.serialize_str(s),
            Value::Double(d) => serializer.serialize_f64(*d),
            Value::Number(n) => serializer.serialize_i64(*n),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer between -2^31 and 2^31")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Bool(value))
            }

            fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(i64::from(value)))
            }

            fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(i64::from(value)))
            }

            fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(i64::from(value)))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(value))
            }

            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(i64::from(value)))
            }

            fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(i64::from(value)))
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(i64::from(value)))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value <= i64::MAX as u64 {
                    Ok(Value::Number(value as i64))
                } else {
                    Err(E::custom(format!("u64 out of range: {}", value)))
                }
            }

            fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Double(f64::from(value)))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Double(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::String(value.to_string()))
            }

            fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::String(value.to_string()))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::String(value))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{Event, PropertyChangeEvent};
    use crate::protocol::EventResponse;
    use crate::{CommandResponse, Property};

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
                data: Value::None,
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
                data: Value::None,
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
                data: Value::Double(50.0),
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
                data: Value::Double(52.0),
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
                    data: Value::Double(52.0),
                    name: Property::Volume
                }),
                id: Some(1),
                error: None,
            }
        )
    }
}
