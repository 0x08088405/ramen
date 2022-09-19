mod ffi;
mod imp;

pub use self::ffi::xcb_window_t;

pub(crate) use imp::{Connection, Window};
