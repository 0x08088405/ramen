mod builder;
mod decoration;

pub use self::{
    builder::Builder,
    decoration::{Controls, Style},
};

use crate::{event::Event, platform::imp};

/// Represents an open window. Dropping it closes the window.
///
/// To instantiate windows, use a [`builder`](crate::connection::Connection::builder).
pub struct Window(imp::Window);

impl Window {
    /// Returns an iterator of events currently in the buffer. The buffer must first be populated with `poll_events()`.
    /// After calling `poll_events()` once, the buffer contents will remain the same, every time this function is
    /// called, until the next time `poll_events()` is called.
    /// 
    /// The return type of this function is defined as `impl IntoIterator<Item = &Event>`. The concrete type may be
    /// different on other platforms or in future versions of `ramen`. As such, your code should not make any
    /// assumptions about what type this function will return, other than that it will be iterable for `Event`s.
    pub fn events(&self) -> impl IntoIterator<Item = &Event> {
        self.0.events()
    }

    /// Pulls any new events into the buffer, discarding any events which were previously in the buffer.
    /// 
    /// Query the buffer by calling `events()`.
    /// 
    /// This function needs to be called regularly to let the operating system know that the application is still
    /// running and hasn't frozen. If this function isn't called on a window within a reasonable amount of time
    /// (usually a few seconds), then the operating system may mark it as unresponsive and/or try to kill it.
    /// 
    /// Note however that there is no such requirement for calling `events()`.
    pub fn poll_events(&mut self) {
        self.0.poll_events()
    }

    pub fn set_position(&self, position: (i16, i16)) {
        self.0.set_position(position)
    }

    pub fn set_size(&self, size: (u16, u16)) {
        self.0.set_size(size)
    }

    pub fn set_visible(&self, visible: bool) {
        self.0.set_visible(visible)
    }
}
