//! Mpv events.
//!
//! Official documentation: [https://mpv.io/manual/master/#list-of-events](https://mpv.io/manual/master/#list-of-events)

#![allow(deprecated)]

use serde::Deserialize;
use serde_json::Value;

use crate::Property;

/// Mpv event variants.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "event", rename_all = "kebab-case")]
pub enum Event {
    /// Happens after a property change for observed properties.
    PropertyChange(PropertyChangeEvent),
    /// Happens right before a new file is loaded.
    ///
    /// When you receive this, the player is loading the file (or possibly already done with it).
    StartFile(StartFileEvent),
    /// Happens after a file was unloaded. Typically, the player will load the next file right away, or quit if this was the last file.
    EndFile(EndFileEvent),
    /// Happens after a file was loaded and begins playback.
    FileLoaded,
    /// Happens on seeking.
    ///
    /// (This might include cases when the player seeks internally,
    /// even without user interaction.
    /// This includes e.g. segment changes when playing ordered chapters Matroska files.)
    Seek,
    /// Start of playback after seek or after file was loaded.
    PlaybackRestart,
    /// Sent when the player quits, and the script should terminate.
    ///
    /// Normally handled automatically.
    /// See [`Details on the script initialization and lifecycle`].
    ///
    /// [`Details on the script initialization and lifecycle`]: https://mpv.io/manual/master/#details-on-the-script-initialization-and-lifecycle
    Shutdown,
    /// Receives messages enabled with `mpv_request_log_messages()`
    ///
    /// (Lua: `mp.enable_messages`).
    LogMessage(LogMessageEvent),
    /// Hook.
    Hook(HookEvent),
    /// See C API.
    #[doc(hidden)]
    GetPropertyReply(Value),
    /// See C API.
    #[doc(hidden)]
    SetPropertyReply(Value),
    /// This is one of the commands for which the `error` field is meaningful.
    //
    // JSON IPC and Lua and possibly other backends treat this specially
    // and may not pass the actual event to the user.
    //
    // See C API.
    #[doc(hidden)]
    CommandReply(Value),
    /// Lua and possibly other backends treat this specially
    /// and may not pass the actual event to the user.
    #[doc(hidden)]
    ClientMessage(Value),
    /// Happens on video output or filter reconfig.
    VideoReconfig,
    /// Happens on audio output or filter reconfig.
    AudioReconfig,
    /// Deprecated: Use `observe_property` instead.
    #[deprecated]
    TracksChanged,
    /// Deprecated: Use `observe_property` instead.
    #[deprecated]
    TrackSwitched,
    /// Deprecated: Use `observe_property` instead.
    #[deprecated]
    Pause,
    /// Deprecated: Use `observe_property` instead.
    #[deprecated]
    Unpause,
    /// Deprecated: Use `observe_property` instead.
    #[deprecated]
    MetadataUpdate,
    /// Deprecated: Use `observe_property` instead.
    #[deprecated]
    Idle,
    /// Deprecated: Use `observe_property` instead.
    #[deprecated]
    Tick,
    /// Deprecated: Use `observe_property` instead.
    #[deprecated]
    ChapterChange,

    /// Non-exhaustive enums could have additional variants added in future.
    ///
    /// Therefore, when matching against variants of non-exhaustive enums,
    /// an extra wildcard arm must be added to account for any future variants.
    #[doc(hidden)]
    __NonExhaustive,

    /// Unknown event.
    ///
    /// Unknown events should not cause deserialization errors, so they are caught here.
    #[serde(other)]
    #[doc(hidden)]
    Other,
}

/// Payload for [`Event::PropertyChange`].
///
/// [`Event::PropertyChange`]: ./enum.Event.html#variant.PropertyChange
#[derive(Debug, Deserialize, PartialEq)]
pub struct PropertyChangeEvent {
    /// The property whose value was changed.
    pub name: Property,
    /// New property data.
    ///
    /// The type usually is the value type of the property,
    /// but may also be [`Value::Null`] when the player is currently shutting down.
    /// Therefore clients should always try to destructure the value instead of simply unwrapping.
    ///
    /// [`Value::Null`]: ../enum.Value.html
    #[serde(default)]
    pub data: Value,
}

