#![allow(nonstandard_style)]

pub(crate) use std::os::raw::{c_int, c_short, c_uint, c_ushort};

/// Helper for mass-creating C style extern types (opaque pointers).
macro_rules! opaque {
    ($( $(#[$attr:meta])* $v:vis $name:ident = $void:ident ),+ $(,)?) => {
        $(
            $(#[$attr])* $v enum $void {}
            $(#[$attr])* $v type $name = *mut $void;
        )+
    };
}

opaque! {
    pub(crate) HBRUSH = HBRUSH__,
    pub(crate) HHOOK = HHOOK__,
    pub(crate) HICON = HICON__,
    pub(crate) HMENU = HMENU__,

    /// Opaque handle to a module in memory.
    pub HINSTANCE = HINSTANCE__,

    /// Opaque handle to a window.
    pub HWND = HWND__,
}

pub(crate) type ATOM = c_ushort;
pub(crate) type BOOL = c_int;
pub(crate) type HCURSOR = HICON;
pub(crate) type wchar_t = u16;

/// A user-defined application window callback procedure.
pub type WNDPROC = unsafe extern "system" fn(HWND, c_uint, usize, isize) -> isize;

#[repr(C)]
pub(crate) struct IMAGE_DOS_HEADER {
    pub(crate) e_magic: u16,
    pub(crate) e_cblp: u16,
    pub(crate) e_cp: u16,
    pub(crate) e_crlc: u16,
    pub(crate) e_cparhdr: u16,
    pub(crate) e_minalloc: u16,
    pub(crate) e_maxalloc: u16,
    pub(crate) e_ss: u16,
    pub(crate) e_sp: u16,
    pub(crate) e_csum: u16,
    pub(crate) e_ip: u16,
    pub(crate) e_cs: u16,
    pub(crate) e_lfarlc: u16,
    pub(crate) e_ovno: u16,
    pub(crate) e_res: [u16; 4],
    pub(crate) e_oemid: u16,
    pub(crate) e_oeminfo: u16,
    pub(crate) e_res2: [u16; 10],
    pub(crate) e_lfanew: i32,
}

#[repr(C)]
pub(crate) struct WNDCLASSEXW {
    pub(crate) cbSize: c_uint,
    pub(crate) style: c_uint,
    pub(crate) lpfnWndProc: WNDPROC,
    pub(crate) cbClsExtra: c_int,
    pub(crate) cbWndExtra: c_int,
    pub(crate) hInstance: HINSTANCE,
    pub(crate) hIcon: HICON,
    pub(crate) hCursor: HCURSOR,
    pub(crate) hbrBackground: HBRUSH,
    pub(crate) lpszMenuName: *const wchar_t,
    pub(crate) lpszClassName: *const wchar_t,
    pub(crate) hIconSm: HICON,
}

#[link(name = "user32")]
extern "system" {
    pub(crate) fn GetClassInfoExW(hinst: HINSTANCE, lpszClass: *const wchar_t, lpwcx: *mut WNDCLASSEXW) -> BOOL;
    pub(crate) fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> ATOM;
}
