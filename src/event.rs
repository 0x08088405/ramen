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

    /// The window was resized.
    Resize((u16, u16)),

    /// A [`Key`] was pressed.
    /// 
    /// This event will tell you which physical key the user has pressed. If your aim is to process text input,
    /// consider using the [`Event::Input`] event type instead.
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

    /// A [`char`] was typed using the keyboard.
    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    Input(char),

    /// A [`MouseButton`] was pressed.
    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    MouseDown(MouseButton),

    /// A [`MouseButton`] was released.
    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    MouseUp(MouseButton),

    /// The mouse was moved.
    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    MouseMove((u16, u16)),

    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    MouseEnter,

    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    MouseLeave,
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
