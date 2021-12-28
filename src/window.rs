use crate::platform::imp;

/// Represents an open window. Dropping it closes the window.
///
/// To instantiate windows, use a [`builder`](Self::builder).
pub struct Window(imp::Window);

impl Window {
    pub const fn builder() -> WindowBuilder {
        WindowBuilder { visible: true }
    }
}

pub struct WindowBuilder {
    visible: bool,
}

impl WindowBuilder {
    pub fn build(&self) -> Result<Window, ()> {
        imp::Window::new(self).map(Window)
    }

    /// Sets whether the window is initially visible.
    ///
    /// Defaults to `true`.
    pub fn visible(&mut self, visible: bool) -> &mut Self {
        self.visible = visible;
        self
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Window::builder()
    }
}
