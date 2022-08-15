mod event;
pub(super) use event::Event;

use std::{ffi, mem::transmute, os::raw, ptr};

#[derive(Debug)]
pub(super) struct Error(raw::c_int);

const XCB_WINDOW_CLASS_INPUT_OUTPUT: u16 = 1;

pub(super) type XcbAtom = u32;
pub(super) type XcbColourMap = u32;
pub(super) type XcbVisualId = u32;
pub(super) type XcbWindow = u32;

pub(super) const XCB_PROP_MODE_REPLACE: u8 = 0;
//pub(super) const XCB_PROP_MODE_APPEND: u8 = 1;
//pub(super) const XCB_PROP_MODE_PREPEND: u8 = 2;

pub(super) const XCB_ATOM_NONE: XcbAtom = 0;
pub(super) const XCB_ATOM_ATOM: XcbAtom = 4;
pub(super) const XCB_ATOM_CARDINAL: XcbAtom = 6;
pub(super) const XCB_ATOM_STRING: XcbAtom = 31;
pub(super) const XCB_ATOM_WM_NAME: XcbAtom = 39;

pub(super) const XCB_CW_BACK_PIXEL: u32 = 2;
pub(super) const XCB_CW_EVENT_MASK: u32 = 2048;
pub(super) const XCB_EVENT_MASK_KEY_PRESS: u32 = 1;
pub(super) const XCB_EVENT_MASK_KEY_RELEASE: u32 = 2;
pub(super) const XCB_EVENT_MASK_BUTTON_PRESS: u32 = 4;
pub(super) const XCB_EVENT_MASK_BUTTON_RELEASE: u32 = 8;

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
    pub(super) atom_wm_protocols: XcbAtom,
    pub(super) atom_wm_delete_window: XcbAtom,
    pub(super) atom_net_wm_name: XcbAtom,
    pub(super) atom_net_wm_pid: XcbAtom,
    pub(super) atom_utf8_string: XcbAtom,
    request_check: unsafe extern "C" fn(*mut ConnectionPtr, Cookie) -> *mut XcbGenericError,
    connection_has_error: unsafe extern "C" fn(*mut ConnectionPtr) -> raw::c_int,
    disconnect: unsafe extern "C" fn(*mut ConnectionPtr),
    flush: unsafe extern "C" fn(*mut ConnectionPtr) -> raw::c_int,
    generate_id: unsafe extern "C" fn(*mut ConnectionPtr) -> u32,
    create_window: unsafe extern "C" fn(*mut ConnectionPtr, u8, XcbWindow, XcbWindow, i16, i16, u16, u16, u16, u16, XcbVisualId, u32, *const ffi::c_void) -> Cookie,
    map_window: unsafe extern "C" fn(*mut ConnectionPtr, XcbWindow) -> Cookie,
    destroy_window: unsafe extern "C" fn(*mut ConnectionPtr, XcbWindow) -> Cookie,
    discard_reply: unsafe extern "C" fn(*mut ConnectionPtr, raw::c_uint),
    poll_for_event: unsafe extern "C" fn(*mut ConnectionPtr) -> *mut event::XcbGenericEvent,
    poll_for_queued_event: unsafe extern "C" fn(*mut ConnectionPtr) -> *mut event::XcbGenericEvent,
    _intern_atom: unsafe extern "C" fn(*mut ConnectionPtr, u8, u16, *const raw::c_char) -> Cookie,
    _intern_atom_reply: unsafe extern "C" fn(*mut ConnectionPtr, Cookie, *mut *mut XcbGenericError) -> *mut event::XcbAtomReply,
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
            atom_wm_protocols: 0,
            atom_wm_delete_window: 0,
            atom_net_wm_name: 0,
            atom_net_wm_pid: 0,
            atom_utf8_string: 0,
            request_check: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            connection_has_error: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            disconnect: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            flush: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            generate_id: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            create_window: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            map_window: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            destroy_window: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            discard_reply: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            poll_for_event: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            poll_for_queued_event: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            _intern_atom: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
            _intern_atom_reply: unsafe { transmute(do_not_call as unsafe extern "C" fn() -> !) },
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

    /// Calls `xcb_destroy_window` on the given window. The given Window will no longer exist and its handle
    /// will be invalid after calling this function.
    pub(super) fn destroy_window(&self, window: XcbWindow) -> Result<(), Error> {
        unsafe {
            let cookie = (self.destroy_window)(self.connection, window);
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

    /// Calls `xcb_change_property`. See its manpage for more information on this function.
    pub(super) fn change_property(&self, mode: u8, window: XcbWindow, property: XcbAtom, prop_type: XcbAtom, format: u8, data_elements: u32, data: *const ffi::c_void) {
        unsafe {
            let cookie = (self.change_property)(self.connection, mode, window, property, prop_type, format, data_elements, data);
            (self.discard_reply)(self.connection, cookie.seq);
        }
    }

    /// Calls `xcb_poll_for_event`. Returns the next event in the queue, if any.
    /// 
    /// This functions polls the connection if there are no more queued events, whereas `poll_queued_event()` does not.
    /// 
    /// This function will return None either if there are no queued events, or the first queued event is not relevant to
    /// the application. So, if trying to get all queued events, `poll_queued_event()` should still be called in a
    /// loop after this even if this returns None.
    pub(super) fn poll_event(&self) -> Option<Event> {
        let ev = unsafe { (self.poll_for_event)(self.connection) };
        if ev.is_null() {
            None
        } else {
            let ret = Event::from_generic(ev);
            unsafe { libc::free(ev.cast()); }
            ret            
        }
    }

    /// Calls `xcb_poll_for_queued_event`. Returns the next event in XCB's queue, if any, without checking for any
    /// new events since XCB's queue was last populated.
    /// 
    /// `poll_event` should first be used to populate the queue. This function returning None means the queue is empty.
    pub(super) fn poll_queued_event(&self) -> Option<Event> {
        loop {
            let ev = unsafe { (self.poll_for_queued_event)(self.connection) };
            if ev.is_null() {
                break None
            } else if let Some(e) = Event::from_generic(ev) {
                unsafe { libc::free(ev.cast()); }
                break Some(e)
            }
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

unsafe fn setup() -> Option<Xcb> {
    macro_rules! load_fn {
        ($name:literal) => {{
            let request_check = libc::dlsym(LIBXCB.0, c_string!($name));
            if request_check.is_null() { None } else { Some(transmute(request_check)) }
        }}
    }

    // Check validity of our connection to libxcb.so and existence of functions we actually need here
    if !LIBXCB.is_valid() { return None }
    enum XcbSetup {}
    let xcb_connect: unsafe extern "C" fn(*const raw::c_char, *mut raw::c_int) -> *mut ConnectionPtr = load_fn!("xcb_connect")?;
    let xcb_connection_has_error: unsafe extern "C" fn(*mut ConnectionPtr) -> raw::c_int = load_fn!("xcb_connection_has_error")?;
    let xcb_get_setup: unsafe extern "C" fn(*mut ConnectionPtr) -> *mut XcbSetup = load_fn!("xcb_get_setup")?;
    let xcb_setup_roots_iterator: unsafe extern "C" fn(*const XcbSetup) -> ScreenIterator = load_fn!("xcb_setup_roots_iterator")?;
    let xcb_setup_roots_length: unsafe extern "C" fn(*const XcbSetup) -> raw::c_int = load_fn!("xcb_setup_roots_length")?;

    // Create an XCB connection
    let connection = xcb_connect(ptr::null(), ptr::null_mut());
    let err = xcb_connection_has_error(connection);
    if err > 0 { return None }

    // Iterate screens
    let setup = xcb_get_setup(connection);
    let length = xcb_setup_roots_length(setup);
    if length <= 0 { return None }
    let iter: ScreenIterator = xcb_setup_roots_iterator(setup);
    let screen = iter.data;
    if screen.is_null() { return None }

    // Define other functions we'll need
    let request_check = load_fn!("xcb_request_check")?;
    let disconnect = load_fn!("xcb_disconnect")?;
    let flush = load_fn!("xcb_flush")?;
    let generate_id = load_fn!("xcb_generate_id")?;
    let create_window = load_fn!("xcb_create_window_checked")?;
    let map_window = load_fn!("xcb_map_window_checked")?;
    let destroy_window = load_fn!("xcb_destroy_window")?;
    let discard_reply = load_fn!("xcb_discard_reply")?;
    let poll_for_event = load_fn!("xcb_poll_for_event")?;
    let poll_for_queued_event = load_fn!("xcb_poll_for_queued_event")?;
    let intern_atom = load_fn!("xcb_intern_atom")?;
    let intern_atom_reply = load_fn!("xcb_intern_atom_reply")?;
    let change_property = load_fn!("xcb_change_property_checked")?;

    // And some non-standard atom values...
    let atom_wm_protocols = intern_atom_internal(connection, intern_atom, intern_atom_reply, "WM_PROTOCOLS")?;
    let atom_wm_delete_window = intern_atom_internal(connection, intern_atom, intern_atom_reply, "WM_DELETE_WINDOW")?;
    let atom_net_wm_name = intern_atom_internal(connection, intern_atom, intern_atom_reply, "_NET_WM_NAME")?;
    let atom_net_wm_pid = intern_atom_internal(connection, intern_atom, intern_atom_reply, "_NET_WM_PID")?;
    let atom_utf8_string = intern_atom_internal(connection, intern_atom, intern_atom_reply, "UTF8_STRING")?;

    Some(Xcb {
        connection,
        screen,
        atom_wm_protocols,
        atom_wm_delete_window,
        atom_net_wm_name,
        atom_net_wm_pid,
        atom_utf8_string,
        request_check,
        connection_has_error: xcb_connection_has_error,
        disconnect,
        flush,
        generate_id,
        create_window,
        map_window,
        destroy_window,
        discard_reply,
        poll_for_event,
        poll_for_queued_event,
        _intern_atom: intern_atom,
        _intern_atom_reply: intern_atom_reply,
        change_property,
    })
}

/// Helper fn for calling intern_atom before Xcb has been constructed... (Can be wrapped by other functions of `Xcb`
/// if we need to do that in future)
/// Note: this will always set `only_if_exists` to false; I can't imagine we'll ever have a use-case for setting it to true.
fn intern_atom_internal(
    connection: *mut ConnectionPtr,
    intern_atom: unsafe extern "C" fn(*mut ConnectionPtr, u8, u16, *const raw::c_char) -> Cookie,
    intern_atom_reply: unsafe extern "C" fn(*mut ConnectionPtr, Cookie, *mut *mut XcbGenericError) -> *mut event::XcbAtomReply,
    name: &str,
) -> Option<XcbAtom> {
    unsafe {
        let cookie = (intern_atom)(connection, false.into(), name.bytes().len() as _, name.as_ptr().cast());
        let mut err: *mut XcbGenericError = ptr::null_mut();
        let reply = (intern_atom_reply)(connection, cookie, (&mut err) as _);
        let atom = if reply.is_null() { None } else { if (*reply).atom == XCB_ATOM_NONE { None } else { Some((*reply).atom) } };
        libc::free(reply as _);
        libc::free(err as _);
        atom
    }
}

lazy_static::lazy_static! {
    static ref LIBXCB: LibXcb = LibXcb(unsafe { libc::dlopen(c_string!("libxcb.so.1"), libc::RTLD_LOCAL | libc::RTLD_LAZY) });
    pub(super) static ref XCB: Xcb = unsafe { setup().unwrap_or_else(Xcb::invalid) };
}
