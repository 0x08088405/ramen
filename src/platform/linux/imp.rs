// TODO: I suppose we'll need some method of deciding at runtime whether to use x11 or wayland? This is just x11
use crate::{error::Error, event::{CloseReason, Event}, util::sync::{Mutex, mutex_lock}, window};
use super::ffi::{self, XCB};

use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref EVENT_QUEUE: Mutex<HashMap<ffi::XcbWindow, Vec<Event>>> = Mutex::new(HashMap::with_capacity(8));
}

/// The initial capacity for any Vec<Event>
/// Event is around 8 bytes in size, so it's fairly costless for this to be a large starting capacity.
const QUEUE_SIZE: usize = 256;

pub(crate) struct Window {
    handle: ffi::XcbWindow,
    event_buffer: Vec<Event>,
}

impl Window {
    pub(crate) fn new(builder: &window::Builder) -> Result<Self, Error> {
        // Check if XCB setup failed, possibly due to libxcb or an extension not being installed.
        // This is recoverable - for example, the user might want to try Wayland setup if this fails.
        if XCB.is_valid() {
            // Generate an ID and spawn a window with that ID
            let id = match XCB.generate_id() {
                // xcb_generate_id returns -1 on any type of failure, most likely because it has run out of
                // resources to fulfil requests for new IDs. It could also mean the connection has been closed.
                Some(id) => id,
                None => return Err(Error::SystemResources),
            };
            let value_mask = ffi::XCB_CW_BACK_PIXEL | ffi::XCB_CW_EVENT_MASK;
            let value_list = &[
                XCB.white_pixel(),
                ffi::XCB_EVENT_MASK_KEY_PRESS | ffi::XCB_EVENT_MASK_KEY_RELEASE | ffi::XCB_EVENT_MASK_BUTTON_PRESS | ffi::XCB_EVENT_MASK_BUTTON_RELEASE,
            ];
            match XCB.create_window(id, 1, 1, 800, 608, 1, value_mask, value_list) {
                // Reasons CreateWindow may fail are:
                // Alloc - maps to Error::OutOfMemory
                // Colormap - we don't currently pass a colormap
                // Cursor - we do not pass a Cursor
                // IDChoice - we got our ID straight from xcb_generate_id and didn't use it for anything else
                // Match - bad configuration of user params, so maps to Error::Invalid
                // Pixmap - we don't currently pass a pixmap
                // Value - bad value for a user param, so maps to Error::Invalid
                // Window - all window IDs we use are checked in advance
                Ok(()) => (),
                Err(ffi::Error(ffi::XCB_ALLOC)) => return Err(Error::OutOfMemory),
                Err(_) => return Err(Error::Invalid),
            }

            // Add WM_DELETE_WINDOW to WM_PROTOCOLS - important so we can hook the user clicking the X button
            XCB.change_property(
                ffi::XCB_PROP_MODE_REPLACE,
                id,
                XCB.atom_wm_protocols,
                ffi::XCB_ATOM_ATOM,
                32,
                1,
                (&XCB.atom_wm_delete_window) as *const u32 as _,
            );

            // Try to write the requested window title to the WM_NAME and _NET_WM_NAME properties
            // Note: multibyte characters won't render correctly in WM_NAME, but any correctly-implemented WM will
            // prioritise using _NET_WM_NAME which is UTF-8 as standard, that's why it's better to write both.
            let title = builder.title.as_ref();
            XCB.change_property(
                ffi::XCB_PROP_MODE_REPLACE,
                id,
                XCB.atom_net_wm_name,
                XCB.atom_utf8_string,
                8,
                title.bytes().len() as _,
                title.as_ptr().cast(),
            );
            XCB.change_property(
                ffi::XCB_PROP_MODE_REPLACE,
                id,
                ffi::XCB_ATOM_WM_NAME,
                ffi::XCB_ATOM_STRING,
                8,
                title.bytes().len() as _,
                title.as_ptr().cast(),
            );

            // Get PID of current process and write that to _NET_WM_PID
            let pid = unsafe { libc::getpid() };
            XCB.change_property(
                ffi::XCB_PROP_MODE_REPLACE,
                id,
                XCB.atom_net_wm_pid,
                ffi::XCB_ATOM_CARDINAL,
                32,
                1,
                (&pid) as *const i32 as _,
            );
            
            // Flush FFI requests. If this fails, it can only mean that the connection was invalidated at some point
            // since we opened it. The most plausible cause of this would be a lack of system resources.
            if XCB.flush().is_err() {
                // No point trying to destroy the window here if the connection is already closed...
                return Err(Error::SystemResources)
            }

            // And finally, try to map the window to the screen
            // If successful the window will become visible at this point.
            if XCB.map_window(id).is_err() {
                // Can only fail due to "Window" error, so I think this is unreachable in practice
                let _ = XCB.destroy_window(id);
                return Err(Error::Invalid)
            }

            // Now we'll insert an entry into the EVENT_QUEUE hashmap for this window we've created.
            // We do this even if the queue probably won't be used, as it's the soundest way to ensure memory gets cleaned up.
            let _ = mutex_lock(&EVENT_QUEUE).insert(id, Vec::with_capacity(QUEUE_SIZE));
            
            Ok(Window { handle: id, event_buffer: Vec::with_capacity(QUEUE_SIZE) })
        } else {
            match XCB.setup_error() {
                ffi::SetupError::DlError(s) => Err(Error::Text(s.into())),
                ffi::SetupError::NoScreens => Err(Error::Unsupported),
                ffi::SetupError::ConnError(ffi::Error(ffi::XCB_CONN_CLOSED_EXT_NOTSUPPORTED)) => Err(Error::Unsupported),
                ffi::SetupError::ConnError(ffi::Error(ffi::XCB_CONN_CLOSED_MEM_INSUFFICIENT)) => Err(Error::OutOfMemory),
                ffi::SetupError::ConnError(_) => Err(Error::Invalid),
                ffi::SetupError::XcbError(ffi::Error(ffi::XCB_ALLOC)) => Err(Error::OutOfMemory),
                ffi::SetupError::XcbError(_) => Err(Error::Invalid),
            }
        }
    }

