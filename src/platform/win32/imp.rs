use crate::window;
use super::ffi::*;

pub(crate) struct Window;

impl Window {
    pub(crate) fn new(builder: &window::WindowBuilder) -> Result<Self, ()> {
        todo!()
    }
}

pub unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: c_uint,
    wparam: usize,
    lparam: isize,
) -> isize {
    0
}
