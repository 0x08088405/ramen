// TODO: I suppose we'll need some method of deciding at runtime whether to use x11 or wayland? This is just x11
use crate::{error::Error, event::Event, window};
use super::ffi::{self, XCB};

pub(crate) struct Window {
    handle: ffi::XcbWindow,
    event_queue: Vec<Event>,
}

impl Window {
    pub(crate) fn new(builder: &window::Builder) -> Result<Self, Error> {
        // Check if XCB setup failed, possibly due to libxcb or an extension not being installed.
        // This is recoverable - for example, the user might want to try Wayland setup if this fails.
        if XCB.is_valid() {
            // Generate an ID and spawn a window with that ID
            let id = XCB.generate_id();
            let value_mask = ffi::XCB_CW_BACK_PIXEL | ffi::XCB_CW_EVENT_MASK;
            let value_list = &[
                XCB.white_pixel(),
                ffi::XCB_EVENT_MASK_KEY_PRESS | ffi::XCB_EVENT_MASK_KEY_RELEASE | ffi::XCB_EVENT_MASK_BUTTON_PRESS | ffi::XCB_EVENT_MASK_BUTTON_RELEASE,
            ];
            if XCB.create_window(id, 1, 1, 800, 608, 1, value_mask, value_list).is_err() {
                // xcb_create_window failed for some reason, presumably a bad user param
                return Err(Error::Invalid)
            }

            // Add WM_DELETE_WINDOW to WM_PROTOCOLS - important so we can hook the user clicking the X button
            let delete_prop = XCB.intern_atom(true, "WM_PROTOCOLS");
            if delete_prop == ffi::XCB_ATOM_NONE {
                // No WM_PROTOCOLS prop on window?
                return Err(Error::Unsupported)
            }
            let delete_data = XCB.intern_atom(false, "WM_DELETE_WINDOW");
            XCB.change_property(
                ffi::XCB_PROP_MODE_REPLACE,
                id,
                delete_prop,
                ffi::XCB_ATOM_ATOM,
                32,
                1,
                (&delete_data) as *const u32 as _,
            );

            // Try to write the requested window title to the WM_NAME and _NET_WM_NAME properties
            // Note: _NET_WM_NAME must be of type UTF8_STRING. We can only write it if UTF8_STRING prop type exists on
            // the user's system. If it doesn't exist, and the title contains multibyte UTF8 characters, AND the user's
            // system locale is not UTF8, then the title won't render properly. I don't care and neither should you.
            let title = builder.title.as_ref();
            let title_prop = XCB.intern_atom(false, "_NET_WM_NAME");
            let title_prop_type = XCB.intern_atom(true, "UTF8_STRING");
            if title_prop_type != ffi::XCB_ATOM_NONE {
                XCB.change_property(
                    ffi::XCB_PROP_MODE_REPLACE,
                    id,
                    title_prop,
                    title_prop_type,
                    8,
                    title.bytes().len() as _,
                    title.as_ptr().cast(),
                );
            }
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
            let pid_prop = XCB.intern_atom(false, "_NET_WM_PID");
            XCB.change_property(
                ffi::XCB_PROP_MODE_REPLACE,
                id,
                pid_prop,
                ffi::XCB_ATOM_CARDINAL,
                32,
                1,
                (&pid) as *const i32 as _,
            );
            
            // Flush FFI requests. I'm not really sure what condition would cause an error here, if one even exists.
            if XCB.flush().is_err() {
                return Err(Error::Invalid)
            }

            // And finally, try to map the window to the screen
            // If successful the window will become visible at this point.
            if XCB.map_window(id).is_err() {
                return Err(Error::Invalid)
            }
            Ok(Window { handle: id, event_queue: Vec::with_capacity(64) })
        } else {
            match ffi::dl_error() {
                Some(s) => Err(Error::Text(s.into())),
                None => Err(Error::Unsupported),
            }
        }
    }

    pub(crate) fn events(&self) -> &[Event] {
        &self.event_queue
    }

    pub(crate) fn poll_events(&mut self) {
        self.event_queue.clear();
        if let Some(event) = XCB.poll_event() {
            println!("{:?}", event);
            loop {
                let event = XCB.poll_queued_event();
                if event.is_none() { break } else { println!("{:?}", event) }
            }
        }
        
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if XCB.destroy_window(self.handle).is_ok() {
            let _ = XCB.flush();
        }
    }
}
