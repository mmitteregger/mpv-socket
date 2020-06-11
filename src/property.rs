#![allow(deprecated)]

use std::fmt;

use serde::Deserialize;
pub use serde_json::{Map, Value};

use crate::Result;

/// Properties are used to set mpv options during runtime,
/// or to query arbitrary information.
///
/// The property documentation is annotated with **RW**
/// to indicate whether the property is generally writable.
///
/// Official documentation: [https://mpv.io/manual/master/#properties](https://mpv.io/manual/master/#properties)
#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Property {
    /// Factor multiplied with speed at which the player attempts to play the file.
    ///
    /// Usually it's exactly 1.
    /// (Display sync mode will make this useful.)
    ///
    /// OSD formatting will display it in the form of +1.23456%,
    /// with the number being (raw - 1) * 100 for the given raw property value.
    AudioSpeedCorrection,
    /// Factor multiplied with speed at which the player attempts to play the file.
    ///
    /// Usually it's exactly 1.
    /// (Display sync mode will make this useful.)
    ///
    /// OSD formatting will display it in the form of +1.23456%,
    /// with the number being (raw - 1) * 100 for the given raw property value.
    VideoSpeedCorrection,
    /// Return whether --video-sync=display is actually active.
    DisplaySyncActive,
    /// Currently played file, with path stripped.
    ///
    /// If this is an URL, try to undo percent encoding as well.
    /// (The result is not necessarily correct, but looks better for display purposes.
    /// Use the path property to get an unmodified filename.)
    Filename,
    /// Like the filename property, but if the text contains a ., strip all text after the last ..
    ///
    /// Usually this removes the file extension.
    FilenameNoExt,
    /// Length in bytes of the source file/stream.
    ///
    /// (This is the same as ${stream-end}.
    /// For segmented/multi-part files, this will return the size of the main or manifest file,
    /// whatever it is.)
    FileSize,
    /// Total number of frames in current file.
    ///
    /// # Note
    ///
    /// This is only an estimate.
    /// (It's computed from two unreliable quantities: fps and stream length.)
    EstimatedFrameCount,
    /// Number of current frame in current stream.
    ///
    /// # Note
    ///
    /// This is only an estimate.
    /// (It's computed from two unreliable quantities: fps and possibly rounded timestamps.)
    EstimatedFrameNumber,
    /// Full path of the currently played file.
    ///
    /// Usually this is exactly the same string you pass on the mpv command line
    /// or the `loadfile` command, even if it's a relative path.
    ///
    /// If you expect an absolute path, you will have to determine it yourself,
    /// for example by using the `working-directory` property.
    Path,
    /// The full path to the currently played media.
    ///
    /// This is different only from path in special cases.
    /// In particular, if `--ytdl=yes` is used, and the URL is detected by `youtube-dl`,
    /// then the script will set this property to the actual media URL.
    /// This property should be set only during the `on_load` or `on_load_fail` hooks,
    /// otherwise it will have no effect (or may do something implementation defined in the future).
    /// The property is reset if playback of the current media ends.
    StreamOpenFilename,
    /// If the currently played file has a `title` tag, use that.
    ///
    /// Otherwise, return the `filename` property.
    MediaTitle,
    /// Symbolic name of the file format.
    ///
    /// In some cases, this is a comma-separated list of format names,
    /// e.g. mp4 is `mov,mp4,m4a,3gp,3g2,mj2` (the list may grow in the future for any format).
    FileFormat,
    /// Name of the current demuxer. (This is useless.)
    ///
    /// (Renamed from demuxer.)
    CurrentDemuxer,
    /// Filename (full path) of the stream layer filename.
    ///
    /// (This is probably useless and is almost never different from path.)
    StreamPath,
    /// Raw byte position in source stream.
    ///
    /// Technically, this returns the position of the most recent packet passed to a decoder.
    StreamPos,
    /// Raw end position in bytes in source stream.
    StreamEnd,
    /// Duration of the current file in seconds.
    ///
    /// If the duration is unknown, the property is unavailable.
    /// Note that the file duration is not always exactly known, so this is an estimate.
    ///
    /// This replaces the length property,
    /// which was deprecated after the mpv 0.9 release.
    /// (The semantics are the same.)
    Duration,
    /*

    */
    // avsync,
    // /// Last A/V synchronization difference. Unavailable if audio or video is disabled.
    // total-avsync-change,
    // /// Total A-V sync correction done. Unavailable if audio or video is disabled.
    // decoder-frame-drop-count,
    //
    // /// Video frames dropped by decoder, because video is too far behind audio (when using --framedrop=decoder). Sometimes, this may be incremented in other situations, e.g. when video packets are damaged, or the decoder doesn't follow the usual rules. Unavailable if video is disabled.
    //
    // /// drop-frame-count is a deprecated alias.
    // frame-drop-count,
    //
    // /// Frames dropped by VO (when using --framedrop=vo).
    //
    // /// vo-drop-frame-count is a deprecated alias.
    // mistimed-frame-count,
    // /// Number of video frames that were not timed correctly in display-sync mode for the sake of keeping A/V sync. This does not include external circumstances, such as video rendering being too slow or the graphics driver somehow skipping a vsync. It does not include rounding errors either (which can happen especially with bad source timestamps). For example, using the display-desync mode should never change this value from 0.
    // vsync-ratio,
    // /// For how many vsyncs a frame is displayed on average. This is available if display-sync is active only. For 30 FPS video on a 60 Hz screen, this will be 2. This is the moving average of what actually has been scheduled, so 24 FPS on 60 Hz will never remain exactly on 2.5, but jitter depending on the last frame displayed.
    // vo-delayed-frame-count,
    // /// Estimated number of frames delayed due to external circumstances in display-sync mode. Note that in general, mpv has to guess that this is happening, and the guess can be inaccurate.
    /*

    */
    /// **(RW)** Position in current file (0-100).
    ///
    /// The advantage over using this instead of calculating it out of other properties
    /// is that it properly falls back to estimating the playback position from the byte position,
    /// if the file duration is not known.
    PercentPos,
    /// **(RW)** Position in current file in seconds.
    TimePos,
    /// Always returns 0.
    ///
    /// Before mpv 0.14, this used to return the start time of the file
    /// (could affect e.g. transport streams).
    ///
    /// See `--rebase-start-time` option.
    #[deprecated]
    TimeStart,
    /// Remaining length of the file in seconds.
    ///
    /// Note that the file duration is not always exactly known, so this is an estimate.
    TimeRemaining,
    /*

    */
    // audio-pts (R),
    // /// Current audio playback position in current file in seconds. Unlike time-pos, this updates more often than once per frame. For audio-only files, it is mostly equivalent to time-pos, while for video-only files this property is not available.
    // playtime-remaining,
    // /// time-remaining scaled by the current speed.
    /*

    */
    /// **(RW)** Position in current file in seconds.
    ///
    /// Unlike `time-pos`, the time is clamped to the range of the file.
    /// (Inaccurate file durations etc. could make it go out of range.
    /// Useful on attempts to seek outside of the file,
    /// as the seek target time is considered the current position during seeking.)
    PlaybackTime,
    /*

    */
    // chapter (RW),
    // /// Current chapter number. The number of the first chapter is 0.
    // edition (RW),
    //
    // /// Current MKV edition number. Setting this property to a different value will restart playback. The number of the first edition is 0.
    //
    // /// Before mpv 0.31.0, this showed the actual edition selected at runtime, if you didn't set the option or property manually. With mpv 0.31.0 and later, this strictly returns the user-set option or property value, and the current-edition property was added to return the runtime selected edition (this matters with --edition=auto, the default).
    // current-edition,
    // /// Currently selected edition. This property is unavailable if no file is loaded, or the file has no editions. (Matroska files make a difference between having no editions and a single edition, which will be reflected by the property, although in practice it does not matter.)
    // chapters,
    // /// Number of chapters.
    // editions,
    // /// Number of MKV editions.
    // edition-list,
    //
    // /// List of editions, current entry marked. Currently, the raw property value is useless.
    //
    // /// This has a number of sub-properties. Replace N with the 0-based edition index.
    //
    // /// edition-list/count
    // ///     Number of editions. If there are no editions, this can be 0 or 1 (1 if there's a useless dummy edition).
    // /// edition-list/N/id
    // ///     Edition ID as integer. Use this to set the edition property. Currently, this is the same as the edition index.
    // /// edition-list/N/default
    // ///     yes if this is the default edition, no otherwise.
    // /// edition-list/N/title
    // ///     Edition title as stored in the file. Not always available.
    //
    // /// When querying the property with the client API using MPV_FORMAT_NODE, or with Lua mp.get_property_native, this will return a mpv_node with the following contents:
    //
    // /// MPV_FORMAT_NODE_ARRAY
    // ///     MPV_FORMAT_NODE_MAP (for each edition)
    // ///         "id"                MPV_FORMAT_INT64
    // ///         "title"             MPV_FORMAT_STRING
    // ///         "default"           MPV_FORMAT_FLAG
    //
    // metadata,
    //
    // /// Metadata key/value pairs.
    //
    // /// If the property is accessed with Lua's mp.get_property_native, this returns a table with metadata keys mapping to metadata values. If it is accessed with the client API, this returns a MPV_FORMAT_NODE_MAP, with tag keys mapping to tag values.
    //
    // /// For OSD, it returns a formatted list. Trying to retrieve this property as a raw string doesn't work.
    //
    // /// This has a number of sub-properties:
    //
    // /// metadata/by-key/<key>
    // ///     Value of metadata entry <key>.
    // /// metadata/list/count
    // ///     Number of metadata entries.
    // /// metadata/list/N/key
    // ///     Key name of the Nth metadata entry. (The first entry is 0).
    // /// metadata/list/N/value
    // ///     Value of the Nth metadata entry.
    // /// metadata/<key>
    // ///     Old version of metadata/by-key/<key>. Use is discouraged, because the metadata key string could conflict with other sub-properties.
    //
    // /// The layout of this property might be subject to change. Suggestions are welcome how exactly this property should work.
    //
    // /// When querying the property with the client API using MPV_FORMAT_NODE, or with Lua mp.get_property_native, this will return a mpv_node with the following contents:
    //
    // /// MPV_FORMAT_NODE_MAP
    // ///     (key and string value for each metadata entry)
    //
    // filtered-metadata,
    // /// Like metadata, but includes only fields listed in the --display-tags option. This is the same set of tags that is printed to the terminal.
    // chapter-metadata,
    //
    // /// Metadata of current chapter. Works similar to metadata property. It also allows the same access methods (using sub-properties).
    //
    // /// Per-chapter metadata is very rare. Usually, only the chapter name (title) is set.
    //
    // /// For accessing other information, like chapter start, see the chapter-list property.
    // vf-metadata/<filter-label>,
    //
    // /// Metadata added by video filters. Accessed by the filter label, which, if not explicitly specified using the @filter-label: syntax, will be <filter-name>NN.
    //
    // /// Works similar to metadata property. It allows the same access methods (using sub-properties).
    //
    // /// An example of this kind of metadata are the cropping parameters added by --vf=lavfi=cropdetect.
    // af-metadata/<filter-label>,
    // /// Equivalent to vf-metadata/<filter-label>, but for audio filters.
    // idle-active,
    //
    // /// Return yes if no file is loaded, but the player is staying around because of the --idle option.
    //
    // /// (Renamed from idle.)
    // core-idle,
    //
    // /// Return yes if the playback core is paused, otherwise no. This can be different pause in special situations, such as when the player pauses itself due to low network cache.
    //
    // /// This also returns yes if playback is restarting or if nothing is playing at all. In other words, it's only no if there's actually video playing. (Behavior since mpv 0.7.0.)
    // cache-speed (R),
    //
    // /// Current I/O read speed between the cache and the lower layer (like network). This gives the number bytes per seconds over a 1 second window (using the type MPV_FORMAT_INT64 for the client API).
    //
    // /// This is the same as demuxer-cache-state/raw-input-rate.
    // demuxer-cache-duration,
    // /// Approximate duration of video buffered in the demuxer, in seconds. The guess is very unreliable, and often the property will not be available at all, even if data is buffered.
    // demuxer-cache-time,
    // /// Approximate time of video buffered in the demuxer, in seconds. Same as demuxer-cache-duration but returns the last timestamp of buffered data in demuxer.
    // demuxer-cache-idle,
    // /// Returns yes if the demuxer is idle, which means the demuxer cache is filled to the requested amount, and is currently not reading more data.
    // demuxer-cache-state,
    //
    // /// Various undocumented or half-documented things.
    //
    // /// Each entry in seekable-ranges represents a region in the demuxer cache that can be seeked to. If there are multiple demuxers active, this only returns information about the "main" demuxer, but might be changed in future to return unified information about all demuxers. The ranges are in arbitrary order. Often, ranges will overlap for a bit, before being joined. In broken corner cases, ranges may overlap all over the place.
    //
    // /// The end of a seek range is usually smaller than the value returned by the demuxer-cache-time property, because that property returns the guessed buffering amount, while the seek ranges represent the buffered data that can actually be used for cached seeking.
    //
    // /// bof-cached indicates whether the seek range with the lowest timestamp points to the beginning of the stream (BOF). This implies you cannot seek before this position at all. eof-cached indicates whether the seek range with the highest timestamp points to the end of the stream (EOF). If both bof-cached and eof-cached are set to yes, and there's only 1 cache range, the entire stream is cached.
    //
    // /// fw-bytes is the number of bytes of packets buffered in the range starting from the current decoding position. This is a rough estimate (may not account correctly for various overhead), and stops at the demuxer position (it ignores seek ranges after it).
    //
    // /// file-cache-bytes is the number of bytes stored in the file cache. This includes all overhead, and possibly unused data (like pruned data). This member is missing if the file cache is not active.
    //
    // /// cache-duration is demuxer-cache-duration. Missing if unavailable.
    //
    // /// raw-input-rate is the estimated input rate of the network layer (or any other byte-oriented input layer) in bytes per second. May be inaccurate or missing.
    //
    // /// When querying the property with the client API using MPV_FORMAT_NODE, or with Lua mp.get_property_native, this will return a mpv_node with the following contents:
    //
    // /// MPV_FORMAT_NODE_MAP
    // ///     "seekable-ranges"   MPV_FORMAT_NODE_ARRAY
    // ///         MPV_FORMAT_NODE_MAP
    // ///             "start"             MPV_FORMAT_DOUBLE
    // ///             "end"               MPV_FORMAT_DOUBLE
    // ///     "bof-cached"        MPV_FORMAT_FLAG
    // ///     "eof-cached"        MPV_FORMAT_FLAG
    // ///     "fw-bytes"          MPV_FORMAT_INT64
    // ///     "file-cache-bytes"  MPV_FORMAT_INT64
    // ///     "cache-duration"    MPV_FORMAT_DOUBLE
    // ///     "raw-input-rate"    MPV_FORMAT_INT64
    //
    // /// Other fields (might be changed or removed in the future):
    //
    // /// eof
    // ///     True if the reader thread has hit the end of the file.
    // /// underrun
    // ///     True if the reader thread could not satisfy a decoder's request for a new packet.
    // /// idle
    // ///     True if the thread is currently not reading.
    // /// total-bytes
    // ///     Sum of packet bytes (plus some overhead estimation) of the entire packet queue, including cached seekable ranges.
    //
    // demuxer-via-network,
    // /// Returns yes if the stream demuxed via the main demuxer is most likely played via network. What constitutes "network" is not always clear, might be used for other types of untrusted streams, could be wrong in certain cases, and its definition might be changing. Also, external files (like separate audio files or streams) do not influence the value of this property (currently).
    // demuxer-start-time (R),
    // /// Returns the start time reported by the demuxer in fractional seconds.
    // paused-for-cache,
    // /// Returns yes when playback is paused because of waiting for the cache.
    // cache-buffering-state,
    // /// Return the percentage (0-100) of the cache fill status until the player will unpause (related to paused-for-cache).
    // eof-reached,
    // /// Returns yes if end of playback was reached, no otherwise. Note that this is usually interesting only if --keep-open is enabled, since otherwise the player will immediately play the next file (or exit or enter idle mode), and in these cases the eof-reached property will logically be cleared immediately after it's set.
    /*

    */
    /// Returns yes if the player is currently seeking, or otherwise trying to restart playback.
    ///
    /// (It's possible that it returns yes while a file is loaded.
    /// This is because the same underlying code is used for seeking and resyncing.)
    Seeking,
    /*

    */
    // mixer-active,
    //
    // /// Return yes if the audio mixer is active, no otherwise.
    //
    // /// This option is relatively useless. Before mpv 0.18.1, it could be used to infer behavior of the volume property.
    // ao-volume (RW),
    // /// System volume. This property is available only if mpv audio output is currently active, and only if the underlying implementation supports volume control. What this option does depends on the API. For example, on ALSA this usually changes system-wide audio, while with PulseAudio this controls per-application volume.
    // ao-mute (RW),
    // /// Similar to ao-volume, but controls the mute state. May be unimplemented even if ao-volume works.
    // audio-codec,
    // /// Audio codec selected for decoding.
    // audio-codec-name,
    // /// Audio codec.
    // audio-params,
    //
    // /// Audio format as output by the audio decoder. This has a number of sub-properties:
    //
    // /// audio-params/format
    // ///     The sample format as string. This uses the same names as used in other places of mpv.
    // /// audio-params/samplerate
    // ///     Samplerate.
    // /// audio-params/channels
    // ///     The channel layout as a string. This is similar to what the --audio-channels accepts.
    // /// audio-params/hr-channels
    // ///     As channels, but instead of the possibly cryptic actual layout sent to the audio device, return a hopefully more human readable form. (Usually only audio-out-params/hr-channels makes sense.)
    // /// audio-params/channel-count
    // ///     Number of audio channels. This is redundant to the channels field described above.
    //
    // /// When querying the property with the client API using MPV_FORMAT_NODE, or with Lua mp.get_property_native, this will return a mpv_node with the following contents:
    //
    // /// MPV_FORMAT_NODE_MAP
    // ///     "format"            MPV_FORMAT_STRING
    // ///     "samplerate"        MPV_FORMAT_INT64
    // ///     "channels"          MPV_FORMAT_STRING
    // ///     "channel-count"     MPV_FORMAT_INT64
    // ///     "hr-channels"       MPV_FORMAT_STRING
    //
    // audio-out-params,
    // /// Same as audio-params, but the format of the data written to the audio API.
    // colormatrix (R),
    // /// Redirects to video-params/colormatrix. This parameter (as well as similar ones) can be overridden with the format video filter.
    // colormatrix-input-range (R),
    // /// See colormatrix.
    // colormatrix-primaries (R),
    // /// See colormatrix.
    // hwdec (RW),
    //
    // /// Reflects the --hwdec option.
    //
    // /// Writing to it may change the currently used hardware decoder, if possible. (Internally, the player may reinitialize the decoder, and will perform a seek to refresh the video properly.) You can watch the other hwdec properties to see whether this was successful.
    //
    // /// Unlike in mpv 0.9.x and before, this does not return the currently active hardware decoder. Since mpv 0.18.0, hwdec-current is available for this purpose.
    // hwdec-current,
    // /// Return the current hardware decoding in use. If decoding is active, return one of the values used by the hwdec option/property. no indicates software decoding. If no decoder is loaded, the property is unavailable.
    // hwdec-interop,
    //
    // /// This returns the currently loaded hardware decoding/output interop driver. This is known only once the VO has opened (and possibly later). With some VOs (like gpu), this might be never known in advance, but only when the decoder attempted to create the hw decoder successfully. (Using --gpu-hwdec-interop can load it eagerly.) If there are multiple drivers loaded, they will be separated by ,.
    //
    // /// If no VO is active or no interop driver is known, this property is unavailable.
    //
    // /// This does not necessarily use the same values as hwdec. There can be multiple interop drivers for the same hardware decoder, depending on platform and VO.
    // video-format,
    // /// Video format as string.
    // video-codec,
    // /// Video codec selected for decoding.
    // width, height,
    // /// Video size. This uses the size of the video as decoded, or if no video frame has been decoded yet, the (possibly incorrect) container indicated size.
    // video-params,
    //
    // /// Video parameters, as output by the decoder (with overrides like aspect etc. applied). This has a number of sub-properties:
    //
    // /// video-params/pixelformat
    // ///     The pixel format as string. This uses the same names as used in other places of mpv.
    // /// video-params/average-bpp
    // ///     Average bits-per-pixel as integer. Subsampled planar formats use a different resolution, which is the reason this value can sometimes be odd or confusing. Can be unavailable with some formats.
    // /// video-params/w, video-params/h
    // ///     Video size as integers, with no aspect correction applied.
    // /// video-params/dw, video-params/dh
    // ///     Video size as integers, scaled for correct aspect ratio.
    // /// video-params/aspect
    // ///     Display aspect ratio as float.
    // /// video-params/par
    // ///     Pixel aspect ratio.
    // /// video-params/colormatrix
    // ///     The colormatrix in use as string. (Exact values subject to change.)
    // /// video-params/colorlevels
    // ///     The colorlevels as string. (Exact values subject to change.)
    // /// video-params/primaries
    // ///     The primaries in use as string. (Exact values subject to change.)
    // /// video-params/gamma
    // ///     The gamma function in use as string. (Exact values subject to change.)
    // /// video-params/sig-peak
    // ///     The video file's tagged signal peak as float.
    // /// video-params/light
    // ///     The light type in use as a string. (Exact values subject to change.)
    // /// video-params/chroma-location
    // ///     Chroma location as string. (Exact values subject to change.)
    // /// video-params/rotate
    // ///     Intended display rotation in degrees (clockwise).
    // /// video-params/stereo-in
    // ///     Source file stereo 3D mode. (See the format video filter's stereo-in option.)
    //
    // /// When querying the property with the client API using MPV_FORMAT_NODE, or with Lua mp.get_property_native, this will return a mpv_node with the following contents:
    //
    // /// MPV_FORMAT_NODE_MAP
    // ///     "pixelformat"       MPV_FORMAT_STRING
    // ///     "w"                 MPV_FORMAT_INT64
    // ///     "h"                 MPV_FORMAT_INT64
    // ///     "dw"                MPV_FORMAT_INT64
    // ///     "dh"                MPV_FORMAT_INT64
    // ///     "aspect"            MPV_FORMAT_DOUBLE
    // ///     "par"               MPV_FORMAT_DOUBLE
    // ///     "colormatrix"       MPV_FORMAT_STRING
    // ///     "colorlevels"       MPV_FORMAT_STRING
    // ///     "primaries"         MPV_FORMAT_STRING
    // ///     "gamma"             MPV_FORMAT_STRING
    // ///     "sig-peak"          MPV_FORMAT_DOUBLE
    // ///     "light"             MPV_FORMAT_STRING
    // ///     "chroma-location"   MPV_FORMAT_STRING
    // ///     "rotate"            MPV_FORMAT_INT64
    // ///     "stereo-in"         MPV_FORMAT_STRING
    //
    // dwidth, dheight,
    //
    // /// Video display size. This is the video size after filters and aspect scaling have been applied. The actual video window size can still be different from this, e.g. if the user resized the video window manually.
    //
    // /// These have the same values as video-out-params/dw and video-out-params/dh.
    // video-dec-params,
    // /// Exactly like video-params, but no overrides applied.
    // video-out-params,
    //
    // /// Same as video-params, but after video filters have been applied. If there are no video filters in use, this will contain the same values as video-params. Note that this is still not necessarily what the video window uses, since the user can change the window size, and all real VOs do their own scaling independently from the filter chain.
    //
    // /// Has the same sub-properties as video-params.
    // video-frame-info,
    //
    // /// Approximate information of the current frame. Note that if any of these are used on OSD, the information might be off by a few frames due to OSD redrawing and frame display being somewhat disconnected, and you might have to pause and force a redraw.
    //
    // /// Sub-properties:
    //
    // /// video-frame-info/picture-type
    // /// video-frame-info/interlaced
    // /// video-frame-info/tff
    // /// video-frame-info/repeat
    //
    // container-fps,
    //
    // /// Container FPS. This can easily contain bogus values. For videos that use modern container formats or video codecs, this will often be incorrect.
    //
    // /// (Renamed from fps.)
    // estimated-vf-fps,
    // /// Estimated/measured FPS of the video filter chain output. (If no filters are used, this corresponds to decoder output.) This uses the average of the 10 past frame durations to calculate the FPS. It will be inaccurate if frame-dropping is involved (such as when framedrop is explicitly enabled, or after precise seeking). Files with imprecise timestamps (such as Matroska) might lead to unstable results.
    // window-scale (RW),
    //
    // /// Window size multiplier. Setting this will resize the video window to the values contained in dwidth and dheight multiplied with the value set with this property. Setting 1 will resize to original video size (or to be exact, the size the video filters output). 2 will set the double size, 0.5 halves the size.
    //
    // /// See current-window-scale for the value derived from the actual window size.
    //
    // /// Since mpv 0.31.0, this always returns the previously set value (or the default value), instead of the value implied by the actual window size. Before mpv 0.31.0, this returned what current-window-scale returns now, after the window was created.
    // current-window-scale,
    // /// The window-scale value calculated from the current window size. This has the same value as window-scale if the window size was not changed since setting the option, and the window size was not restricted in other ways. The property is unavailable if no video is active.
    // display-names,
    // /// Names of the displays that the mpv window covers. On X11, these are the xrandr names (LVDS1, HDMI1, DP1, VGA1, etc.). On Windows, these are the GDI names (\.DISPLAY1, \.DISPLAY2, etc.) and the first display in the list will be the one that Windows considers associated with the window (as determined by the MonitorFromWindow API.) On macOS these are the Display Product Names as used in the System Information and only one display name is returned since a window can only be on one screen.
    // display-fps,
    //
    // /// The refresh rate of the current display. Currently, this is the lowest FPS of any display covered by the video, as retrieved by the underlying system APIs (e.g. xrandr on X11). It is not the measured FPS. It's not necessarily available on all platforms. Note that any of the listed facts may change any time without a warning.
    //
    // /// Writing to this property is deprecated. It has the same effect as writing to override-display-fps. Since mpv 0.31.0, this property is unavailable if no display FPS was reported (e.g. if no video is active), while in older versions, it returned the --display-fps option value.
    // estimated-display-fps,
    // /// Only available if display-sync mode (as selected by --video-sync) is active. Returns the actual rate at which display refreshes seem to occur, measured by system time.
    // vsync-jitter,
    // /// Estimated deviation factor of the vsync duration.
    // display-hidpi-scale,
    // /// The HiDPI scale factor as reported by the windowing backend. If no VO is active, or if the VO does not report a value, this property is unavailable. It may be saner to report an absolute DPI, however, this is the way HiDPI support is implemented on most OS APIs. See also --hidpi-window-scale.
    // video-aspect (RW),
    //
    // /// Deprecated. This is tied to --video-aspect-override, but always reports the current video aspect if video is active.
    //
    // /// The read and write components of this option can be split up into video-params/aspect and video-aspect-override respectively.
    // osd-width, osd-height,
    //
    // /// Last known OSD width (can be 0). This is needed if you want to use the overlay-add command. It gives you the actual OSD size, which can be different from the window size in some cases.
    //
    // /// Alias to osd-dimensions/w and osd-dimensions/h.
    // osd-par,
    //
    // /// Last known OSD display pixel aspect (can be 0).
    //
    // /// Alias to osd-dimensions/osd-par.
    // osd-dimensions,
    //
    // /// Last known OSD dimensions.
    //
    // /// Has the following sub-properties (which can be read as MPV_FORMAT_NODE or Lua table with mp.get_property_native):
    //
    // /// w
    // ///     Size of the VO window in OSD render units (usually pixels, but may be scaled pixels with VOs like xv).
    // /// h
    // ///     Size of the VO window in OSD render units,
    // /// par
    // ///     Pixel aspect ratio of the OSD (usually 1).
    // /// aspect
    // ///     Display aspect ratio of the VO window. (Computing from the properties above.)
    // /// mt, mb, ml, mr
    // ///     OSD to video margins (top, bottom, left, right). This describes the area into which the video is rendered.
    //
    // /// Any of these properties may be unavailable or set to dummy values if the VO window is not created or visible.
    // sub-text,
    //
    // /// Return the current subtitle text regardless of sub visibility. Formatting is stripped. If the subtitle is not text-based (i.e. DVD/BD subtitles), an empty string is returned.
    //
    // /// This property is experimental and might be removed in the future.
    // sub-text-ass,
    //
    // /// Like sub-text, but return the text in ASS format. Text subtitles in other formats are converted. For native ASS subtitles, events that do not contain any text (but vector drawings etc.) are not filtered out. If multiple events match with the current playback time, they are concatenated with line breaks. Contains only the "Text" part of the events.
    //
    // /// This property is not enough to render ASS subtitles correctly, because ASS header and per-event metadata are not returned. You likely need to do further filtering on the returned string to make it useful.
    //
    // /// This property is experimental and might be removed in the future.
    // sub-start,
    // /// Return the current subtitle start time (in seconds). If there's multiple current subtitles, returns the first start time. If no current subtitle is present null is returned instead.
    // sub-end,
    // /// Return the current subtitle end time (in seconds). If there's multiple current subtitles, return the last end time. If no current subtitle is present, or if it's present but has unknown or incorrect duration, null is returned instead.
    // playlist-pos (RW),
    //
    // /// Current position on playlist. The first entry is on position 0. Writing to this property may start playback at the new position.
    //
    // /// In some cases, this is not necessarily the currently playing file. See explanation of current and playing flags in playlist.
    //
    // /// If there the playlist is empty, or if it's non-empty, but no entry is "current", this property returns -1. Likewise, writing -1 will put the player into idle mode (or exit playback if idle mode is not enabled). If an out of range index is written to the property, this behaves as if writing -1. (Before mpv 0.33.0, instead of returning -1, this property was unavailable if no playlist entry was current.)
    //
    // /// Writing the current value back to the property is subject to change. Currently, it will restart playback of the playlist entry. But in the future, writing the current value will be ignored. Use the playlist-play-index command to get guaranteed behavior.
    // playlist-pos-1 (RW),
    // /// Same as playlist-pos, but 1-based.
    // playlist-current-pos (RW),
    //
    // /// Index of the "current" item on playlist. This usually, but not necessarily, the currently playing item (see playlist-playing-pos). Depending on the exact internal state of the player, it may refer to the playlist item to play next, or the playlist item used to determine what to play next.
    //
    // /// For reading, this is exactly the same as playlist-pos.
    //
    // /// For writing, this only sets the position of the "current" item, without stopping playback of the current file (or starting playback, if this is done in idle mode). Use -1 to remove the current flag.
    //
    // /// This property is only vaguely useful. If set during playback, it will typically cause the playlist entry after it to be played next. Another possibly odd observable state is that if playlist-next is run during playback, this property is set to the playlist entry to play next (unlike the previous case). There is an internal flag that decides whether the current playlist entry or the next one should be played, and this flag is currently inaccessible for API users. (Whether this behavior will kept is possibly subject to change.)
    // playlist-playing-pos,
    //
    // /// Index of the "playing" item on playlist. A playlist item is "playing" if it's being loaded, actually playing, or being unloaded. This property is set during the MPV_EVENT_START_FILE (start-file) and the MPV_EVENT_START_END (end-file) events. Outside of that, it returns -1. If the playlist entry was somehow removed during playback, but playback hasn't stopped yet, or is in progress of being stopped, it also returns -1. (This can happen at least during state transitions.)
    //
    // /// In the "playing" state, this is usually the same as playlist-pos, except during state changes, or if playlist-current-pos was written explicitly.
    // playlist-count,
    // /// Number of total playlist entries.
    // playlist,
    //
    // /// Playlist, current entry marked. Currently, the raw property value is useless.
    //
    // /// This has a number of sub-properties. Replace N with the 0-based playlist entry index.
    //
    // /// playlist/count
    // ///     Number of playlist entries (same as playlist-count).
    // /// playlist/N/filename
    // ///     Filename of the Nth entry.
    // /// playlist/N/playing
    // ///     yes if the playlist-playing-pos property points to this entry, unavailable or no otherwise.
    // /// playlist/N/current
    // ///     yes if the playlist-current-pos property points to this entry, unavailable or no otherwise.
    // /// playlist/N/title
    // ///     Name of the Nth entry. Only available if the playlist file contains such fields, and only if mpv's parser supports it for the given playlist format.
    // /// playlist/N/id
    // ///     Unique ID for this entry. This is an automatically assigned integer ID that is unique for the entire life time of the current mpv core instance. Other commands, events, etc. use this as playlist_entry_id fields.
    //
    // /// When querying the property with the client API using MPV_FORMAT_NODE, or with Lua mp.get_property_native, this will return a mpv_node with the following contents:
    //
    // /// MPV_FORMAT_NODE_ARRAY
    // ///     MPV_FORMAT_NODE_MAP (for each playlist entry)
    // ///         "filename"  MPV_FORMAT_STRING
    // ///         "current"   MPV_FORMAT_FLAG (might be missing; since mpv 0.7.0)
    // ///         "playing"   MPV_FORMAT_FLAG (same)
    // ///         "title"     MPV_FORMAT_STRING (optional)
    // ///         "id"        MPV_FORMAT_INT64
    //
    // track-list,
    //
    // /// List of audio/video/sub tracks, current entry marked. Currently, the raw property value is useless.
    //
    // /// This has a number of sub-properties. Replace N with the 0-based track index.
    //
    // /// track-list/count
    // ///     Total number of tracks.
    // /// track-list/N/id
    // ///     The ID as it's used for -sid/--aid/--vid. This is unique within tracks of the same type (sub/audio/video), but otherwise not.
    // /// track-list/N/type
    // ///     String describing the media type. One of audio, video, sub.
    // /// track-list/N/src-id
    // ///     Track ID as used in the source file. Not always available. (It is missing if the format has no native ID, if the track is a pseudo-track that does not exist in this way in the actual file, or if the format is handled by libavformat, and the format was not whitelisted as having track IDs.)
    // /// track-list/N/title
    // ///     Track title as it is stored in the file. Not always available.
    // /// track-list/N/lang
    // ///     Track language as identified by the file. Not always available.
    // /// track-list/N/albumart
    // ///     yes if this is a video track that consists of a single picture, no or unavailable otherwise. This is used for video tracks that are really attached pictures in audio files.
    // /// track-list/N/default
    // ///     yes if the track has the default flag set in the file, no otherwise.
    // /// track-list/N/forced
    // ///     yes if the track has the forced flag set in the file, no otherwise.
    // /// track-list/N/codec
    // ///     The codec name used by this track, for example h264. Unavailable in some rare cases.
    // /// track-list/N/external
    // ///     yes if the track is an external file, no otherwise. This is set for separate subtitle files.
    // /// track-list/N/external-filename
    // ///     The filename if the track is from an external file, unavailable otherwise.
    // /// track-list/N/selected
    // ///     yes if the track is currently decoded, no otherwise.
    // /// track-list/N/ff-index
    // ///     The stream index as usually used by the FFmpeg utilities. Note that this can be potentially wrong if a demuxer other than libavformat (--demuxer=lavf) is used. For mkv files, the index will usually match even if the default (builtin) demuxer is used, but there is no hard guarantee.
    // /// track-list/N/decoder-desc
    // ///     If this track is being decoded, the human-readable decoder name,
    // /// track-list/N/demux-w, track-list/N/demux-h
    // ///     Video size hint as indicated by the container. (Not always accurate.)
    // /// track-list/N/demux-channel-count
    // ///     Number of audio channels as indicated by the container. (Not always accurate - in particular, the track could be decoded as a different number of channels.)
    // /// track-list/N/demux-channels
    // ///     Channel layout as indicated by the container. (Not always accurate.)
    // /// track-list/N/demux-samplerate
    // ///     Audio sample rate as indicated by the container. (Not always accurate.)
    // /// track-list/N/demux-fps
    // ///     Video FPS as indicated by the container. (Not always accurate.)
    // /// track-list/N/demux-bitrate
    // ///     Audio average bitrate, in bits per second. (Not always accurate.)
    // /// track-list/N/demux-rotation
    // ///     Video clockwise rotation metadata, in degrees.
    // /// track-list/N/demux-par
    // ///     Pixel aspect ratio.
    // /// track-list/N/audio-channels (deprecated)
    // ///     Deprecated alias for track-list/N/demux-channel-count.
    // /// track-list/N/replaygain-track-peak, track-list/N/replaygain-track-gain
    // ///     Per-track replaygain values. Only available for audio tracks with corresponding information stored in the source file.
    // /// track-list/N/replaygain-album-peak, track-list/N/replaygain-album-gain
    // ///     Per-album replaygain values. If the file has per-track but no per-album information, the per-album values will be copied from the per-track values currently. It's possible that future mpv versions will make these properties unavailable instead in this case.
    //
    // /// When querying the property with the client API using MPV_FORMAT_NODE, or with Lua mp.get_property_native, this will return a mpv_node with the following contents:
    //
    // /// MPV_FORMAT_NODE_ARRAY
    // ///     MPV_FORMAT_NODE_MAP (for each track)
    // ///         "id"                MPV_FORMAT_INT64
    // ///         "type"              MPV_FORMAT_STRING
    // ///         "src-id"            MPV_FORMAT_INT64
    // ///         "title"             MPV_FORMAT_STRING
    // ///         "lang"              MPV_FORMAT_STRING
    // ///         "albumart"          MPV_FORMAT_FLAG
    // ///         "default"           MPV_FORMAT_FLAG
    // ///         "forced"            MPV_FORMAT_FLAG
    // ///         "selected"          MPV_FORMAT_FLAG
    // ///         "external"          MPV_FORMAT_FLAG
    // ///         "external-filename" MPV_FORMAT_STRING
    // ///         "codec"             MPV_FORMAT_STRING
    // ///         "ff-index"          MPV_FORMAT_INT64
    // ///         "decoder-desc"      MPV_FORMAT_STRING
    // ///         "demux-w"           MPV_FORMAT_INT64
    // ///         "demux-h"           MPV_FORMAT_INT64
    // ///         "demux-channel-count" MPV_FORMAT_INT64
    // ///         "demux-channels"    MPV_FORMAT_STRING
    // ///         "demux-samplerate"  MPV_FORMAT_INT64
    // ///         "demux-fps"         MPV_FORMAT_DOUBLE
    // ///         "demux-bitrate"     MPV_FORMAT_INT64
    // ///         "demux-rotation"    MPV_FORMAT_INT64
    // ///         "demux-par"         MPV_FORMAT_DOUBLE
    // ///         "audio-channels"    MPV_FORMAT_INT64
    // ///         "replaygain-track-peak" MPV_FORMAT_DOUBLE
    // ///         "replaygain-track-gain" MPV_FORMAT_DOUBLE
    // ///         "replaygain-album-peak" MPV_FORMAT_DOUBLE
    // ///         "replaygain-album-gain" MPV_FORMAT_DOUBLE
    //
    // chapter-list,
    //
    // /// List of chapters, current entry marked. Currently, the raw property value is useless.
    //
    // /// This has a number of sub-properties. Replace N with the 0-based chapter index.
    //
    // /// chapter-list/count
    // ///     Number of chapters.
    // /// chapter-list/N/title
    // ///     Chapter title as stored in the file. Not always available.
    // /// chapter-list/N/time
    // ///     Chapter start time in seconds as float.
    //
    // /// When querying the property with the client API using MPV_FORMAT_NODE, or with Lua mp.get_property_native, this will return a mpv_node with the following contents:
    //
    // /// MPV_FORMAT_NODE_ARRAY
    // ///     MPV_FORMAT_NODE_MAP (for each chapter)
    // ///         "title" MPV_FORMAT_STRING
    // ///         "time"  MPV_FORMAT_DOUBLE
    //
    // af, vf (RW),
    //
    // /// See --vf/--af and the vf/af command.
    //
    // /// When querying the property with the client API using MPV_FORMAT_NODE, or with Lua mp.get_property_native, this will return a mpv_node with the following contents:
    //
    // /// MPV_FORMAT_NODE_ARRAY
    // ///     MPV_FORMAT_NODE_MAP (for each filter entry)
    // ///         "name"      MPV_FORMAT_STRING
    // ///         "label"     MPV_FORMAT_STRING [optional]
    // ///         "enabled"   MPV_FORMAT_FLAG [optional]
    // ///         "params"    MPV_FORMAT_NODE_MAP [optional]
    // ///             "key"   MPV_FORMAT_STRING
    // ///             "value" MPV_FORMAT_STRING
    //
    // /// It's also possible to write the property using this format.
    // seekable,
    // /// Return whether it's generally possible to seek in the current file.
    // partially-seekable,
    //
    // /// Return yes if the current file is considered seekable, but only because the cache is active. This means small relative seeks may be fine, but larger seeks may fail anyway. Whether a seek will succeed or not is generally not known in advance.
    //
    // /// If this property returns true, seekable will also return true.
    // playback-abort,
    // /// Return whether playback is stopped or is to be stopped. (Useful in obscure situations like during on_load hook processing, when the user can stop playback, but the script has to explicitly end processing.)
    // cursor-autohide (RW),
    // /// See --cursor-autohide. Setting this to a new value will always update the cursor, and reset the internal timer.
    // osd-sym-cc,
    // /// Inserts the current OSD symbol as opaque OSD control code (cc). This makes sense only with the show-text command or options which set OSD messages. The control code is implementation specific and is useless for anything else.
    // osd-ass-cc,
    //
    // /// ${osd-ass-cc/0} disables escaping ASS sequences of text in OSD, ${osd-ass-cc/1} enables it again. By default, ASS sequences are escaped to avoid accidental formatting, and this property can disable this behavior. Note that the properties return an opaque OSD control code, which only makes sense for the show-text command or options which set OSD messages.
    //
    // /// Example
    //
    // ///     --osd-status-msg='This is ${osd-ass-cc/0}{\\b1}bold text'
    // ///     show-text "This is ${osd-ass-cc/0}{\b1}bold text"
    //
    // /// Any ASS override tags as understood by libass can be used.
    //
    // /// Note that you need to escape the \ character, because the string is processed for C escape sequences before passing it to the OSD code.
    //
    // /// A list of tags can be found here: http://docs.aegisub.org/latest/ASS_Tags/
    // vo-configured,
    // /// Return whether the VO is configured right now. Usually this corresponds to whether the video window is visible. If the --force-window option is used, this is usually always returns yes.
    // vo-passes,
    //
    // /// Contains introspection about the VO's active render passes and their execution times. Not implemented by all VOs.
    //
    // /// This is further subdivided into two frame types, vo-passes/fresh for fresh frames (which have to be uploaded, scaled, etc.) and vo-passes/redraw for redrawn frames (which only have to be re-painted). The number of passes for any given subtype can change from frame to frame, and should not be relied upon.
    //
    // /// Each frame type has a number of further sub-properties. Replace TYPE with the frame type, N with the 0-based pass index, and M with the 0-based sample index.
    //
    // /// vo-passes/TYPE/count
    // ///     Number of passes.
    // /// vo-passes/TYPE/N/desc
    // ///     Human-friendy description of the pass.
    // /// vo-passes/TYPE/N/last
    // ///     Last measured execution time, in nanoseconds.
    // /// vo-passes/TYPE/N/avg
    // ///     Average execution time of this pass, in nanoseconds. The exact timeframe varies, but it should generally be a handful of seconds.
    // /// vo-passes/TYPE/N/peak
    // ///     The peak execution time (highest value) within this averaging range, in nanoseconds.
    // /// vo-passes/TYPE/N/count
    // ///     The number of samples for this pass.
    // /// vo-passes/TYPE/N/samples/M
    // ///     The raw execution time of a specific sample for this pass, in nanoseconds.
    //
    // /// When querying the property with the client API using MPV_FORMAT_NODE, or with Lua mp.get_property_native, this will return a mpv_node with the following contents:
    //
    // /// MPV_FORMAT_NODE_MAP
    // /// "TYPE" MPV_FORMAT_NODE_ARRAY
    // ///     MPV_FORMAT_NODE_MAP
    // ///         "desc"    MPV_FORMAT_STRING
    // ///         "last"    MPV_FORMAT_INT64
    // ///         "avg"     MPV_FORMAT_INT64
    // ///         "peak"    MPV_FORMAT_INT64
    // ///         "count"   MPV_FORMAT_INT64
    // ///         "samples" MPV_FORMAT_NODE_ARRAY
    // ///              MP_FORMAT_INT64
    //
    // /// Note that directly accessing this structure via subkeys is not supported, the only access is through aforementioned MPV_FORMAT_NODE.
    // perf-info,
    // /// Further performance data. Querying this property triggers internal collection of some data, and may slow down the player. Each query will reset some internal state. Property change notification doesn't and won't work. All of this may change in the future, so don't use this. The builtin stats script is supposed to be the only user; since it's bundled and built with the source code, it can use knowledge of mpv internal to render the information properly. See stats script description for some details.
    // video-bitrate, audio-bitrate, sub-bitrate,
    //
    // /// Bitrate values calculated on the packet level. This works by dividing the bit size of all packets between two keyframes by their presentation timestamp distance. (This uses the timestamps are stored in the file, so e.g. playback speed does not influence the returned values.) In particular, the video bitrate will update only per keyframe, and show the "past" bitrate. To make the property more UI friendly, updates to these properties are throttled in a certain way.
    //
    // /// The unit is bits per second. OSD formatting turns these values in kilobits (or megabits, if appropriate), which can be prevented by using the raw property value, e.g. with ${=video-bitrate}.
    //
    // /// Note that the accuracy of these properties is influenced by a few factors. If the underlying demuxer rewrites the packets on demuxing (done for some file formats), the bitrate might be slightly off. If timestamps are bad or jittery (like in Matroska), even constant bitrate streams might show fluctuating bitrate.
    //
    // /// How exactly these values are calculated might change in the future.
    //
    // /// In earlier versions of mpv, these properties returned a static (but bad) guess using a completely different method.
    // packet-video-bitrate, packet-audio-bitrate, packet-sub-bitrate,
    //
    // /// Old and deprecated properties for video-bitrate, audio-bitrate, sub-bitrate. They behave exactly the same, but return a value in kilobits. Also, they don't have any OSD formatting, though the same can be achieved with e.g. ${=video-bitrate}.
    //
    // /// These properties shouldn't be used anymore.
    // audio-device-list,
    //
    // /// Return the list of discovered audio devices. This is mostly for use with the client API, and reflects what --audio-device=help with the command line player returns.
    //
    // /// When querying the property with the client API using MPV_FORMAT_NODE, or with Lua mp.get_property_native, this will return a mpv_node with the following contents:
    //
    // /// MPV_FORMAT_NODE_ARRAY
    // ///     MPV_FORMAT_NODE_MAP (for each device entry)
    // ///         "name"          MPV_FORMAT_STRING
    // ///         "description"   MPV_FORMAT_STRING
    //
    // /// The name is what is to be passed to the --audio-device option (and often a rather cryptic audio API-specific ID), while description is human readable free form text. The description is set to the device name (minus mpv-specific <driver>/ prefix) if no description is available or the description would have been an empty string.
    //
    // /// The special entry with the name set to auto selects the default audio output driver and the default device.
    //
    // /// The property can be watched with the property observation mechanism in the client API and in Lua scripts. (Technically, change notification is enabled the first time this property is read.)
    // audio-device (RW),
    //
    // /// Set the audio device. This directly reads/writes the --audio-device option, but on write accesses, the audio output will be scheduled for reloading.
    //
    // /// Writing this property while no audio output is active will not automatically enable audio. (This is also true in the case when audio was disabled due to reinitialization failure after a previous write access to audio-device.)
    //
    // /// This property also doesn't tell you which audio device is actually in use.
    //
    // /// How these details are handled may change in the future.
    // current-vo,
    // /// Current video output driver (name as used with --vo).
    // current-ao,
    // /// Current audio output driver (name as used with --ao).
    // shared-script-properties (RW),
    //
    // /// This is a key/value map of arbitrary strings shared between scripts for general use. The player itself does not use any data in it (although some builtin scripts may). The property is not preserved across player restarts.
    //
    // /// This is very primitive, inefficient, and annoying to use. It's a makeshift solution which could go away any time (for example, when a better solution becomes available). This is also why this property has an annoying name. You should avoid using it, unless you absolutely have to.
    //
    // /// Lua scripting has helpers starting with utils.shared_script_property_. They are undocumented because you should not use this property. If you still think you must, you should use the helpers instead of the property directly.
    //
    // /// You are supposed to use the change-list command to modify the contents. Reading, modifying, and writing the property manually could data loss if two scripts update different keys at the same time due to lack of synchronization. The Lua helpers take care of this.
    //
    // /// (There is no way to ensure synchronization if two scripts try to update the same key at the same time.)
    // working-directory,
    // /// Return the working directory of the mpv process. Can be useful for JSON IPC users, because the command line player usually works with relative paths.
    // protocol-list,
    // /// List of protocol prefixes potentially recognized by the player. They are returned without trailing :// suffix (which is still always required). In some cases, the protocol will not actually be supported (consider https if ffmpeg is not compiled with TLS support).
    // decoder-list,
    //
    // /// List of decoders supported. This lists decoders which can be passed to --vd and --ad.
    //
    // /// codec
    // ///     Canonical codec name, which identifies the format the decoder can handle.
    // /// driver
    // ///     The name of the decoder itself. Often, this is the same as codec. Sometimes it can be different. It is used to distinguish multiple decoders for the same codec.
    // /// description
    // ///     Human readable description of the decoder and codec.
    //
    // /// When querying the property with the client API using MPV_FORMAT_NODE, or with Lua mp.get_property_native, this will return a mpv_node with the following contents:
    //
    // /// MPV_FORMAT_NODE_ARRAY
    // ///     MPV_FORMAT_NODE_MAP (for each decoder entry)
    // ///         "codec"         MPV_FORMAT_STRING
    // ///         "driver"        MPV_FORMAT_STRING
    // ///         "description"   MPV_FORMAT_STRING
    //
    // encoder-list,
    // /// List of libavcodec encoders. This has the same format as decoder-list. The encoder names (driver entries) can be passed to --ovc and --oac (without the lavc: prefix required by --vd and --ad).
    // demuxer-lavf-list,
    // /// List of available libavformat demuxers' names. This can be used to check for support for a specific format or use with --demuxer-lavf-format.
    // input-key-list,
    // /// List of Key names, same as output by --input-keylist.
    // mpv-version,
    // /// Return the mpv version/copyright string. Depending on how the binary was built, it might contain either a release version, or just a git hash.
    // mpv-configuration,
    // /// Return the configuration arguments which were passed to the build system (typically the way ./waf configure ... was invoked).
    // ffmpeg-version,
    // /// Return the contents of the av_version_info() API call. This is a string which identifies the build in some way, either through a release version number, or a git hash. This applies to Libav as well (the property is still named the same.) This property is unavailable if mpv is linked against older FFmpeg and Libav versions.
    // libass-version,
    // /// Return the value of ass_library_version(). This is an integer, encoded in a somewhat weird form (apparently "hex BCD"), indicating the release version of the libass library linked to mpv.
    // options/<name> (RW),
    //
    // /// Read-only access to value of option --<name>. Most options can be changed at runtime by writing to this property. Note that many options require reloading the file for changes to take effect. If there is an equivalent property, prefer setting the property instead.
    //
    // /// There shouldn't be any reason to access options/<name> instead of <name>, except in situations in which the properties have different behavior or conflicting semantics.
    // file-local-options/<name>,
    //
    // /// Similar to options/<name>, but when setting an option through this property, the option is reset to its old value once the current file has stopped playing. Trying to write an option while no file is playing (or is being loaded) results in an error.
    //
    // /// (Note that if an option is marked as file-local, even options/ will access the local value, and the old value, which will be restored on end of playback, cannot be read or written until end of playback.)
    // option-info/<name>,
    //
    // /// Additional per-option information.
    //
    // /// This has a number of sub-properties. Replace <name> with the name of a top-level option. No guarantee of stability is given to any of these sub-properties - they may change radically in the feature.
    //
    // /// option-info/<name>/name
    // ///     Returns the name of the option.
    // /// option-info/<name>/type
    // ///     Return the name of the option type, like String or Integer. For many complex types, this isn't very accurate.
    // /// option-info/<name>/set-from-commandline
    // ///     Return yes if the option was set from the mpv command line, no otherwise. What this is set to if the option is e.g. changed at runtime is left undefined (meaning it could change in the future).
    // /// option-info/<name>/set-locally
    // ///     Return yes if the option was set per-file. This is the case with automatically loaded profiles, file-dir configs, and other cases. It means the option value will be restored to the value before playback start when playback ends.
    // /// option-info/<name>/default-value
    // ///     The default value of the option. May not always be available.
    // /// option-info/<name>/min, option-info/<name>/max
    // ///     Integer minimum and maximum values allowed for the option. Only available if the options are numeric, and the minimum/maximum has been set internally. It's also possible that only one of these is set.
    // /// option-info/<name>/choices
    // ///     If the option is a choice option, the possible choices. Choices that are integers may or may not be included (they can be implied by min and max). Note that options which behave like choice options, but are not actual choice options internally, may not have this info available.
    //
    // property-list,
    // /// Return the list of top-level properties.
    // profile-list,
    // /// Return the list of profiles and their contents. This is highly implementation-specific, and may change any time. Currently, it returns an array of options for each profile. Each option has a name and a value, with the value currently always being a string. Note that the options array is not a map, as order matters and duplicate entries are possible. Recursive profiles are not expanded, and show up as special profile options.
    // command-list,
    // /// Return the list of input commands. This returns an array of maps, where each map node represents a command. This map currently only has a single entry: name for the name of the command. (This property is supposed to be a replacement for --input-cmdlist. The option dumps some more information, but it's a valid feature request to extend this property if needed.)
    // input-bindings,
    //
    // /// Return list of current input key bindings. This returns an array of maps, where each map node represents a binding for a single key/command. This map has the following entries:
    //
    // /// key
    // ///     The key name. This is normalized and may look slightly different from how it was specified in the source (e.g. in input.conf).
    // /// cmd
    // ///     The command mapped to the key. (Currently, this is exactly the same string as specified in the source, other than stripping whitespace and comments. It's possible that it will be normalized in the future.)
    // /// is_weak
    // ///     If set to true, any existing and active user bindings will take priority.
    // /// owner
    // ///     If this entry exists, the name of the script (or similar) which added this binding.
    // /// section
    // ///     Name of the section this binding is part of. This is a rarely used mechanism. This entry may be removed or change meaning in the future.
    // /// priority
    // ///     A number. Bindings with a higher value are preferred over bindings with a lower value. If the value is negative, this binding is inactive and will not be triggered by input. Note that mpv does not use this value internally, and matching of bindings may work slightly differently in some cases. In addition, this value is dynamic and can change around at runtime.
    // /// comment
    // ///     If available, the comment following the command on the same line. (For example, the input.conf entry f cycle bla # toggle bla would result in an entry with comment = "toggle bla", cmd = "cycle bla".)
    //
    // /// This property is read-only, and change notification is not supported. Currently, there is no mechanism to change key bindings at runtime, other than scripts adding or removing their own bindings.
    /*

    */
    // These do not appear in the Property List section, but can be queried/set and are important.
    /// **(RW)** Set the startup volume.
    ///
    /// 0 means silence, 100 means no volume reduction or amplification.
    /// Negative values can be passed for compatibility, but are treated as 0.
    ///
    /// Since mpv 0.18.1, this always controls the internal mixer (aka "softvol").
    Volume,
    /// Pause or unpause.
    Pause,
}

