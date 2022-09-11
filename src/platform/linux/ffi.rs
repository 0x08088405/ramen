#![allow(bad_style)]
#![allow(clippy::too_many_arguments)]

pub(super) use libc::{c_char, c_int, c_uint, c_void, free, getpid};

use libc::{dlsym, dlerror};
unsafe fn dlopen(name: *const c_char) -> *mut c_void {
    libc::dlopen(name, libc::RTLD_LOCAL | libc::RTLD_LAZY)
}

load! {
    pub(super) xlib(libX11) "libX11.so.6", "libX11.so" {
        fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
        fn XDefaultScreen(display: *mut Display) -> c_int;
        fn XCloseDisplay(display: *mut Display) -> c_int;
    }
    pub(super) xlib_xcb(libX11_xcb) "libX11-xcb.so.1", "libX11-xcb.so" {
        fn XGetXCBConnection(dpy: *mut Display) -> *mut xcb_connection_t;
        fn XSetEventQueueOwner(dpy: *mut Display, owner: EventQueueOwner);
    }
    pub(super) xcb(libxcb) "libxcb.so.1", "libxcb.so" {
        //fn xcb_connect(displayname: *const c_char, screenp: *mut c_int) -> *mut xcb_connection_t;
        fn xcb_connection_has_error(c: *mut xcb_connection_t) -> c_int;
        //fn xcb_disconnect(c: *mut xcb_connection_t);
        fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;
        fn xcb_setup_roots_iterator(R: *const xcb_setup_t) -> xcb_screen_iterator_t;
        fn xcb_screen_next(i: *mut xcb_screen_iterator_t);

        fn xcb_flush(c: *mut xcb_connection_t) -> c_int;
        fn xcb_generate_id(c: *mut xcb_connection_t) -> u32;
        fn xcb_request_check(c: *mut xcb_connection_t, sequence: c_uint) -> *mut xcb_generic_error_t;
        fn xcb_create_window_checked(
            c: *mut xcb_connection_t,
            depth: u8,
            wid: xcb_window_t,
            parent: xcb_window_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u16,
            visual: xcb_visualid_t,
            value_mask: u32,
            value_list: *const u32,
        ) -> c_uint;
        fn xcb_change_property(
            c: *mut xcb_connection_t,
            mode: u8,
            window: xcb_window_t,
            property: xcb_atom_t,
            r#type: xcb_atom_t,
            format: u8,
            data_len: u32,
            data: *const c_void,
        ) -> c_uint;
        fn xcb_map_window_checked(c: *mut xcb_connection_t, window: xcb_window_t) -> c_uint;
        fn xcb_intern_atom(
            c: *mut xcb_connection_t,
            only_if_exists: u8,
            name_len: u16,
            name: *const c_char,
        ) -> c_uint;
        fn xcb_intern_atom_reply(
            c: *mut xcb_connection_t,
            sequence: c_uint,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_intern_atom_reply_t;
        fn xcb_poll_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
        fn xcb_poll_for_queued_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
        fn xcb_destroy_window(c: *mut xcb_connection_t, xid: xcb_window_t) -> c_uint;
    }
}

pub(super) enum Display {}
pub(super) enum xcb_setup_t {}
pub(super) enum xcb_connection_t {}

#[repr(C)]
#[allow(dead_code)]
pub(super) enum EventQueueOwner { XlibOwnsEventQueue = 0, XCBOwnsEventQueue }
pub(super) type xcb_atom_t = u32;
pub(super) type xcb_colormap_t = u32;
pub(super) type xcb_visualid_t = u32;
pub(super) type xcb_window_t = u32;

pub(super) const XCB_WINDOW_CLASS_INPUT_OUTPUT: u16 = 1;
pub(super) const XCB_COPY_FROM_PARENT: u8 = 0;
// pub(super) const XCB_KEY_PRESS: u8 = 2;
// pub(super) const XCB_KEY_RELEASE: u8 = 3;
// pub(super) const XCB_BUTTON_PRESS: u8 = 4;
// pub(super) const XCB_BUTTON_RELEASE: u8 = 5;
pub(super) const XCB_FOCUS_IN: u8 = 9;
pub(super) const XCB_FOCUS_OUT: u8 = 10;
pub(super) const XCB_CLIENT_MESSAGE: u8 = 33;

pub(super) const XCB_PROP_MODE_REPLACE: u8 = 0;
//pub(super) const XCB_PROP_MODE_APPEND: u8 = 1;
//pub(super) const XCB_PROP_MODE_PREPEND: u8 = 2;

//pub(super) const XCB_ATOM_NONE: xcb_atom_t = 0;
pub(super) const XCB_ATOM_ATOM: xcb_atom_t = 4;
pub(super) const XCB_ATOM_CARDINAL: xcb_atom_t = 6;
pub(super) const XCB_ATOM_STRING: xcb_atom_t = 31;
pub(super) const XCB_ATOM_WM_NAME: xcb_atom_t = 39;

pub(super) const XCB_CW_EVENT_MASK: u32 = 2048;
pub(super) const XCB_EVENT_MASK_KEY_PRESS: u32 = 1;
pub(super) const XCB_EVENT_MASK_KEY_RELEASE: u32 = 2;
pub(super) const XCB_EVENT_MASK_BUTTON_PRESS: u32 = 4;
pub(super) const XCB_EVENT_MASK_BUTTON_RELEASE: u32 = 8;
pub(super) const XCB_EVENT_MASK_FOCUS_CHANGE: u32 = 2097152;

pub(super) const XCB_NONE: c_int = 0;
pub(super) const XCB_ALLOC: c_int = 11;
pub(super) const XCB_CONN_CLOSED_EXT_NOTSUPPORTED: c_int = 2;
pub(super) const XCB_CONN_CLOSED_MEM_INSUFFICIENT: c_int = 3;

#[repr(C)]
pub(super) struct xcb_generic_error_t {
    pub(super) response_type: u8,
    pub(super) error_code: u8,
    pub(super) sequence: u16,
    pub(super) resource_id: u32,
    pub(super) minor_code: u16,
    pub(super) major_code: u8,
    pub(super) _pad0: u8,
    pub(super) _pad: [u32; 5],
    pub(super) full_sequence: u32,
}

#[repr(C)]
pub(super) struct xcb_screen_iterator_t {
    pub(super) data: *mut xcb_screen_t,
    pub(super) rem: c_int,
    pub(super) index: c_int,
}

#[repr(C)]
pub(super) struct xcb_screen_t {
    pub(super) root: xcb_window_t,
    pub(super) default_colourmap: xcb_colormap_t,
    pub(super) white_pixel: u32,
    pub(super) black_pixel: u32,
    pub(super) current_input_masks: u32,
    pub(super) width_in_pixels: u16,
    pub(super) height_in_pixels: u16,
    pub(super) width_in_millimeters: u16,
    pub(super) height_in_millimeters: u16,
    pub(super) min_installed_maps: u16,
    pub(super) max_installed_maps: u16,
    pub(super) root_visual: xcb_visualid_t,
    pub(super) backing_stores: u8,
    pub(super) save_unders: u8,
    pub(super) root_depth: u8,
    pub(super) allowed_depths_len: u8,
}

#[repr(C)]
pub(super) struct xcb_intern_atom_reply_t {
    pub(super) response_type: u8,
    pub(super) pad0: u8,
    pub(super) sequence: u16,
    pub(super) length: u32,
    pub(super) atom: xcb_atom_t,
}

#[repr(C)]
pub(super) struct xcb_generic_event_t {
    pub(super) response_type: u8,
    pub(super) _pad0: u8,
    pub(super) sequence: u16,
    pub(super) _pad: [u32; 7],
    pub(super) full_sequence: u32,
}

#[repr(C)]
pub(super) struct xcb_client_message_event_t {
    pub(super) response_type: u8,
    pub(super) format: u8,
    pub(super) sequence: u16,
    pub(super) window: xcb_window_t,
    pub(super) r#type: xcb_atom_t,
    pub(super) client_data: ClientData,
}

#[repr(C)]
pub(crate) union ClientData {
    pub(crate) data8: [u8; 20],
    pub(crate) data16: [u16; 10],
    pub(crate) data32: [u32; 5],
}

#[repr(C)]
pub(super) struct xcb_focus_in_event_t {
    pub(super) response_type: u8,
    pub(super) send_event: u8,
    pub(super) sequence: u16,
    pub(super) event: xcb_window_t,
    pub(super) mode: u8,
    pub(super) _pad0: [u8; 3],
}
