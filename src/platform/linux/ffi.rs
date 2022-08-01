use crate::event::Event;
use std::{ffi, mem::transmute, os::raw, ptr};

pub(super) struct Error(raw::c_int);

const XCB_WINDOW_CLASS_INPUT_OUTPUT: u16 = 1;

const XCB_KEY_PRESS: u8 = 2;
const XCB_KEY_RELEASE: u8 = 3;
const XCB_BUTTON_PRESS: u8 = 4;
const XCB_BUTTON_RELEASE: u8 = 5;

pub(super) type XcbAtom = u32;
pub(super) type XcbColourMap = u32;
pub(super) type XcbVisualId = u32;
pub(super) type XcbWindow = u32;

pub(super) const XCB_PROP_MODE_REPLACE: u8 = 0;
pub(super) const XCB_PROP_MODE_APPEND: u8 = 1;
//pub(super) const XCB_PROP_MODE_PREPEND: u8 = 2;

pub(super) const XCB_ATOM_NONE: XcbAtom = 0;
pub(super) const XCB_ATOM_ATOM: XcbAtom = 4;

pub(super) const XCB_CW_BACK_PIXEL: u32 = 2;
pub(super) const XCB_CW_EVENT_MASK: u32 = 2048;
pub(super) const XCB_EVENT_MASK_KEY_PRESS: u32 = 1;
pub(super) const XCB_EVENT_MASK_KEY_RELEASE: u32 = 2;
pub(super) const XCB_EVENT_MASK_BUTTON_PRESS: u32 = 4;
pub(super) const XCB_EVENT_MASK_BUTTON_RELEASE: u32 = 8;

#[repr(C)]
struct XcbGenericEvent {
    response_type: u8,
    _pad0: u8,
    sequence: u16,
    _pad: [u32; 7],
    full_sequence: u32,
}

#[repr(C)]
struct XcbGenericError {
    response_type: u8,
    error_code: u8,
    sequence: u16,
    resource_id: u32,
    minor_code: u16,
    major_code: u8,
    _pad0: u8,
    _pad: [u32; 5],
    full_sequence: u32,
}

#[repr(C)]
struct XcbAtomReply {
    response_type: u8,
    _pad0: u8,
    sequence: u16,
    length: u32,
    atom: XcbAtom,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct Cookie {
    seq: raw::c_uint,
}

/// Helps you create C-compatible string literals, like `c_string!("Hello!")` -> `b"Hello!\0"`.
macro_rules! c_string {
    ($s:expr) => {
        concat!($s, "\0").as_ptr().cast()
    };
}

/// Calls dlerror, returning the error string or None if there's no error
pub(super) fn dl_error() -> Option<&'static str> {
    unsafe {
        let start = libc::dlerror() as *mut u8;
        if start.is_null() {
            None
        } else {
        let mut count = 0;
        while *start.add(count) != 0 {
            count += 1;
        }
        Some(std::str::from_utf8_unchecked(std::slice::from_raw_parts(start, count)))
        }
    }
}

// Dummy function which will be pointed to by an invalid Xcb struct
unsafe extern "C" fn do_not_call() -> ! {
    panic!("XCB function was called on invalid Xcb struct");
}

/// Referent type for xcb_connection_t
enum ConnectionPtr {}

/// XCB connection wrapper
pub(super) struct Xcb {
    connection: *mut ConnectionPtr,
    screen: *mut Screen,
    request_check: unsafe extern "C" fn(*mut ConnectionPtr, Cookie) -> *mut XcbGenericError,
    connection_has_error: unsafe extern "C" fn(*mut ConnectionPtr) -> raw::c_int,
    disconnect: unsafe extern "C" fn(*mut ConnectionPtr),
    flush: unsafe extern "C" fn(*mut ConnectionPtr) -> raw::c_int,
    generate_id: unsafe extern "C" fn(*mut ConnectionPtr) -> u32,
    create_window: unsafe extern "C" fn(*mut ConnectionPtr, u8, XcbWindow, XcbWindow, i16, i16, u16, u16, u16, u16, XcbVisualId, u32, *const ffi::c_void) -> Cookie,
    map_window: unsafe extern "C" fn(*mut ConnectionPtr, XcbWindow) -> Cookie,
    discard_reply: unsafe extern "C" fn(*mut ConnectionPtr, raw::c_uint),
    poll_for_event: unsafe extern "C" fn(*mut ConnectionPtr) -> *mut XcbGenericEvent,
    poll_for_queued_event: unsafe extern "C" fn(*mut ConnectionPtr) -> *mut XcbGenericEvent,
    intern_atom: unsafe extern "C" fn(*mut ConnectionPtr, u8, u16, *const raw::c_char) -> Cookie,
    intern_atom_reply: unsafe extern "C" fn(*mut ConnectionPtr, Cookie, *mut *mut XcbGenericError) -> *mut XcbAtomReply,
    change_property: unsafe extern "C" fn(*mut ConnectionPtr, u8, XcbWindow, XcbAtom, XcbAtom, u8, u32, *const ffi::c_void) -> Cookie,
}
unsafe impl Send for Xcb {}
unsafe impl Sync for Xcb {}
impl Drop for Xcb {
    fn drop(&mut self) {
        // Note: "If `c` is `NULL`, nothing is done" - an XCB header
        unsafe { (self.disconnect)(self.connection) };
    }
}
impl Xcb {
    /// If there's a problem during setup, this function will be called to create an Xcb in an invalid state.
    fn invalid() -> Self {
        Self {
            connection: ptr::null_mut(),
            screen: ptr::null_mut(),
            request_check: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            connection_has_error: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            disconnect: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            flush: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            generate_id: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            create_window: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            map_window: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            discard_reply: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            poll_for_event: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            poll_for_queued_event: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            intern_atom: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            intern_atom_reply: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            change_property: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
        }
    }

