// TODO: I suppose we'll need some method of deciding at runtime whether to use x11 or wayland? This is just x11
use crate::{error::Error, event::Event, window};
use super::ffi::{self, XCB};

pub(crate) struct Window(ffi::XcbWindow);

impl Window {
    pub(crate) fn new(_builder: &window::Builder) -> Result<Self, Error> {
        if XCB.is_valid() {
            let id = XCB.generate_id();
            if XCB.create_window(id, 1, 1, 800, 608, 1, 0, &[]).is_err() {
                return Err(Error::SystemResources) // TODO: ?
            }
            if XCB.map_window(id).is_err() {
                return Err(Error::SystemResources) // TODO: ??
            }
            if XCB.flush().is_err() {
                return Err(Error::SystemResources) // TODO: ???
            }
            Ok(Window(id))
        } else {
            match ffi::dl_error() {
                Some(s) => Err(Error::Other(s)),
                None => Err(Error::SystemResources), // TODO: ?, ?
            }
        }
    }

    pub(crate) fn events(&self) -> &[Event] {
        &[]
    }

    pub(crate) fn poll_events(&mut self) {
        
    }
}
