// https://mpv.io/manual/master/#json-ipc

use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read, Write};
use std::num::Wrapping;
use std::path::Path;

pub use error::*;
pub use property::*;
use protocol::{Command, CommandResponse, Request};

use crate::event::{Event, PropertyChangeEvent, Reason};
use crate::protocol::EventResponse;

mod error;
pub mod event;
mod property;
pub(crate) mod protocol;
mod serde_impl;

trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}

#[derive(Copy, Clone, Eq, PartialEq)]
struct RequestId(Wrapping<i64>);

impl RequestId {
    fn new() -> RequestId {
        RequestId(Wrapping(0))
    }

    fn next(&mut self) -> i64 {
        self.advance(1)
    }

    fn advance(&mut self, num: i64) -> i64 {
        self.0 = self.0 + Wrapping(num);
        (self.0).0
    }
}

pub struct MpvSocket {
    socket: BufReader<Box<dyn ReadWrite>>,
    read_buf: Vec<u8>,
    last_request_id: RequestId,
    closed: bool,
}

#[cfg(target_os = "windows")]
impl MpvSocket {
    pub fn connect<P: AsRef<Path>>(path: P) -> Result<MpvSocket> {
        log::info!("connecting to: {}", path.as_ref().display());
        let mut tries_left = 5u8;

        let socket = loop {
            let open_pipe_result = OpenOptions::new()
                .read(true)
                .write(true)
                .open(path.as_ref());

            let error = match open_pipe_result {
                Ok(socket) => {
                    break socket;
                }
                Err(error) => match error.raw_os_error() {
                    Some(code) => match code {
                        ERROR_PIPE_BUSY => {
                            // On Windows the socket/pipe can only be opened
                            // by one application and thread at the same time
                            // and it can happen spuriously when closing/opening the connections
                            // very often very fast, so try to guard against that.
                            tries_left -= 1;
                            if tries_left != 0 {
                                std::thread::sleep(std::time::Duration::from_millis(10));
                                continue;
                            }

                            error
                        }
                        _ => error,
                    },
                    None => error,
                },
            };

            return Err(format!("failed to open mpv socket: {}", error).into());
        };

        Ok(MpvSocket {
            socket: BufReader::new(Box::new(socket)),
            read_buf: Vec::with_capacity(128),
            last_request_id: RequestId::new(),
            closed: false,
        })
    }
}

impl MpvSocket {
    /// Return the name of the client as string.
    ///
    /// This is the string "ipc-N" with N being an integer number.
    pub fn client_name(&mut self) -> Result<String> {
        self.send_recv_convert_command(Command::ClientName)
    }

    /// Return the current mpv internal time in microseconds as a number.
    ///
    /// This is basically the system time, with an arbitrary offset.
    pub fn get_time_us(&mut self) -> Result<i64> {
        self.send_recv_convert_command(Command::GetTimeUs)
    }

    /// Return the value of the given property.
    ///
    /// See [`Properties`] for more information about properties.
    ///
    /// [`Properties`]: https://mpv.io/manual/master/#properties
    pub fn get_property(&mut self, property: Property) -> Result<Value> {
        self.send_recv_command(Command::GetProperty(property))
    }

    /// Set the given property to the given value.
    ///
    /// See [`Properties`] for more information about properties.
    ///
    /// [`Properties`]: https://mpv.io/manual/master/#properties
    pub fn set_property(&mut self, property: Property, value: impl Into<Value>) -> Result<Value> {
        self.send_recv_command(Command::SetProperty(property, value.into()))
    }

