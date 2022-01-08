#[derive(Copy, Clone, Debug, Hash, PartialEq)]
#[non_exhaustive]
pub enum Event {
    /// A request has been made to close the window.
    /// For more information on why, see the associated [`CloseReason`].
    CloseRequest(CloseReason),
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
