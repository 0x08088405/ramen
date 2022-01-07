use crate::platform::imp;
use std::borrow::Cow;

/// Builder for instantiating a [`Window`](super::Window).
///
/// To create a builder, use [`Window::builder`](super::Window::builder) or the default implementation.
pub struct Builder {
    pub(crate) class_name: Cow<'static, str>,
    pub(crate) title: Cow<'static, str>,
    pub(crate) visible: bool,
}

impl Builder {
    pub(crate) const fn new() -> Self {
        Builder {
            class_name: Cow::Borrowed("ramen_window"),
            title: Cow::Borrowed("a nice window"),
            visible: true,
        }
    }

    pub fn build(&self) -> Result<super::Window, ()> {
        imp::Window::new(self).map(super::Window)
    }

    /// Sets the platform-specific window class name.
    ///
    /// Defaults to `"ramen_window"`.
    pub fn class_name<T>(&mut self, class_name: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.class_name = class_name.into();
        self
    }

    /// Sets the initial window title.
    ///
    /// Defaults to `"a nice window"`.
    pub fn title<T>(&mut self, title: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.title = title.into();
        self
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