    /// Checks if the connection is valid. An invalid connection usually means setup has not been successful,
    /// but may also mean the connection has shut down due to a fatal error. Further function calls to a
    /// connection in this state will have no effect.
    /// 
    /// See manual page on `xcb_connection_has_error` for more information.
    pub(super) fn is_valid(&self) -> bool {
        !self.connection.is_null() && unsafe { (self.connection_has_error)(self.connection) } <= 0
    }

    /// Returns the screen's white pixel value on this particular system.
    pub(super) fn white_pixel(&self) -> u32 {
        unsafe { (*self.screen).white_pixel }
    }

    /// Calls `xcb_flush`. This should generally be done at the end of any function in imp.rs, or in any other
    /// situation where a function that was just called needs to be fully completed before moving on.
    pub(super) fn flush(&self) -> Result<(), Error> {
        unsafe {
            let r = (self.flush)(self.connection);
            if r > 0 { Ok(()) } else { Err(Error(r)) }
        }
    }

    /// Calls `xcb_generate_id`. Generating an ID is required to create anything which needs an ID, such as a window.
    pub(super) fn generate_id(&self) -> u32 {
        unsafe { (self.generate_id)(self.connection) }
    }

    /// Calls `xcb_create_window_checked` with the given parameters.
    pub(super) fn create_window(&self, id: XcbWindow, x: i16, y: i16, width: u16, height: u16, border_width: u16, value_mask: u32, value_list: &[u32]) -> Result<(), Error> {
        unsafe {
            let cookie = (self.create_window)(self.connection, 0, id, (*self.screen).root, x, y, width, height, border_width, XCB_WINDOW_CLASS_INPUT_OUTPUT, 0, value_mask, value_list.as_ptr().cast());
            let r = (self.request_check)(self.connection, cookie);
            if r.is_null() {
                Ok(())
            } else {
                let e = Error((*r).error_code.into());
                (self.discard_reply)(self.connection, cookie.seq);
                Err(e)
            }
        }
    }

    /// Calls `xcb_map_window_checked` on the given window.
    pub(super) fn map_window(&self, window: XcbWindow) -> Result<(), Error> {
        unsafe {
            let cookie = (self.map_window)(self.connection, window);
            let r = (self.request_check)(self.connection, cookie);
            if r.is_null() {
                Ok(())
            } else {
                let e = Error((*r).error_code.into());
                (self.discard_reply)(self.connection, cookie.seq);
                Err(e)
            }
        }
    }

    /// Calls `xcb_intern_atom`. Results will be unexpected if `name` is longer than 65535 bytes.
    pub(super) fn intern_atom(&self, only_if_exists: bool, name: &str) -> XcbAtom {
        unsafe {
            // TODO: how to check this error correctly?
            let cookie = (self.intern_atom)(self.connection, only_if_exists.into(), name.bytes().len() as _, name.as_ptr().cast());
            let mut err: *mut XcbGenericError = ptr::null_mut();
            let reply = (self.intern_atom_reply)(self.connection, cookie, (&mut err) as _);
            let atom = (*reply).atom;
            (self.discard_reply)(self.connection, cookie.seq);
            atom
        }
    }

    /// Calls `xcb_change_property`. See its manpage for more information on this function.
    pub(super) fn change_property(&self, mode: u8, window: XcbWindow, property: XcbAtom, prop_type: XcbAtom, format: u8, data_elements: u32, data: *const ffi::c_void) {
        unsafe {
            let cookie = (self.change_property)(self.connection, mode, window, property, prop_type, format, data_elements, data);
            (self.discard_reply)(self.connection, cookie.seq);
        }
    }

