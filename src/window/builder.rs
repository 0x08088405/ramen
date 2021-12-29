use crate::platform::imp;

/// Builder for instantiating a [`Window`](super::Window).
///
/// To create a builder, use [`Window::builder`](super::Window::builder).
pub struct Builder {
    visible: bool,
}

impl Builder {
    pub(crate) const fn new() -> Self {
        Builder {
            visible: true,
        }
    }

    pub fn build(&self) -> Result<super::Window, ()> {
        imp::Window::new(self).map(super::Window)
    }

    /// Sets whether the window is initially visible.
    ///
    /// Defaults to `true`.
    pub fn visible(&mut self, visible: bool) -> &mut Self {
        self.visible = visible;
        self
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
