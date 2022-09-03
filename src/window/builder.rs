use super::{Controls, Style};
use crate::{connection::Connection, error::Error, platform::imp};

use std::borrow::Cow;

/// Builder for instantiating a [`Window`](super::Window).
///
/// To create a builder, use [`Connection::builder`](Connection::builder).
/// 
/// To finish building and open a window, use [`build`](Self::build). This will consume the Builder.
/// Builders can be cloned if you'd like to re-use one to build multiple windows.
#[derive(Clone)]
pub struct Builder {
    pub(crate) connection: Connection,
    pub(crate) class_name: Cow<'static, str>,
    pub(crate) style: Style,
    pub(crate) title: Cow<'static, str>,
}

impl Builder {
    pub(crate) const fn new(connection: Connection, style: Option<Style>) -> Self {
        Builder {
            connection,
            class_name: Cow::Borrowed("ramen_window"),
            style: match style {
                // Why is `Option::unwrap_or` not const fn?!
                Some(style) => style,
                None => Style::new(),
            },
            title: Cow::Borrowed("a nice window"),
        }
    }

    pub fn build(self) -> Result<super::Window, Error> {
        imp::Window::new(self).map(super::Window)
    }

    pub fn borderless(mut self, borderless: bool) -> Self {
        self.style.borderless = borderless;
        self
    }

    pub fn controls(mut self, controls: Option<Controls>) -> Self {
        self.style.controls = controls;
        self
    }

    /// Sets the platform-specific window class name.
    ///
    /// Defaults to `"ramen_window"`.
    pub fn class_name<T>(mut self, class_name: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.class_name = class_name.into();
        self
    }

    /// Sets whether the window can be initially interactively resized by the user.
    ///
    /// Note that this being `false` does not prevent it being done via API calls.
    ///
    /// Defaults to `true`.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.style.resizable = resizable;
        self
    }

    /// Sets the initial window title.
    ///
    /// Defaults to `"a nice window"`.
    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.title = title.into();
        self
    }

    /// Sets whether the window controls and title bar initially have a right-to-left layout.
    ///
    /// Defaults to `false`.
    pub fn right_to_left(mut self, right_to_left: bool) -> Self {
        self.style.right_to_left = right_to_left;
        self
    }

    /// Sets whether the window is initially visible to the user.
    ///
    /// Defaults to `true`.
    pub fn visible(mut self, visible: bool) -> Self {
        self.style.visible = visible;
        self
    }
}
