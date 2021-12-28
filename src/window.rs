/// Represents an open window. Dropping it closes the window.
///
/// To instantiate windows, use a [`builder`](Self::builder).
pub struct Window {

}

impl Window {
    pub const fn builder() -> WindowBuilder {
        WindowBuilder::new()
    }
}

pub struct WindowBuilder {
    visible: bool,
}

impl WindowBuilder {
    const fn new() -> Self {
        Self {
            visible: true,
        }
    }

    pub fn build(&self) -> Result<Window, ()> {
        todo!()
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
        Self::new()
    }
}