impl<'a> From<&'a Property> for Value {
    fn from(property: &'a Property) -> Self {
        let value = match property {
            Property::AudioSpeedCorrection => "audio-speed-correction",
            Property::VideoSpeedCorrection => "video-speed-correction",
            Property::DisplaySyncActive => "display-sync-active",
            Property::Filename => "filename",
            Property::FilenameNoExt => "filename/no-ext",
            Property::FileSize => "file-size",
            Property::EstimatedFrameCount => "estimated-frame-count",
            Property::EstimatedFrameNumber => "estimated-frame-number",
            Property::Path => "path",
            Property::StreamOpenFilename => "stream-open-filename",
            Property::MediaTitle => "media-title",
            Property::FileFormat => "file-format",
            Property::CurrentDemuxer => "current-demuxer",
            Property::StreamPath => "stream-path",
            Property::StreamPos => "stream-pos",
            Property::StreamEnd => "stream-end",
            Property::Duration => "duration",
            Property::PercentPos => "percent-pos",
            Property::TimePos => "time-pos",
            Property::TimeStart => "time-start",
            Property::TimeRemaining => "time-remaining",
            Property::PlaybackTime => "playback-time",
            Property::Seeking => "seeking",
            // Where are these documented?
            Property::Volume => "volume",
            Property::Pause => "pause",
        };
        Value::from(value)
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: Value = self.into();
        write!(f, "{}", value.as_str().unwrap())
    }
}