    /// Watch a property for changes.
    ///
    /// If the given property is changed,
    /// then the iterator will return the next [`Value`].
    ///
    /// See [`Properties`] for more information about properties.
    ///
    /// [`Properties`]: https://mpv.io/manual/master/#properties
    /// [`Value`]: ./struct.Value.html
    pub fn observe_property<'a>(
        &'a mut self,
        property: Property,
    ) -> Result<impl Iterator<Item = Result<Value>> + 'a> {
        self.send_recv_command(Command::ObserveProperty(1, property))?;

        Ok(
            EventIter::new(self, 1).filter_map(|event_response| match event_response {
                Ok(event_response) => match event_response.event {
                    Event::PropertyChange(property_change_event) => {
                        Some(Ok(property_change_event.data))
                    }
                    _ => {
                        log::debug!("filtered event: {:?}", event_response);
                        None
                    }
                },
                Err(error) => Some(Err(error)),
            }),
        )
    }

    /// Watch properties for changes.
    ///
    /// If one of the given properties is changed,
    /// then the iterator will return the next [`Event`].
    ///
    /// See [`Properties`] for more information about properties.
    ///
    /// [`Properties`]: https://mpv.io/manual/master/#properties
    /// [`Event`]: ./struct.Event.html
    pub fn observe_properties<'a>(
        &'a mut self,
        properties: impl IntoIterator<Item = Property>,
    ) -> Result<impl Iterator<Item = Result<PropertyChangeEvent>> + 'a> {
        let mut property_index = 0;
        for property in properties {
            property_index += 1;
            self.send_recv_command(Command::ObserveProperty(property_index, property))?;
        }

        Ok(
            EventIter::new(self, property_index).filter_map(
                |event_response| match event_response {
                    Ok(event_response) => match event_response.event {
                        Event::PropertyChange(property_change_event) => {
                            Some(Ok(property_change_event))
                        }
                        _ => None,
                    },
                    Err(error) => Some(Err(error)),
                },
            ),
        )
    }

    fn send_recv_convert_command<T>(&mut self, command: Command) -> Result<T>
    where
        T: TryFromValue,
    {
        T::try_from(self.send_recv_command(command)?)
    }

    fn send_recv_command(&mut self, command: Command) -> Result<Value> {
        if self.closed {
            return Err("mpv socket is closed".into());
        }

        let request = Request {
            command,
            request_id: self.last_request_id.next(),
        };
        let req_json = serde_json::to_vec(&request)?;
        if log::log_enabled!(log::Level::Trace) {
            log::trace!("sending: {}", String::from_utf8_lossy(&req_json));
        }

        let writer = self.socket.get_mut();
        writer.write_all(&req_json)?;
        writer.write_all(b"\n")?;
        writer.flush()?;

        loop {
            self.read_buf.clear();
            let num_bytes = self.socket.read_until(b'\n', &mut self.read_buf)?;
            let res_json = String::from_utf8_lossy(&self.read_buf[..num_bytes]);
            if log::log_enabled!(log::Level::Trace) {
                log::trace!("received: {}", res_json.trim());
            }

            let response: CommandResponse = serde_json::from_str(res_json.as_ref())?;

            if response.request_id == Some(request.request_id) {
                return match response.error.as_ref().map(|error| error.as_str()) {
                    Some("success") => Ok(response.data),
                    Some(error) => Err(format!("mpv error response: {}", error).into()),
                    None => Err(format!("unknown mpv response: {:?}", response).into()),
                };
            }
        }
    }
}

struct EventIter<'a> {
    mpv: &'a mut MpvSocket,
    num_observed_properties: i64,
}

impl<'a> EventIter<'a> {
    fn new(mpv: &'a mut MpvSocket, num_observed_properties: i64) -> EventIter {
        EventIter {
            mpv,
            num_observed_properties,
        }
    }
}

impl<'a> Iterator for EventIter<'a> {
    type Item = Result<EventResponse>;

    fn next(&mut self) -> Option<Result<EventResponse>> {
        if self.mpv.closed {
            return None;
        }

        self.mpv.read_buf.clear();
        let num_bytes = match self.mpv.socket.read_until(b'\n', &mut self.mpv.read_buf) {
            Ok(num_bytes) => num_bytes,
            Err(error) => return Some(Err(error.into())),
        };
        if num_bytes == 0 {
            return None;
        }

        let res_json = String::from_utf8_lossy(&self.mpv.read_buf[..num_bytes]);
        if log::log_enabled!(log::Level::Trace) {
            log::trace!("received: {}", res_json.trim());
        }

        let res_event: EventResponse = match serde_json::from_str(res_json.as_ref()) {
            Ok(event) => event,
            Err(error) => {
                return Some(Err(error.into()));
            }
        };

        match &res_event.event {
            Event::Shutdown => {
                self.mpv.closed = true;
            }
            Event::EndFile(end_file_event) => {
                if end_file_event.reason == Reason::Quit {
                    self.mpv.closed = true;
                }
            }
            _ => {}
        }

        let next = match res_event.error.as_ref().map(|error| error.as_str()) {
            Some("success") | None => res_event,
            Some(error) => return Some(Err(format!("mpv error response: {}", error).into())),
        };

        return Some(Ok(next));
    }
}