    pub(crate) fn events(&self) -> &[Event] {
        &self.event_buffer
    }

    pub(crate) fn poll_events(&mut self) {
        // First: lock the global event queue, which is used as backup storage for events
        // which have been pulled but are not immediately relevant
        let mut map = mutex_lock(&EVENT_QUEUE);

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
        if let Some((event, window)) = XCB.poll_event().and_then(process_event) {
            // We have a ramen event relevant to some window - but is it this one?
            if window == self.handle {
                self.event_buffer.push(event);
            } else if let Some(queue) = map.get_mut(&window) {
                queue.push(event);
            }
        }
        while let Some(event) = XCB.poll_queued_event() {
            if let Some((event, window)) = process_event(event) {
                // We have a ramen event relevant to some window - but is it this one?
                if window == self.handle {
                    self.event_buffer.push(event);
                } else if let Some(queue) = map.get_mut(&window) {
                    queue.push(event);
                }
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let _ = mutex_lock(&EVENT_QUEUE).remove(&self.handle);
        if XCB.destroy_window(self.handle).is_ok() {
            let _ = XCB.flush();
        }
    }
}

// For translating an ffi Event to a ramen Event
fn process_event(ev: ffi::Event) -> Option<(Event, ffi::XcbWindow)> {
    unsafe {
        match ev {
            ffi::Event::ClientMessage { format, client_data, r#type, window } => {
                if format == 32 && r#type == XCB.atom_wm_protocols && client_data.data32[0] == XCB.atom_wm_delete_window {
                    Some((Event::CloseRequest(CloseReason::SystemMenu), window))
                } else {
                    None
                }
            },
            //_ => None,
        }
    }
}
