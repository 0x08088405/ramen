mod builder;
mod decoration;

pub use self::{
    builder::Builder,
    decoration::{Controls, Style},
};

use crate::{event::Event, platform::imp};

/// Represents an open window. Dropping it closes the window.
///
/// To instantiate windows, use a [`builder`](Self::builder).
pub struct Window(imp::Window);

impl Window {
    /// Creates a [`Builder`] for interactively instantiating a [`Window`].
    pub const fn builder() -> Builder {
        Builder::new(None)
    }

    /// Similar to [`builder`](Self::builder), but derives the defaults from a given [`Style`].
    pub const fn with_style(style: Style) -> Builder {
        Builder::new(Some(style))
    }

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
    pub fn poll_events(&mut self) {
        self.0.poll_events()
    }
}