    /// Calls `xcb_poll_for_event`. Returns the next event in the queue, if any.
    pub(super) fn poll_event(&self) -> Option<Event> {
        Self::process_event(unsafe { (self.poll_for_event)(self.connection) })
    }

    /// Calls `xcb_poll_for_queued_event`. Returns the next event in the queue, if any.
    pub(super) fn poll_queued_event(&self) -> Option<Event> {
        Self::process_event(unsafe { (self.poll_for_queued_event)(self.connection) })
    }

    /// Converts an XcbGenericEvent to a ramen Event
    fn process_event(event: *mut XcbGenericEvent) -> Option<Event> {
        if event.is_null() {
            None
        } else {
            let ret = unsafe {
                match (*event).response_type & !0x80 {
                    XCB_KEY_PRESS => {
                        let _event: *mut XcbButtonEvent = event.cast();
                        None
                    },
                    _ => None,
                }
            };
            unsafe { libc::free(event.cast()); }
            ret
        }
    }
}

/// Pointer to dynamically-loaded libxcb.so
struct LibXcb (*mut ffi::c_void);
impl LibXcb {
    fn is_valid(&self) -> bool {
        !self.0.is_null()
    }
}
impl Drop for LibXcb {
    fn drop(&mut self) {
        unsafe { let _ = libc::dlclose(self.0); }
    }
}
unsafe impl Send for LibXcb {}
unsafe impl Sync for LibXcb {}

#[repr(C)]
#[derive(Debug)]
struct ScreenIterator {
    data: *mut Screen,
    rem: raw::c_int,
    index: raw::c_int,
}

#[repr(C)]
#[derive(Debug)]
struct Screen {
    root: XcbWindow,
    default_colourmap: XcbColourMap,
    white_pixel: u32,
    black_pixel: u32,
    current_input_masks: u32,
    width_in_pixels: u16,
    height_in_pixels: u16,
    width_in_millimeters: u16,
    height_in_millimeters: u16,
    min_installed_maps: u16,
    max_installed_maps: u16,
    root_visual: XcbVisualId,
    backing_stores: u8,
    save_unders: u8,
    root_depth: u8,
    allowed_depths_len: u8,
}

#[repr(C)]
#[derive(Debug)]
struct XcbButtonEvent {
    response_type: u8,
    detail: u8,
    sequence: u16,
    time: u32,
    root: XcbWindow,
    event: XcbWindow,
    child: XcbWindow,
    root_x: i16,
    root_y: i16,
    event_x: i16,
    event_y: i16,
    state: u16,
    same_screen: u8,
    _pad: u8,
}