impl<'a> Drop for EventIter<'a> {
    fn drop(&mut self) {
        if self.mpv.closed {
            return;
        }

        let result = {
            let mut result = Ok(());
            for i in 1..=self.num_observed_properties {
                match self.mpv.send_recv_command(Command::UnobserveProperty(i)) {
                    Ok(_value) => {}
                    Err(error) => {
                        result = Err(error);
                        break;
                    }
                }
            }
            result
        };
        match result {
            Ok(json) => json,
            Err(error) => {
                if let Some(io_error) = error.downcast_ref::<std::io::Error>() {
                    if io_error.raw_os_error() == Some(ERROR_NO_DATA) {
                        // Ignore this error,
                        // a closed media player is not a problem
                        // and will leave no trace of stale or wrong state.
                        return;
                    }
                }
                log::error!("error while dropping iterator: {}", error);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    fn init() -> MpvSocket {
        let _ = pretty_env_logger::try_init_timed();
        MpvSocket::connect(r"\\.\pipe\mpv-socket").unwrap()
    }

    #[test]
    fn client_name() {
        let mut mpv_socket = init();
        let client_name = mpv_socket.client_name().unwrap();
        log::info!("Client name: {}", client_name);
        assert!(!client_name.is_empty());
    }

    #[test]
    fn time_us() {
        let mut mpv_socket = init();
        let time_us = mpv_socket.get_time_us().unwrap();
        log::info!("Time microseconds: {}", time_us);
        assert_ne!(time_us, 0);
    }

    #[test]
    fn get_property_volume() {
        let mut mpv_socket = init();
        let volume = mpv_socket.get_property(Property::Volume).unwrap();
        log::info!("Volume: {:?}", volume);
        assert!(matches!(volume, Value::Number(..)));
    }

    #[test]
    fn set_property_pause() {
        let mut mpv_socket = init();
        mpv_socket.set_property(Property::Pause, false).unwrap();
        log::info!("Unpaused playback");
    }

    #[test]
    fn observe_property_playback_time() {
        let mut mpv_socket = init();
        let playback_time_iter = mpv_socket.observe_property(Property::PlaybackTime).unwrap();

        for playback_time in playback_time_iter.take(25) {
            log::info!("Playback time: {:?}", playback_time);
        }
    }

    #[test]
    fn observe_property_then_observe_other_property() {
        let mut mpv_socket = init();
        let iter = mpv_socket.observe_property(Property::PlaybackTime).unwrap();

        for playback_time in iter.take(25).map(|result| result.unwrap()) {
            if let Value::Number(playback_time) = playback_time {
                log::info!("Playback time: {:?}", playback_time);
            }
        }

        let iter = mpv_socket.observe_property(Property::StreamPos).unwrap();

        for stream_pos in iter.take(25).map(|result| result.unwrap()) {
            match stream_pos {
                Value::Number(stream_pos) => log::info!("Stream pos: {}", stream_pos),
                Value::Null => {}
                value => panic!(
                    "old or otherwise invalid property value returned: {:?}",
                    value
                ),
            }
        }
    }

    #[test]
    fn observe_properties() {
        let mut mpv_socket = init();
        let iter = mpv_socket
            .observe_properties(
                [Property::PlaybackTime, Property::TimeRemaining]
                    .iter()
                    .copied(),
            )
            .unwrap();

        for property in iter.take(25).map(|result| result.unwrap()) {
            log::info!("Property: {:?}", property);
        }
    }
}
