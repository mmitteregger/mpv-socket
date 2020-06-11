pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

/// All pipe instances are busy.
pub(crate) const ERROR_PIPE_BUSY: i32 = 231;
/// The pipe is being closed.
pub(crate) const ERROR_NO_DATA: i32 = 232;
