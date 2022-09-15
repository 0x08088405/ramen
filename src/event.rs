#[cfg(feature = "input")]
use crate::input::{Key, MouseButton};

#[derive(Copy, Clone, Debug, Hash, PartialEq)]
#[non_exhaustive]
pub enum Event {
    /// A request has been made to close the window.
    /// For more information on why, see the associated [`CloseReason`].
    CloseRequest(CloseReason),

    /// The window focus state has been updated (`true` if focused).
    Focus(bool),

    /// A [`Key`] was pressed.
    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    KeyboardDown(Key),

    /// A [`Key`] was auto-repeated by the system.
    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    KeyboardRepeat(Key),

    /// A [`Key`] was released.
    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    KeyboardUp(Key),

    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    MouseDown(MouseButton),

    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    MouseUp(MouseButton),

    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    MouseMove((u16, u16)),
}

/// Details the source of [`Event::CloseRequest`].
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CloseReason {
    /// The user has pressed the system control for closing the window, typically an "X button".
    SystemMenu,

    /// The user has pressed a system keyboard shortcut to close the active window.
    KeyboardShortcut,

    /// The reason for the close request is unknown.
    Unknown,
}
