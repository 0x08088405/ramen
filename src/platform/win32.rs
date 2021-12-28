mod ffi;
mod imp;
mod util;

// public re-exports
pub use self::{
    ffi::{HINSTANCE, HWND, WNDPROC},
    imp::window_proc,
    util::base_hinstance,
};

// internals
pub(crate) use imp::Window;
