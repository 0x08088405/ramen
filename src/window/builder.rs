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
    pub(crate) maximised: bool,
    pub(crate) position: Option<(i16, i16)>,
    pub(crate) size: (u16, u16),
    pub(crate) style: Style,
    pub(crate) title: Cow<'static, str>,
}

impl Builder {
    pub(crate) const fn new(connection: Connection, style: Option<Style>) -> Self {
        Builder {
            connection,
            class_name: Cow::Borrowed("ramen_window"),
            maximised: false,
            position: None,
            size: (800, 600),
            style: match style {
                // Why is `Option::unwrap_or` not const fn?!
                Some(style) => style,
                None => Style::new(),
            },
            title: Cow::Borrowed("a nice window"),
        }
    }

    /// Attempt to build a Window, consuming this Builder object.
    pub fn build(self) -> Result<super::Window, Error> {
        imp::Window::new(self).map(super::Window)
    }

    /// Sets whether the window should be borderless.
    /// 
    /// Defaults to `false`.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.style.borderless = borderless;
        self
    }

    /// Specifies the control buttons this window should have.
    /// 
    /// Defaults to `None`, meaning the controls will be decided by the operating system.
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

    /// Sets whether the window should begin maximised on the user's monitor.
    /// 
    /// If the user has multiple monitors, the primary monitor will usually be selected.
    /// 
    /// Defaults to `false`.
    pub fn maximised(mut self, maximised: bool) -> Self {
        self.maximised = maximised;
        self
    }

    /// Sets the position of the top-left of the window on the screen. If `None`, the operating system will decide
    /// where to place the window.
    /// 
    /// While it is necessary to pass a position to the X11 backend, it is unlikely to be used, since almost every
    /// X11-based Window Manager will ignore this and use its own positioning logic. If the Window Manager places
    /// the window in a different place, you will receive a `Move` event.
    /// 
    /// Defaults to `None`.
    pub fn position(mut self, position: Option<(i16, i16)>) -> Self {
        self.position = position;
        self
    }

    /// Sets the size of the window.
    /// 
    /// Defaults to (800, 600).
    pub fn size(mut self, size: (u16, u16)) -> Self {
        self.size = size;
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
