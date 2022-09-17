#[cfg(feature = "input")]
use crate::input::{Key, MouseButton};

#[derive(Copy, Clone, Debug, Hash, PartialEq)]
#[non_exhaustive]
pub enum Event {
    /// A request has been made to close the window, most likely by clicking the 'x' button or by pressing alt+F4.
    /// This can be ignored - the window will not actually close until it is dropped.
    CloseRequest,

    /// The window focus state has been updated (`true` if focused).
    Focus(bool),

    /// The window was maximised, possibly in response to `set_maximised()`.
    Maximise(bool),

    /// The window was minimised.
    Minimise(bool),

    /// The window was moved.
    Move((i16, i16)),

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

    /// The mouse has entered the client area of the window.
    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    MouseEnter,

    /// The mouse is no longer in the client area of the window.
    #[cfg_attr(feature = "nightly-rustdoc", doc(cfg(feature = "input")))]
    #[cfg_attr(not(feature = "nightly-rustdoc"), cfg(feature = "input"))]
    MouseLeave,
}
