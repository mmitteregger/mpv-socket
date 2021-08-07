// We have a lot of c-types in here, stop warning about their names!
#![allow(non_camel_case_types)]
// fmt::Debug isn't helpful on FFI types
#![allow(missing_debug_implementations)]

use std::ffi::CStr;
use std::ptr;

use crate::error::mpv_socket_error;

#[macro_use]
mod macros;
pub mod error;

/// cbindgen:ignore
static VERSION_CSTR: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");

ffi_fn! {
    /// Returns a static (null terminated) string of the mpv-socket version.
    fn mpv_socket_version() -> *const libc::c_char {
        VERSION_CSTR.as_ptr() as _
    } ?= std::ptr::null()
}

pub struct mpv_socket(pub(crate) ::mpv_socket::MpvSocket);

#[repr(C)]
pub struct mpv_socket_connect_result {
    pub error: *mut mpv_socket_error,
    pub socket: *mut mpv_socket,
}

impl Default for mpv_socket_connect_result {
    fn default() -> mpv_socket_connect_result {
        mpv_socket_connect_result {
            error: ptr::null_mut(),
            socket: ptr::null_mut(),
        }
    }
}

ffi_fn! {
    fn mpv_socket_connect(path: *const libc::c_char) -> mpv_socket_connect_result {
        let mut result = mpv_socket_connect_result::default();

        let path = match unsafe { CStr::from_ptr(path) }.to_str() {
            Ok(path) => path,
            Err(error) => {
                result.error = error!("invalid path: {}", error);
                return result;
            }
        };

        let socket = try_or_bail!(::mpv_socket::MpvSocket::connect(path), result);

        result.socket = Box::into_raw(Box::new(mpv_socket(socket)));
        return result;
    }
}

ffi_fn! {
    /// Frees a `mpv_socket`.
    fn mpv_socket_free(socket: *mut mpv_socket) {
        drop(unsafe { Box::from_raw(socket) });
    }
}

ffi_fn! {
    fn mpv_socket_observe_property_f64(
        socket: *mut mpv_socket,
        property: *const libc::c_char,
        callback: unsafe extern "C" fn(f64, *mut libc::c_void),
        context: *mut libc::c_void,
    ) -> *mut mpv_socket_error {
        let socket = unsafe { &mut (*socket).0 };
        let property_str = unsafe { CStr::from_ptr(property) }.to_str().unwrap();
        let property = match serde_json::from_str(&format!("\"{}\"", property_str)) {
            Ok(property) => property,
            Err(error) => return error!("invalid property \"{}\": {}", property_str, error),
        };

        let iter = match socket.observe_property(property) {
            Ok(iter) => iter,
            Err(error) => return error!(error),
        };

        for result in iter {
            match result {
                Ok(value) => unsafe { callback(value, context) },
                Err(error) => return error!(error),
            }
        }

        ptr::null_mut()
    }
}
