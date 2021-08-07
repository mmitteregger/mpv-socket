#[repr(C)]
pub enum mpv_socket_code {
    /// All is well.
    MPV_SOCKET_E_OK,
    /// General error, details in the `mpv_socket_error *`.
    MPV_SOCKET_E_ERROR,
}

pub struct mpv_socket_error(pub(crate) Box<dyn std::error::Error + Send + Sync + 'static>);

impl mpv_socket_error {
    fn print_to(&self, dst: &mut [u8]) -> usize {
        use std::io::Write;

        let mut dst = std::io::Cursor::new(dst);

        // A write! error doesn't matter. As much as possible will have been
        // written, and the Cursor position will know how far that is (even
        // if that is zero).
        let _ = write!(dst, "{}", self.0);
        dst.position() as usize
    }
}

ffi_fn! {
    /// Frees a `mpv_socket_error`.
    fn mpv_socket_error_free(err: *mut mpv_socket_error) {
        drop(unsafe { Box::from_raw(err) });
    }
}

ffi_fn! {
    /// Print the details of this error to a buffer.
    ///
    /// The `dst_len` value must be the maximum length that the buffer can
    /// store.
    ///
    /// The return value is number of bytes that were written to `dst`.
    fn mpv_socket_error_print(err: *const mpv_socket_error, dst: *mut u8, dst_len: libc::size_t) -> libc::size_t {
        let dst = unsafe {
            std::slice::from_raw_parts_mut(dst, dst_len)
        };
        unsafe { &*err }.print_to(dst)
    }
}
