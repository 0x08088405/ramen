// TODO: I suppose we'll need some method of deciding at runtime whether to use x11 or wayland? This is just x11
use crate::{error::Error, event::{CloseReason, Event}, util::sync::mutex_lock, connection, window};
use super::ffi::*;

use std::collections::HashMap;

/// The initial capacity for any Vec<Event>
/// Event is around 8 bytes in size, so it's fairly costless for this to be a large starting capacity.
const QUEUE_SIZE: usize = 256;

pub(crate) struct Connection {
    display: *mut Display,
    connection: *mut xcb_connection_t,
    screen: *mut xcb_screen_t,
    event_buffer: HashMap<xcb_window_t, Vec<Event>>,
    hostname: Option<Vec<c_char>>,
    atoms: Atoms,
    extensions: Extensions,
}

#[derive(Clone, Copy)]
struct Atoms {
    wm_protocols: xcb_atom_t,
    wm_delete_window: xcb_atom_t,
    _net_wm_name: xcb_atom_t,
    utf8_string: xcb_atom_t,
    _net_wm_pid: xcb_atom_t,
    wm_client_machine: xcb_atom_t,
}

#[derive(Clone, Copy)]
struct Extensions {
    #[cfg(feature = "input")]
    xinput: u8,
}

impl Connection {
    pub(crate) fn new() -> Result<Self, Error> {
        unsafe {
            libX11::load()?;
            libX11_xcb::load()?;
            libxcb::load()?;

            let display = XOpenDisplay(std::ptr::null_mut());
            if display.is_null() {
                // TODO: Unclear why this could fail when passing nullptr to it. Maybe the system has no screens?
                // Maybe the underlying connection has failed, but how would we check?
                return Err(Error::Unknown)
            }
            let screen_num = XDefaultScreen(display);
            let connection = XGetXCBConnection(display);
            XSetEventQueueOwner(display, EventQueueOwner::XCBOwnsEventQueue);
            let mut iter = xcb_setup_roots_iterator(xcb_get_setup(connection));
            for _ in 0..screen_num {
                xcb_screen_next(&mut iter);
            }
            let screen = iter.data;
            let atoms = Atoms::new(connection)?;

            // Make sure xinput is available
            #[allow(unused_assignments)]
            let mut xi_opcode = 0;
            #[cfg(feature = "input")]
            {
                // xcb_query_extension cannot generate errors, so we don't check
                let xi_name = "XInputExtension";
                let xi = xcb_query_extension_reply(
                    connection,
                    xcb_query_extension(connection, xi_name.bytes().len() as _, xi_name.as_ptr().cast()),
                    std::ptr::null_mut(),
                );
                if xi.is_null() {
                    return Err(Error::SystemResources)
                }
                if (*xi).present == 0 {
                    return Err(Error::Unsupported)
                }
                xi_opcode = (*xi).major_opcode;
                free(xi.cast());

                libxcb_xinput::load()?;
            }

            // Try to get machine's hostname
            let mut len = 16;
            let mut hostname: Vec<c_char> = Vec::new();
            let hostname = loop {
                hostname.resize_with(len, Default::default); // Make sure vec is full of null-terminators
                let err = libc::gethostname((&mut hostname).as_mut_ptr(), len);
                if err == 0 {
                    // We got the hostname, now let's make sure the i8 vec is exactly the right size with no extra nulls
                    if let Some(pos) = hostname.iter().position(|x| *x == 0) {
                        hostname.set_len(pos + 1);
                    } else {
                        // There are no null-terminators, this means the vec was exactly the size of the hostname
                        // So we need to push a null-terminator onto it ourselves
                        hostname.push(0);
                    }
                    //hostname.shrink_to_fit(); // useful?
                    break Some(hostname);
                } else {
                    // Either ENAMETOOLONG or EINVAL would both indicate that the hostname is longer than the buffer
                    match len.checked_mul(2) {
                        Some(l) if l <= (1 << 16) => len = l,
                        _ => break None, // Give up if some sanity limit is reached or we overflowed usize..
                    }
                }
            };

            Ok(Connection {
                display,
                connection,
                screen,
                event_buffer: HashMap::new(),
                hostname,
                atoms,
                extensions: Extensions {
                    #[cfg(feature = "input")]
                    xinput: xi_opcode,
                },
            })
        }
    }

    // Helper wrapper for `xcb_connection_has_error` for use with `?`. Assumes pointer is valid.
    unsafe fn check(c: *mut xcb_connection_t) -> Result<(), Error> {
        let err = xcb_connection_has_error(c);
        match err {
            XCB_NONE => Ok(()),
            XCB_CONN_CLOSED_EXT_NOTSUPPORTED => Err(Error::Unsupported),
            XCB_CONN_CLOSED_MEM_INSUFFICIENT => Err(Error::SystemResources),
            _ => Err(Error::Invalid),
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            let _ = xcb_flush(self.connection);
            let _ = XCloseDisplay(self.display);
        }
    }
}