unsafe fn setup() -> Xcb {
    // Check validity of our connection to libxcb.so and existence of functions we actually need here
    if !LIBXCB.is_valid() { return Xcb::invalid() }
    let xcb_connect = libc::dlsym(LIBXCB.0, c_string!("xcb_connect"));
    if xcb_connect.is_null() { return Xcb::invalid() }
    let xcb_connection_has_error = libc::dlsym(LIBXCB.0, c_string!("xcb_connection_has_error"));
    if xcb_connection_has_error.is_null() { return Xcb::invalid() }
    let xcb_get_setup = libc::dlsym(LIBXCB.0, c_string!("xcb_get_setup"));
    if xcb_get_setup.is_null() { return Xcb::invalid() }
    let xcb_setup_roots_iterator = libc::dlsym(LIBXCB.0, c_string!("xcb_setup_roots_iterator"));
    if xcb_setup_roots_iterator.is_null() { return Xcb::invalid() }
    let xcb_setup_roots_length = libc::dlsym(LIBXCB.0, c_string!("xcb_setup_roots_length"));
    if xcb_setup_roots_length.is_null() { return Xcb::invalid() }

    // Create an XCB connection
    let xcb_connect: unsafe extern "C" fn(*const raw::c_char, *mut raw::c_int) -> *mut ConnectionPtr = transmute(xcb_connect);
    let xcb_connection_has_error: unsafe extern "C" fn(*mut ConnectionPtr) -> raw::c_int = transmute(xcb_connection_has_error);
    let connection = xcb_connect(ptr::null(), ptr::null_mut());

    // Iterate screens
    enum SetupPtr {}
    let xcb_get_setup: unsafe extern "C" fn(*mut ConnectionPtr) -> *mut SetupPtr = transmute(xcb_get_setup);
    let xcb_setup_roots_iterator: unsafe extern "C" fn(*const SetupPtr) -> ScreenIterator = transmute(xcb_setup_roots_iterator);
    let xcb_setup_roots_length: unsafe extern "C" fn(*const SetupPtr) -> raw::c_int = transmute(xcb_setup_roots_length);
    let setup = xcb_get_setup(connection);
    let length = xcb_setup_roots_length(setup);
    if length <= 0 { return Xcb::invalid() }
    let iter: ScreenIterator = xcb_setup_roots_iterator(setup);
    let screen = iter.data;
    if screen.is_null() { return Xcb::invalid() }

    // Define other functions we'll need
    let request_check = libc::dlsym(LIBXCB.0, c_string!("xcb_request_check"));
    if request_check.is_null() { return Xcb::invalid() }
    let request_check: unsafe extern "C" fn(*mut ConnectionPtr, Cookie) -> *mut XcbGenericError = transmute(request_check);
    let disconnect = libc::dlsym(LIBXCB.0, c_string!("xcb_disconnect"));
    if disconnect.is_null() { return Xcb::invalid() }
    let disconnect: unsafe extern "C" fn(*mut ConnectionPtr) = transmute(disconnect);
    let flush = libc::dlsym(LIBXCB.0, c_string!("xcb_flush"));
    if flush.is_null() { return Xcb::invalid() }
    let flush: unsafe extern "C" fn(*mut ConnectionPtr) -> raw::c_int = transmute(flush);
    let generate_id = libc::dlsym(LIBXCB.0, c_string!("xcb_generate_id"));
    if generate_id.is_null() { return Xcb::invalid() }
    let generate_id: unsafe extern "C" fn(*mut ConnectionPtr) -> u32 = transmute(generate_id);
    let create_window = libc::dlsym(LIBXCB.0, c_string!("xcb_create_window_checked"));
    if create_window.is_null() { return Xcb::invalid() }
    let create_window: unsafe extern "C" fn(*mut ConnectionPtr, u8, XcbWindow, XcbWindow, i16, i16, u16, u16, u16, u16, XcbVisualId, u32, *const ffi::c_void) -> Cookie = transmute(create_window);
    let map_window = libc::dlsym(LIBXCB.0, c_string!("xcb_map_window_checked"));
    if map_window.is_null() { return Xcb::invalid() }
    let map_window: unsafe extern "C" fn(*mut ConnectionPtr, XcbWindow) -> Cookie = transmute(map_window);
    let discard_reply = libc::dlsym(LIBXCB.0, c_string!("xcb_discard_reply"));
    if discard_reply.is_null() { return Xcb::invalid() }
    let discard_reply: unsafe extern "C" fn(*mut ConnectionPtr, raw::c_uint) = transmute(discard_reply);
    let poll_for_event = libc::dlsym(LIBXCB.0, c_string!("xcb_poll_for_event"));
    if poll_for_event.is_null() { return Xcb::invalid() }
    let poll_for_event: unsafe extern "C" fn(*mut ConnectionPtr) -> *mut XcbGenericEvent = transmute(poll_for_event);
    let poll_for_queued_event = libc::dlsym(LIBXCB.0, c_string!("xcb_poll_for_queued_event"));
    if poll_for_queued_event.is_null() { return Xcb::invalid() }
    let poll_for_queued_event: unsafe extern "C" fn(*mut ConnectionPtr) -> *mut XcbGenericEvent = transmute(poll_for_queued_event);
    let intern_atom = libc::dlsym(LIBXCB.0, c_string!("xcb_intern_atom"));
    if intern_atom.is_null() { return Xcb::invalid() }
    let intern_atom: unsafe extern "C" fn(*mut ConnectionPtr, u8, u16, *const raw::c_char) -> Cookie = transmute(intern_atom);
    let intern_atom_reply = libc::dlsym(LIBXCB.0, c_string!("xcb_intern_atom_reply"));
    if intern_atom_reply.is_null() { return Xcb::invalid() }
    let intern_atom_reply: unsafe extern "C" fn(*mut ConnectionPtr, Cookie, *mut *mut XcbGenericError) -> *mut XcbAtomReply = transmute(intern_atom_reply);
    let change_property = libc::dlsym(LIBXCB.0, c_string!("xcb_change_property_checked"));
    if change_property.is_null() { return Xcb::invalid() }
    let change_property: unsafe extern "C" fn(*mut ConnectionPtr, u8, XcbWindow, XcbAtom, XcbAtom, u8, u32, *const ffi::c_void) -> Cookie = transmute(change_property);

    let err = xcb_connection_has_error(connection);
    if  err <= 0 {
        Xcb {
            connection,
            screen,
            request_check,
            connection_has_error: xcb_connection_has_error,
            disconnect,
            flush,
            generate_id,
            create_window,
            map_window,
            discard_reply,
            poll_for_event,
            poll_for_queued_event,
            intern_atom,
            intern_atom_reply,
            change_property,
        }
    } else {
        Xcb::invalid()
    }
}

lazy_static::lazy_static! {
    static ref LIBXCB: LibXcb = LibXcb(unsafe { libc::dlopen(c_string!("libxcb.so.1"), libc::RTLD_LOCAL | libc::RTLD_LAZY) });
    pub(super) static ref XCB: Xcb = unsafe { setup() };
}
