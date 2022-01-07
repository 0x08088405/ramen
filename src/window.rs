mod builder;
mod decoration;

pub use self::{
    builder::Builder,
    decoration::{Controls, Style},
};

use crate::platform::imp;

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
}