unsafe impl Send for Connection {}

impl Atoms {
    unsafe fn new(connection: *mut xcb_connection_t) -> Result<Self, Error> {
        const N_ATOMS: usize = 6;
        let mut atom_replies = [0 as c_uint; N_ATOMS];
        let mut atoms = [0 as xcb_atom_t; N_ATOMS];
        macro_rules! atom {
            ($n:literal, $name:literal) => {{
                atom_replies[$n] = xcb_intern_atom(connection, 0, $name.len() as u16, $name.as_ptr().cast());
            }};
        }
        atom!(0, "WM_PROTOCOLS");
        atom!(1, "WM_DELETE_WINDOW");
        atom!(2, "_NET_WM_NAME");
        atom!(3, "UTF8_STRING");
        atom!(4, "_NET_WM_PID");
        atom!(5, "WM_CLIENT_MACHINE");
        for (r, seq) in atoms.iter_mut().zip(atom_replies.into_iter()) {
            let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
            let reply = xcb_intern_atom_reply(connection, seq, &mut err);
            if !reply.is_null() {
                *r = (*reply).atom;
                free(reply.cast());
            } else {
                free(err.cast());
                // xcb_intern_atom can only fail due to alloc error or value error,
                // and this can't be a value error because we always pass a valid value (0) for only_if_exists
                return Err(Error::SystemResources);
            }
        }
        Ok(Self {
            wm_protocols: atoms[0],
            wm_delete_window: atoms[1],
            _net_wm_name: atoms[2],
            utf8_string: atoms[3],
            _net_wm_pid: atoms[4],
            wm_client_machine: atoms[5],
        })
    }
}

pub(crate) struct Window {
    connection: connection::Connection,
    handle: xcb_window_t,
    event_buffer: Vec<Event>,
}