/// Payload for [`Event::StartFile`].
///
/// [`Event::StartFile`]: ./enum.Event.html#variant.StartFile
#[derive(Debug, Deserialize, PartialEq)]
pub struct StartFileEvent {
    /// Playlist entry ID of the file being loaded now.
    pub playlist_entry_id: Option<i64>,
}

/// Payload for [`Event::EndFile`].
///
/// [`Event::EndFile`]: ./enum.Event.html#variant.EndFile
#[derive(Debug, Deserialize, PartialEq)]
pub struct EndFileEvent {
    /// Why the playback has ended.
    pub reason: Option<Reason>,
    /// Playlist entry ID of the file that was being played
    /// or attempted to be played.
    ///
    /// This has the same value as the playlist_entry_id field
    /// in the corresponding start-file event.
    pub playlist_entry_id: Option<i64>,
    /// Set to mpv error string describing the approximate reason why playback failed.
    ///
    /// Unset if no error known.
    ///
    /// (In Lua scripting, this value was set on the error field directly.
    /// This is deprecated since mpv 0.33.0.
    /// In the future, this error field will be unset for this specific event.)
    pub file_error: Option<String>,
    /// If loading ended, because the playlist entry to be played was for example a playlist,
    /// and the current playlist entry is replaced with a number of other entries.
    ///
    /// This may happen at least with `MPV_END_FILE_REASON_REDIRECT`
    /// (other event types may use this for similar but different purposes in the future).
    ///
    /// In this case, `playlist_insert_id` will be set
    /// to the playlist entry ID of the first inserted entry,
    /// and `playlist_insert_num_entries` to the total number of inserted playlist entries.
    ///
    /// Note this in this specific case,
    /// the ID of the last inserted entry is `playlist_insert_id+num-1`.
    ///
    /// Beware that depending on circumstances,
    /// you may observe the new playlist entries before seeing the event
    /// (e.g. reading the "playlist" property
    /// or getting a property change notification before receiving the event).
    ///
    /// If this is 0 in the C API, this field isn't added.
    pub playlist_insert_id: Option<i64>,
    /// See `playlist_insert_id`.
    ///
    /// Only present if playlist_insert_id is present.
    pub playlist_insert_num_entries: Option<i64>,
}

/// Reason for [`Event::EndFile`].
///
/// [`Event::EndFile`]: ./enum.Event.html#variant.EndFile
#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Reason {
    /// The file has ended.
    ///
    /// This can (but doesn't have to) include incomplete files
    /// or broken network connections under circumstances.
    Eof,
    /// Playback was ended by a command.
    Stop,
    /// Playback was ended by sending the quit command.
    Quit,
    /// An error happened.
    ///
    /// In this case, an error field is present with the error string.
    Error,
    /// Happens with playlists and similar.
    ///
    /// For details see `MPV_END_FILE_REASON_REDIRECT` in the C API.
    Redirect,
    /// Unknown.
    ///
    /// Normally doesn't happen, unless the Lua API is out of sync with the C API.
    ///
    /// (Likewise, it could happen that your script gets reason strings
    /// that did not exist yet at the time your script was written.)
    Unknown,
}

/// Payload for [`Event::LogMessage`].
///
/// [`Event::LogMessage`]: ./enum.Event.html#variant.LogMessage
#[derive(Debug, Deserialize, PartialEq)]
pub struct LogMessageEvent {
    /// The module prefix, identifies the sender of the message.
    ///
    /// This is what the terminal player puts in front of the message text
    /// when using the `--v` option, and is also what is used for `--msg-level`.
    prefix: String,
    /// The log level as string.
    ///
    /// See `msg.log` for possible log level names.
    /// Note that later versions of mpv might add new levels
    /// or remove (undocumented) existing ones.
    level: String,
    /// The log message.
    ///
    /// The text will end with a newline character.
    /// Sometimes it can contain multiple lines.
    ///
    /// Keep in mind that these messages are meant to be hints for humans.
    /// You should not parse them, and prefix/level/text of messages might change any time.
    text: String,
}

/// Payload for [`Event::Hook`].
///
/// [`Event::Hook`]: ./enum.Event.html#variant.Hook
#[derive(Debug, Deserialize, PartialEq)]
pub struct HookEvent {
    /// ID to pass to `mpv_hook_continue()`.
    ///
    /// The Lua scripting wrapper provides a better API around this with `mp.add_hook()`.
    hook_id: Value,
}
