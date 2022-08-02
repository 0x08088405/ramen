// TODO: I suppose we'll need some method of deciding at runtime whether to use x11 or wayland? This is just x11
use crate::{error::Error, event::Event, window};
use super::ffi::{self, XCB, XCB_EVENT_MASK_KEY_PRESS, XCB_EVENT_MASK_KEY_RELEASE, XCB_EVENT_MASK_BUTTON_PRESS, XCB_EVENT_MASK_BUTTON_RELEASE, XCB_ATOM_WM_NAME};

pub(crate) struct Window {
    handle: ffi::XcbWindow,
    event_queue: Vec<Event>,
}

impl Window {
    pub(crate) fn new(builder: &window::Builder) -> Result<Self, Error> {
        if XCB.is_valid() {
            let id = XCB.generate_id();
            let value_mask = ffi::XCB_CW_BACK_PIXEL | ffi::XCB_CW_EVENT_MASK;
            let value_list = &[
                XCB.white_pixel(),
                XCB_EVENT_MASK_KEY_PRESS | XCB_EVENT_MASK_KEY_RELEASE | XCB_EVENT_MASK_BUTTON_PRESS | XCB_EVENT_MASK_BUTTON_RELEASE,
            ];
            if XCB.create_window(id, 1, 1, 800, 608, 1, value_mask, value_list).is_err() {
                return Err(Error::SystemResources) // TODO: ?
            }

            let delete_prop = XCB.intern_atom(true, "WM_PROTOCOLS");
            if delete_prop == ffi::XCB_ATOM_NONE { 
                return Err(Error::SystemResources) // TODO: 
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

            let title = builder.title.as_ref();
            let title_prop = XCB.intern_atom(false, "_NET_WM_NAME");
            let title_prop_type = XCB.intern_atom(false, "UTF8_STRING");
            XCB.change_property(
                ffi::XCB_PROP_MODE_REPLACE,
                id,
                title_prop,
                title_prop_type,
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
            
            if XCB.flush().is_err() {
                return Err(Error::SystemResources) // TODO: ???
            }
            if XCB.map_window(id).is_err() {
                return Err(Error::SystemResources) // TODO: ??
            }
            Ok(Window { handle: id, event_queue: Vec::with_capacity(64) })
        } else {
            match ffi::dl_error() {
                Some(s) => Err(Error::Other(s)),
                None => Err(Error::SystemResources), // TODO: ?, ?
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