impl Window {
    pub(crate) fn new(builder: window::Builder) -> Result<Self, Error> {
        unsafe {
            let mut connection_mtx = mutex_lock(&builder.connection.0);
            let connection: &mut Connection = &mut *connection_mtx;
            let c = connection.connection;
            let hostname = connection.hostname.as_ref();

            // Generate an ID for our new window
            let xid = xcb_generate_id(c);
            if xid == !0u32 {
                // xcb_generate_id returns -1 on any type of failure, most likely because it has run out of
                // resources to fulfil requests for new IDs. It could also mean the connection has been closed.
                return Err(Error::SystemResources);
            }

            // Clear the event queue, in case any events remain in it intended for a previous object with this xid we just claimed
            let event = xcb_poll_for_event(c);
            if !event.is_null() {
                if let Some((event, window)) = process_event(&connection.atoms, &connection.extensions, event) {
                    if let Some(queue) = connection.event_buffer.get_mut(&window) {
                        queue.push(event);
                    }
                }
            }
            let mut event = xcb_poll_for_queued_event(c);
            while !event.is_null() {
                if let Some((event, window)) = process_event(&connection.atoms, &connection.extensions, event) {
                    if let Some(queue) = connection.event_buffer.get_mut(&window) {
                        queue.push(event);
                    }
                }
                event = xcb_poll_for_queued_event(c);
            }

            // Create the new X window
            // ButtonPress is exclusive, so we request it in CreateWindow to make sure we get it first
            #[cfg(feature = "input")]
            const EVENT_MASK: u32 = XCB_EVENT_MASK_BUTTON_PRESS;
            #[cfg(not(feature = "input"))]
            const EVENT_MASK: u32 = 0;
            const VALUE_MASK: u32 = XCB_CW_EVENT_MASK;
            const VALUE_LIST: &[u32] = &[EVENT_MASK];

            let create_error = xcb_request_check(c, xcb_create_window_checked(
                c,
                XCB_COPY_FROM_PARENT,
                xid,
                (*connection.screen).root, // idk
                0,
                0,
                800,
                608,
                0,
                XCB_WINDOW_CLASS_INPUT_OUTPUT,
                XCB_COPY_FROM_PARENT.into(),
                VALUE_MASK,
                VALUE_LIST.as_ptr(),
            ));
            if !create_error.is_null() {
                // Reasons CreateWindow may fail are:
                // Alloc - maps to Error::SystemResources
                // Colormap - we don't currently pass a colormap
                // Cursor - we do not pass a Cursor
                // IDChoice - we got our ID straight from xcb_generate_id and didn't use it for anything else
                // Match - bad configuration of user params, so maps to Error::Invalid
                // Pixmap - we don't currently pass a pixmap
                // Value - bad value for a user param, so maps to Error::Invalid
                // Window - we just created that XID so that's not possible
                let errno = (*create_error).error_code;
                free(create_error.cast());
                if errno as c_int == XCB_ALLOC {
                    return Err(Error::SystemResources);
                } else {
                    return Err(Error::Invalid);
                }
            }

            // Select xinput events
            #[cfg(feature = "input")]
            {
                // xcb_input_xi_select_events cannot generate errors so we use _checked and discard it
                #[repr(C)]
                struct XiMask {
                    head: xcb_input_event_mask_t,
                    body: u32,
                }
                let mut mask = XiMask {
                    head: xcb_input_event_mask_t {
                        deviceid: XCB_INPUT_DEVICE_ALL_MASTER,
                        mask_len: 1,
                    },
                    body: XCB_INPUT_XI_EVENT_MASK_KEY_PRESS | XCB_INPUT_XI_EVENT_MASK_KEY_RELEASE
                        | XCB_INPUT_XI_EVENT_MASK_BUTTON_PRESS | XCB_INPUT_XI_EVENT_MASK_BUTTON_RELEASE
                        | XCB_INPUT_XI_EVENT_MASK_MOTION | XCB_INPUT_XI_EVENT_MASK_ENTER | XCB_INPUT_XI_EVENT_MASK_LEAVE
                        | XCB_INPUT_XI_EVENT_MASK_FOCUS_IN | XCB_INPUT_XI_EVENT_MASK_FOCUS_OUT,
                };
                xcb_discard_reply(c, xcb_input_xi_select_events_checked(c, xid, 1, (&mut mask.head) as _));
            }

            // Add WM_DELETE_WINDOW to WM_PROTOCOLS
            let _ = xcb_change_property(
                c,
                XCB_PROP_MODE_REPLACE,
                xid,
                connection.atoms.wm_protocols,
                XCB_ATOM_ATOM,
                32,
                1,
                (&connection.atoms.wm_delete_window) as *const u32 as _,
            );

            // Try to write the requested window title to the WM_NAME and _NET_WM_NAME properties
            // Note: multibyte characters won't render correctly in WM_NAME, but any modern and worthwhile WM will
            // prioritise using _NET_WM_NAME which is UTF-8 as standard, that's why it's better to write both.
            let title = builder.title.as_ref();
            let _ = xcb_change_property(
                c,
                XCB_PROP_MODE_REPLACE,
                xid,
                connection.atoms._net_wm_name,
                connection.atoms.utf8_string,
                8,
                title.bytes().len() as _,
                title.as_ptr().cast(),
            );
            let _ = xcb_change_property(
                c,
                XCB_PROP_MODE_REPLACE,
                xid,
                XCB_ATOM_WM_NAME,
                XCB_ATOM_STRING,
                8,
                title.bytes().len() as _,
                title.as_ptr().cast(),
            );

            // If hostname is known, get PID of current process and write that to _NET_WM_PID
            // But don't write either of these properties if hostname is not known, because:
            // "If _NET_WM_PID is set, the ICCCM-specified property WM_CLIENT_MACHINE MUST also be set." - EWMH spec
            if let Some(hostname) = hostname {
                let pid = getpid();
                let _ = xcb_change_property(
                    c,
                    XCB_PROP_MODE_REPLACE,
                    xid,
                    connection.atoms._net_wm_pid,
                    XCB_ATOM_CARDINAL,
                    32,
                    1,
                    (&pid) as *const i32 as _,
                );

                let _ = xcb_change_property(
                    c,
                    XCB_PROP_MODE_REPLACE,
                    xid,
                    connection.atoms.wm_client_machine,
                    XCB_ATOM_STRING,
                    8,
                    hostname.len() as _,
                    hostname.as_ptr().cast(),
                );
            }

            // Try to map window to screen
            let map_error = xcb_request_check(c, xcb_map_window_checked(c, xid));
            if !map_error.is_null() {
                // Can only fail due to "Window" error, so I think this is unreachable in practice
                free(map_error.cast());
                Connection::check(c)?;
                return Err(Error::Unknown)
            }

            // Now we'll insert an entry into the EVENT_QUEUE hashmap for this window we've created.
            // We do this even if the queue probably won't be used, as it's the soundest way to ensure
            // memory gets cleaned up.
            let _ = connection.event_buffer.insert(xid, Vec::with_capacity(QUEUE_SIZE));

            // TODO: This "returns <= 0 on error", how is that value significant? Is it -EINVAL type thing?
            if xcb_flush(c) <= 0 {
                Connection::check(c)?;
                return Err(Error::Unknown)
            }

            std::mem::drop(connection_mtx);
            Ok(Window {
                connection: builder.connection,
                handle: xid,
                event_buffer: Vec::with_capacity(QUEUE_SIZE),
            })
        }
    }