pub trait TryFromValue: Sized {
    fn try_from(value: Value) -> Result<Self>;
}

impl TryFromValue for Value {
    fn try_from(value: Value) -> Result<Value> {
        Ok(value)
    }
}

impl TryFromValue for bool {
    fn try_from(value: Value) -> Result<bool> {
        match value {
            Value::Bool(value) => Ok(value),
            _ => Err(format!("expected bool, but got: {:?}", value).into()),
        }
    }
}

impl TryFromValue for u64 {
    fn try_from(value: Value) -> Result<u64> {
        value
            .as_u64()
            .ok_or_else(|| format!("expected u64, but got: {:?}", value).into())
    }
}

impl TryFromValue for i64 {
    fn try_from(value: Value) -> Result<i64> {
        value
            .as_i64()
            .ok_or_else(|| format!("expected i64, but got: {:?}", value).into())
    }
}

impl TryFromValue for f64 {
    fn try_from(value: Value) -> Result<f64> {
        value
            .as_f64()
            .ok_or_else(|| format!("expected f64, but got: {:?}", value).into())
    }
}

impl TryFromValue for String {
    fn try_from(value: Value) -> Result<String> {
        match value {
            Value::String(value) => Ok(value),
            _ => Err(format!("expected string, but got: {:?}", value).into()),
        }
    }
}

impl TryFromValue for Vec<Value> {
    fn try_from(value: Value) -> Result<Vec<Value>> {
        match value {
            Value::Array(value) => Ok(value),
            _ => Err(format!("expected array, but got: {:?}", value).into()),
        }
    }
}

impl TryFromValue for Map<String, Value> {
    fn try_from(value: Value) -> Result<Map<String, Value>> {
        match value {
            Value::Object(value) => Ok(value),
            _ => Err(format!("expected object, but got: {:?}", value).into()),
        }
    }
}
