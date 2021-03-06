/// Simple opaque error type for this library.
///
/// The real cause can be downcasted if necessary.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// Type alias for `Result<T, mpv_socket::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// All pipe instances are busy.
#[cfg(target_os = "windows")]
pub(crate) const ERROR_PIPE_BUSY: i32 = 231;
/// The pipe is being closed.
#[cfg(target_os = "windows")]
pub(crate) const ERROR_NO_DATA: i32 = 232;