    pub(crate) fn events(&self) -> &[Event] {
        &self.event_buffer
    }

    pub(crate) fn poll_events(&mut self) {
        unsafe {
            // First: lock the global event queue, which is used as backup storage for events
            // which have been pulled but are not immediately relevant
            let mut connection_ = mutex_lock(&self.connection.0);
            let Connection {
                atoms,
                extensions,
                connection: c,
                event_buffer: map,
                ..
            } = &mut *connection_;

            // Clear our event buffer of the previous set of events
            self.event_buffer.clear();

            // Fill our event buffer with any events which may have been stored in the global event queue,
            // also clearing them from the global queue
            // Note: this queue SHOULD always exist, but it's possible some bad or malicious user code might get a
            // `None` result, so it's better to check and take no action if there's no queue to copy from...
            if let Some(queue) = map.get_mut(&self.handle) {
                std::mem::swap(&mut self.event_buffer, queue);
            }

            // Call `poll_event` once, which populates XCB's internal linked list from the connection
            let event = xcb_poll_for_event(*c);
            if !event.is_null() {
                if let Some((event, window)) = process_event(atoms, extensions, event) {
                    if window == self.handle {
                        self.event_buffer.push(event);
                    } else if let Some(queue) = map.get_mut(&window) {
                        queue.push(event);
                    }
                }
            }
            let mut event = xcb_poll_for_queued_event(*c);
            while !event.is_null() {
                if let Some((event, window)) = process_event(atoms, extensions, event) {
                    if window == self.handle {
                        self.event_buffer.push(event);
                    } else if let Some(queue) = map.get_mut(&window) {
                        queue.push(event);
                    }
                }
                event = xcb_poll_for_queued_event(*c);
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let mut connection_ = mutex_lock(&self.connection.0);
        let connection = &mut connection_;
        unsafe {
            let _ = xcb_destroy_window(connection.connection, self.handle);
            let _ = xcb_flush(connection.connection);
        }
    }
}

unsafe fn process_event(atoms: &Atoms, extensions: &Extensions, ev: *mut xcb_generic_event_t) -> Option<(Event, xcb_window_t)> {
    let mapping = match (*ev).response_type & !(1 << 7) {
        XCB_CLIENT_MESSAGE => {
            let event = &*(ev as *mut xcb_client_message_event_t);
            if event.format == 32 && event.r#type == atoms.wm_protocols &&
                event.client_data.data32[0] == atoms.wm_delete_window
            {
                Some((Event::CloseRequest(CloseReason::SystemMenu), event.window))
            } else {
                None
            }
        },
        #[cfg(feature = "input")]
        XCB_GE_GENERIC => {
            let event = &*(ev as *mut xcb_ge_generic_event_t);
            if event.extension == extensions.xinput {
                match event.event_type & !(1 << 7) {
                    XCB_INPUT_KEY_PRESS => {
                        // TODO: this
                        let _event = &*(ev as *mut xcb_input_key_press_event_t);
                        println!("Key press");
                        None
                    },
                    XCB_INPUT_KEY_RELEASE => {
                        // TODO: this
                        let _event = &*(ev as *mut xcb_input_key_release_event_t);
                        println!("Key release");
                        None
                    },
                    XCB_INPUT_BUTTON_PRESS => {
                        // TODO: this
                        let _event = &*(ev as *mut xcb_input_button_press_event_t);
                        println!("Button press");
                        None
                    },
                    XCB_INPUT_BUTTON_RELEASE => {
                        // TODO: this
                        let _event = &*(ev as *mut xcb_input_button_release_event_t);
                        println!("Button release");
                        None
                    },
                    XCB_INPUT_MOTION => {
                        // TODO: this
                        let _event = &*(ev as *mut xcb_input_motion_event_t);
                        None
                    },
                    XCB_INPUT_ENTER => {
                        // TODO: this
                        let _event = &*(ev as *mut xcb_input_enter_event_t);
                        println!("Input enter");
                        None
                    },
                    XCB_INPUT_LEAVE => {
                        // TODO: this
                        let _event = &*(ev as *mut xcb_input_leave_event_t);
                        println!("Input leave");
                        None
                    },
                    e @ XCB_INPUT_FOCUS_IN | e @ XCB_INPUT_FOCUS_OUT => {
                        let state = e == XCB_INPUT_FOCUS_IN;
                        Some((Event::Focus(state), (*(ev as *mut xcb_input_focus_in_event_t)).event))
                    },
                    _ => None,
                }
            } else {
                None
            }
        },
        _ => None,
    };
    free(ev.cast());
    mapping
}
