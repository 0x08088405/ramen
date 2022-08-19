const _XCB_KEY_PRESS: u8 = 2;
const _XCB_KEY_RELEASE: u8 = 3;
const _XCB_BUTTON_PRESS: u8 = 4;
const _XCB_BUTTON_RELEASE: u8 = 5;
const XCB_FOCUS_IN: u8 = 9;
const XCB_FOCUS_OUT: u8 = 10;
const XCB_CLIENT_MESSAGE: u8 = 33;

#[derive(Clone, Copy)]
pub(crate) enum Event {
    ClientMessage {
        format: u8,
        window: super::XcbWindow,
        r#type: super::XcbAtom,
        client_data: ClientData,
    },
    Focus {
        window: super::XcbWindow,
        state: bool,
    },
}

impl Event {
    pub(super) fn from_generic(ev: *mut XcbGenericEvent) -> Option<Self> {
        unsafe {
            let ev_type = (*ev).response_type & !0x80;
            match ev_type {
                XCB_CLIENT_MESSAGE => {
                    let ev: *mut XcbClientMessage = ev.cast();
                    Some(Self::ClientMessage { format: (*ev).format, window: (*ev).window, r#type: (*ev).r#type, client_data: (*ev).client_data })
                },
                XCB_FOCUS_IN => {
                    let ev: *mut XcbFocusEvent = ev.cast();
                    Some(Self::Focus { window: (*ev).event, state: true })
                },
                XCB_FOCUS_OUT => {
                    let ev: *mut XcbFocusEvent = ev.cast();
                    Some(Self::Focus { window: (*ev).event, state: false })
                },
                _ => None,
            }
        }
    }
}

#[repr(C)]
pub(super) struct XcbGenericEvent {
    pub(super) response_type: u8,
    pub(super) _pad0: u8,
    pub(super) sequence: u16,
    pub(super) _pad: [u32; 7],
    pub(super) full_sequence: u32,
}

#[repr(C)]
pub(super) struct XcbAtomReply {
    pub(super) response_type: u8,
    pub(super) _pad0: u8,
    pub(super) sequence: u16,
    pub(super) length: u32,
    pub(super) atom: super::XcbAtom,
}

#[repr(C)]
pub(super) struct XcbClientMessage {
    pub(super) response_type: u8,
    pub(super) format: u8,
    pub(super) sequence: u16,
    pub(super) window: super::XcbWindow,
    pub(super) r#type: super::XcbAtom,
    pub(super) client_data: ClientData,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union ClientData {
    pub(crate) data8: [u8; 20],
    pub(crate) data16: [u16; 10],
    pub(crate) data32: [u32; 5],
}

#[repr(C)]
#[derive(Debug)]
struct XcbButtonEvent {
    response_type: u8,
    detail: u8,
    sequence: u16,
    time: u32,
    root: super::XcbWindow,
    event: super::XcbWindow,
    child: super::XcbWindow,
    root_x: i16,
    root_y: i16,
    event_x: i16,
    event_y: i16,
    state: u16,
    same_screen: u8,
    _pad: u8,
}

#[repr(C)]
#[derive(Debug)]
struct XcbFocusEvent {
    response_type: u8,
    send_event: u8,
    sequence: u16,
    event: super::XcbWindow,
    mode: u8,
    _pad0: [u8; 3],
}
