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
    atoms: Atoms,
}

#[derive(Clone, Copy)]
struct Atoms {
    wm_protocols: xcb_atom_t,
    wm_delete_window: xcb_atom_t,
    _net_wm_name: xcb_atom_t,
    utf8_string: xcb_atom_t,
    _net_wm_pid: xcb_atom_t,
}

impl Connection {
    pub(crate) fn new() -> Result<Self, Error> {
        unsafe {
            if let Err(e) = libX11::load() { return Err(e) }
            if let Err(e) = libX11_xcb::load() { return Err(e) }
            if let Err(e) = libxcb::load() { return Err(e) }
            let display = XOpenDisplay(std::ptr::null_mut());
            if display.is_null() {
                panic!("oh no"); // TODO
            }
            let screen_num = XDefaultScreen(display);
            let connection = XGetXCBConnection(display);
            XSetEventQueueOwner(display, EventQueueOwner::XCBOwnsEventQueue);
            let mut iter = xcb_setup_roots_iterator(xcb_get_setup(connection));
            for _ in 0..screen_num {
                xcb_screen_next(&mut iter);
            }
            let screen = iter.data;
            let atoms = Atoms::new(connection);
            Ok(Connection {
                display,
                connection,
                screen,
                atoms,
                event_buffer: HashMap::new(),
            })
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
    unsafe fn new(connection: *mut xcb_connection_t) -> Self {
        const N_ATOMS: usize = 5;
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
        for (i, seq) in atom_replies.into_iter().enumerate() {
            let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
            let reply = xcb_intern_atom_reply(connection, seq, &mut err);
            if !reply.is_null() {
                atoms[i] = (&*reply).atom;
                free(reply.cast());
            } else {
                free(err.cast());
                panic!("oh no we got an error");
            }
        }
        Self {
            wm_protocols: atoms[0],
            wm_delete_window: atoms[1],
            _net_wm_name: atoms[2],
            utf8_string: atoms[3],
            _net_wm_pid: atoms[4],
        }
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
            let c = connection_mtx.connection;

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
                if let Some((event, window)) = process_event(&connection_mtx.atoms, event) {
                    if let Some(queue) = connection_mtx.event_buffer.get_mut(&window) {
                        queue.push(event);
                    }
                }
            }
            let mut event = xcb_poll_for_queued_event(c);
            while !event.is_null() {
                if let Some((event, window)) = process_event(&connection_mtx.atoms, event) {
                    if let Some(queue) = connection_mtx.event_buffer.get_mut(&window) {
                        queue.push(event);
                    }
                }
                event = xcb_poll_for_queued_event(c);
            }

            // Create the new X window
            const EVENT_MASK: u32 = XCB_EVENT_MASK_KEY_PRESS
                | XCB_EVENT_MASK_KEY_RELEASE
                | XCB_EVENT_MASK_BUTTON_PRESS
                | XCB_EVENT_MASK_BUTTON_RELEASE
                | XCB_EVENT_MASK_FOCUS_CHANGE;
            const VALUE_MASK: u32 = XCB_CW_EVENT_MASK;
            const VALUE_LIST: &[u32] = &[EVENT_MASK];

            let create_error = xcb_request_check(c, xcb_create_window_checked(
                c,
                XCB_COPY_FROM_PARENT,
                xid,
                (&*connection_mtx.screen).root, // idk
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
                let errno = (&*create_error).error_code;
                free(create_error.cast());
                if errno as c_int == XCB_ALLOC {
                    return Err(Error::SystemResources);
                } else {
                    return Err(Error::Invalid);
                }
            }

            // Add WM_DELETE_WINDOW to WM_PROTOCOLS
            let _ = xcb_change_property(
                c,
                XCB_PROP_MODE_REPLACE,
                xid,
                connection_mtx.atoms.wm_protocols,
                XCB_ATOM_ATOM,
                32,
                1,
                (&connection_mtx.atoms.wm_delete_window) as *const u32 as _,
            );

            // Try to write the requested window title to the WM_NAME and _NET_WM_NAME properties
            // Note: multibyte characters won't render correctly in WM_NAME, but any modern and worthwhile WM will
            // prioritise using _NET_WM_NAME which is UTF-8 as standard, that's why it's better to write both.
            let title = builder.title.as_ref();
            let _ = xcb_change_property(
                c,
                XCB_PROP_MODE_REPLACE,
                xid,
                connection_mtx.atoms._net_wm_name,
                connection_mtx.atoms.utf8_string,
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

            // Get PID of current process and write that to _NET_WM_PID
            let pid = getpid();
            let _ = xcb_change_property(
                c,
                XCB_PROP_MODE_REPLACE,
                xid,
                connection_mtx.atoms._net_wm_pid,
                XCB_ATOM_CARDINAL,
                32,
                1,
                (&pid) as *const i32 as _,
            );

            // TODO: This "returns <= 0 on error", how is that value significant? Is it -EINVAL type thing?
            // TODO: Don't think this is needed, because of the checked map thing below. Sometime, look into it.
            let _ = xcb_flush(c);

            // time to map!!!
            let map_error = xcb_request_check(c, xcb_map_window_checked(c, xid));
            if !map_error.is_null() {
                // Can only fail due to "Window" error, so I think this is unreachable in practice
                // TODO: map connection errors
                free(map_error.cast());
                panic!("oh no");
            }

            // Now we'll insert an entry into the EVENT_QUEUE hashmap for this window we've created.
            // We do this even if the queue probably won't be used, as it's the soundest way to ensure
            // memory gets cleaned up.
            let _ = connection_mtx.event_buffer.insert(xid, Vec::with_capacity(QUEUE_SIZE));

            if xcb_connection_has_error(c) != 0 {
                panic!("oh no");
            }

            //std::mem::drop(connection);
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
            let Connection { atoms, connection: c, event_buffer: map, .. } = &mut *connection_;

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
                if let Some((event, window)) = process_event(&atoms, event) {
                    if window == self.handle {
                        self.event_buffer.push(event);
                    } else if let Some(queue) = map.get_mut(&window) {
                        queue.push(event);
                    }
                }
            }
            let mut event = xcb_poll_for_queued_event(*c);
            while !event.is_null() {
                if let Some((event, window)) = process_event(&atoms, event) {
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

unsafe fn process_event(atoms: &Atoms, ev: *mut xcb_generic_event_t) -> Option<(Event, xcb_window_t)> {
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
        XCB_FOCUS_IN | XCB_FOCUS_OUT => {
            let state = (*ev).response_type == XCB_FOCUS_IN;
            Some((Event::Focus(state), (&*(ev as *mut xcb_focus_in_event_t)).event))
        },
        _ => None,
    };
    free(ev.cast());
    mapping
}
