mod builder;

pub use self::builder::Builder;

use crate::platform::imp;

/// Represents an open window. Dropping it closes the window.
///
/// To instantiate windows, use a [`builder`](Self::builder).
pub struct Window(imp::Window);

impl Window {
    pub const fn builder() -> Builder {
        Builder::new()
    }
}
