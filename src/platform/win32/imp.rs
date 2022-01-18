//! TODO explain rationale behind creating a thread for this

use super::ffi::*;

use crate::{
    error::Error,
    event::{CloseReason, Event},
    util::{TryPush, sync::{self, Condvar, Mutex}},
    window,
};

use std::{cell::UnsafeCell, hint, mem, ptr, sync::{atomic::{self, AtomicBool}, Arc}, thread};

/// Marker for identifying windows instantiated by ramen.
/// This is necessary because some software likes to inject windows on your thread that you didn't create.
/// Identifying a window is done by storing this value in the tail allocation of the window class struct,
/// which is guaranteed to exist for our windows with `WNDCLASSEXW->cbClsExtra` being exactly usize-sized.
/// On why it's a `usize` of all things, see the remarks on `set_class_storage`.
const RAMEN_WINDOW_MARKER: usize = u32::from_le_bytes(*b"viri") as usize;

/// Retrieves the base module [`HINSTANCE`].
#[inline]
pub fn base_hinstance() -> HINSTANCE {
    extern "system" {
        // Microsoft's linkers provide a static `HINSTANCE` to not have to query it at runtime.
        // Doing it this way is also much more predictable for dynamic and static linked libraries.
        // See: https://devblogs.microsoft.com/oldnewthing/20041025-00/?p=37483
        static __ImageBase: IMAGE_DOS_HEADER;
    }
    (unsafe { &__ImageBase }) as *const IMAGE_DOS_HEADER as HINSTANCE
}

/// Converts a &str to a Windows wide-string (for `LPCWSTR` parameters).
///
/// If the length is 0 then no allocation is made, you get a pointer to static memory.
fn str_to_wstr(src: &str, buffer: &mut Vec<WCHAR>) -> Option<*const WCHAR> {
    // MultiByteToWideChar can't actually handle zero-length because 0 means error.
    // The boundaries of `c_int` are checked here too since it's cast below a few times.
    if src.is_empty() || src.len() >= c_int::max_value() as usize {
        return Some([0x00].as_ptr())
    }
    let src_len = src.len() as c_int;

    unsafe {
        // calculate buffer size
        // +1 for null terminator (that we add ourselves)
        let req_buffer_size = MultiByteToWideChar(CP_UTF8, 0, src.as_ptr().cast(), src_len, ptr::null_mut(), 0) + 1;

        // ensure buffer capacity
        buffer.clear();
        buffer.try_reserve(req_buffer_size as usize).ok()?;

        // write to destination buffer
        let chars_written =
            MultiByteToWideChar(CP_UTF8, 0, src.as_ptr().cast(), src_len, buffer.as_mut_ptr(), req_buffer_size)
                as usize;

        // append null terminator
        *buffer.as_mut_ptr().add(chars_written) = 0x00;

        // mark buffer as initialised
        buffer.set_len(req_buffer_size as usize);
        Some(buffer.as_ptr())
    }
}

/// Turns a `Style` into a dwStyle and dwExStyle.
/// This does not include the close button, see `set_close_button`.
fn style_to_bits(style: &window::Style) -> (DWORD, DWORD) {
    let window::Style {
        borderless,
        controls,
        resizable,
        right_to_left,
        visible,
    } = *style;

    let (mut style, mut style_ex) = (0, 0);

    // TODO Why does this need THICKFRAME to work? Very strange.
    if borderless {
        style |= WS_POPUP | WS_THICKFRAME;
    } else {
        style |= WS_OVERLAPPED | WS_BORDER | WS_CAPTION;
    }
    if let Some(controls) = controls {
        if controls.minimise {
            style |= WS_MINIMIZEBOX;
        }
        if controls.maximise {
            style |= WS_MAXIMIZEBOX;
        }
        style |= WS_SYSMENU;
    }
    if resizable {
        style |= WS_THICKFRAME;
    }
    if right_to_left {
        style_ex |= WS_EX_LAYOUTRTL;
    }
    if visible {
        style |= WS_VISIBLE;
    }

    (style, style_ex)
}

/// Due to legacy reasons, the close button is a system menu item and not a window style.
unsafe fn set_close_button(hwnd: HWND, enabled: bool) {
    let menu: HMENU = GetSystemMenu(hwnd, FALSE);
    let flag = if enabled {
        MF_BYCOMMAND | MF_ENABLED
    } else {
        MF_BYCOMMAND | MF_DISABLED | MF_GRAYED
    };
    let _ = EnableMenuItem(menu, SC_CLOSE as UINT, flag);
}

pub(crate) struct Window {
    hwnd: HWND,
    state: *mut WindowState, // 'thread, volatile
    thread: Option<thread::JoinHandle<()>>,
}
unsafe impl Send for Window {}
unsafe impl Sync for Window {}

/// Parameters sent to `WM_NCCREATE` and `WM_CREATE`.
struct WindowCreateParams {
    state: *mut WindowState,
}

/// Volatile state which the `Window` and its thread both have a pointer to.
struct WindowState {
    close_reason: Option<CloseReason>,
    destroy: AtomicBool, // see remarks on `cbt_hookproc`, `WM_DESTROY`
    event_backbuf: Vec<Event>,
    event_frontbuf: Vec<Event>,
    event_sync: (Condvar, Mutex<bool>), // bool is "is_blocking", see `WindowState::dispatch_event`
    qpc_counts_per_sec: u64,
    qpc_countdown: u64,
    qpc_previous: u64,
}

unsafe fn make_window(builder: &window::Builder) -> Result<Window, Error> {
    // A window class describes the default state of a window, more or less.
    // It needs to be registered to the system-global table if it has not been.
    let mut class = mem::MaybeUninit::<WNDCLASSEXW>::uninit();
    let class_ptr = class.as_mut_ptr();
    (*class_ptr).cbSize = mem::size_of_val(&class) as UINT;

    // The most convenient way to identify a window class is by its name.
    // There is a handle system (ATOMs), but you can't do a reverse lookup with the string.
    // For more info, read up on Windows's Atom Tables. Class names are in the User Atom Table.
    let mut class_name_wstr = Vec::<WCHAR>::new();
    let class_name = str_to_wstr(&*builder.class_name, class_name_wstr.as_mut())
        .ok_or(Error::OutOfMemory)?;

    // Check if it's been registered by trying to query information about the class.
    // If it hasn't been, fill in the info and register it.
    if GetClassInfoExW(base_hinstance(), class_name, class_ptr) == 0 {
        // Failure sets a global error code, but we don't care, we know the error
        SetLastError(ERROR_SUCCESS);

        // Fill in the default state and register the class.
        let class = &mut *class_ptr;
        class.style = CS_OWNDC;
        class.lpfnWndProc = window_proc;
        class.hIcon = ptr::null_mut();
        class.hCursor = ptr::null_mut();
        class.hbrBackground = ptr::null_mut();
        class.lpszMenuName = ptr::null_mut();
        // TODO Filter reserved class names
        class.lpszClassName = class_name;
        class.hIconSm = ptr::null_mut();

        // See the remarks on `RAMEN_WINDOW_MARKER`
        class.cbClsExtra = mem::size_of::<usize>() as c_int;
        class.cbWndExtra = 0;

        // A handle to the executable base is given so the OS knows when to free it (if you do not).
        class.hInstance = base_hinstance();

        // Unlike what most libraries think, this is fallible, even if the input is valid.
        // It's quite trivial to fill up the (system-global) User Atom Table (2^16-1 entries) and OOM.
        if RegisterClassExW(class) == 0 {
            return Err(Error::SystemResources);
        }
    }

    let mut title_wstr = Vec::new();
    let _ = str_to_wstr(&*builder.title, &mut title_wstr).ok_or(Error::OutOfMemory)?;

    let style = builder.style;

    // Mechanism thingy
    let recv = Arc::new((Condvar::new(), Mutex::new(None)));
    let send = Arc::clone(&recv);

    // Time to create the window thread!
    let thread = thread::Builder::new().spawn(move || {
        let class_name = class_name_wstr;
        let send = send;
        let style = style;
        let title = title_wstr;

        // Attach the CBT hook for the current thread
        let thread_id = GetCurrentThreadId();
        let cbt_hook = SetWindowsHookExW(WH_CBT, cbt_hookproc, ptr::null_mut(), thread_id);
        assert!(!cbt_hook.is_null()); // TODO

        let (dw_style, dw_style_ex) = style_to_bits(&style);

        let mut qpc_counts_per_sec = 0;
        let _ = QueryPerformanceFrequency(&mut qpc_counts_per_sec);
        let mut qpc_previous = 0;
        let _ = QueryPerformanceCounter(&mut qpc_previous);
        let window_state = UnsafeCell::new(WindowState {
            close_reason: None,
            destroy: AtomicBool::new(false),
            event_backbuf: Vec::new(),
            event_frontbuf: Vec::new(),
            event_sync: (Condvar::new(), Mutex::new(false)),
            qpc_counts_per_sec,
            qpc_countdown: qpc_counts_per_sec,
            qpc_previous,
        });

        let create_params = WindowCreateParams {
            state: window_state.get(),
        };

        let hwnd = CreateWindowExW(
            dw_style_ex,
            if class_name.is_empty() { [0].as_ptr() } else { class_name.as_ptr() },
            if title.is_empty() { [0].as_ptr() } else { title.as_ptr() },
            dw_style,
            400, // x
            400, // y
            800, // w (nc)
            608, // h (nc)
            ptr::null_mut(),
            ptr::null_mut(),
            base_hinstance(),
            (&create_params) as *const WindowCreateParams as *mut c_void,
        );
        assert!(!hwnd.is_null());
        // TODO handle this ^

        let (cvar, mutex) = &*send;
        let mut lock = sync::mutex_lock(&mutex);
        *lock = Some(Ok(Window {
            hwnd,
            state: window_state.get(),
            thread: None,
        }));
        sync::cvar_notify_one(&cvar);
        mem::drop(lock);

        // This is considered a menu item, so it has to be updated after creating the window.
        set_close_button(hwnd, style.controls.as_ref().map(|x| x.close).unwrap_or(false));

        // Run message loop until error or exit
        let mut msg = mem::MaybeUninit::zeroed();
        'message_loop: loop {
            // `HWND hWnd` is set to NULL here to query all messages on the thread,
            // as the exit condition/signal `WM_QUIT` is not associated with any window.
            // This is one of the main motives (besides no blocking) to give each window a thread.
            match GetMessageW(msg.as_mut_ptr(), ptr::null_mut(), 0, 0) {
                -1 => panic!("Hard error {:#06X} in GetMessageW loop!", GetLastError()),
                0 => break 'message_loop,
                _ => {
                    // Dispatch message to `window_proc`
                    // NOTE: Some events call `window_proc` directly instead of through here
                    let _ = DispatchMessageW(msg.as_ptr());
                },
            }
        }

        mem::drop(window_state);
        let _ = UnhookWindowsHookEx(cbt_hook);
    }).map_err(|_| Error::SystemResources)?;

    // wait until thread gives us a result, yield value
    let (cvar, mutex) = &*recv;
    let mut lock = sync::mutex_lock(&mutex);
    loop {
        if let Some(result) = (&mut *lock).take() {
            break result.map(|mut window| {
                window.thread = Some(thread);
                window
            })
        } else {
            sync::cvar_wait(&cvar, &mut lock);
        }
    }
}

impl Window {
    pub(crate) fn new(builder: &window::Builder) -> Result<Self, Error> {
        unsafe { make_window(builder) }
    }

    pub(crate) fn events(&self) -> &[Event] {
        unsafe {
            // safety: `poll_events`'s signature invalidates this reference
            (&*self.state).event_frontbuf.as_slice()
        }
    }

    pub(crate) fn poll_events(&mut self) {
        unsafe {
            let state = &mut *self.state;
            let (cvar, mutex) = &state.event_sync;
            let mut is_blocking = sync::mutex_lock(mutex);
            if *is_blocking {
                *is_blocking = false;
                sync::cvar_notify_one(&cvar);
            }

            let mut qpc_counter = 0;
            let _ = QueryPerformanceCounter(&mut qpc_counter);
            state.qpc_countdown = state.qpc_counts_per_sec;
            state.qpc_previous = qpc_counter;

            state.event_frontbuf.clear();
            mem::swap(&mut state.event_frontbuf, &mut state.event_backbuf);
            mem::drop(is_blocking);
        }
    }
}

impl WindowState {
    fn dispatch_event(&mut self, event: Event) {
        let (cvar, mutex) = &self.event_sync;
        let mut is_blocking = sync::mutex_lock(mutex);
        let mut qpc_counter = 0;
        let _ = unsafe { QueryPerformanceCounter(&mut qpc_counter) };
        let qpc_difference = qpc_counter - self.qpc_previous;
        self.qpc_previous = qpc_counter;
        let timed_out = self.qpc_countdown < qpc_difference;
        if !timed_out {
            self.qpc_countdown -= qpc_difference;
        }
        loop {
            if *is_blocking || timed_out || self.event_backbuf._try_push(event).is_err() {
                *is_blocking = true;
                sync::cvar_wait(&cvar, &mut is_blocking);
            } else {
                break
            }
        }
    }
}

/// See remarks on `RAMEN_WINDOW_MARKER`.
unsafe fn is_ramen_window(hwnd: HWND) -> bool {
    class_storage(hwnd, GCL_CBCLSEXTRA) == mem::size_of::<usize>() && class_storage(hwnd, 0) == RAMEN_WINDOW_MARKER
}

/// Returns a pointer to the `WindowState` for a ramen window.
#[inline]
unsafe fn user_state(hwnd: HWND) -> *mut WindowState {
    instance_storage(hwnd, GWL_USERDATA) as *mut WindowState
}

/// Hook procedure for managing things that other bits of Win32 simply don't provide a way to do
unsafe extern "system" fn cbt_hookproc(code: c_int, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match code {
        // We need to uphold the invariant that as long as there's a `Window` there really is one.
        // There's no other way to say no to `DestroyWindow` calls than a CBT hook like this.
        HCBT_DESTROYWND => {
            let hwnd = wparam as HWND;
            if is_ramen_window(hwnd) {
                // 0 - allow, 1 - forbid (hence !)
                (!(&*user_state(hwnd)).destroy.load(atomic::Ordering::Acquire)) as LRESULT
            } else {
                CallNextHookEx(ptr::null_mut(), code, wparam, lparam)
            }
        },
        _ => CallNextHookEx(ptr::null_mut(), code, wparam, lparam),
    }
}

pub unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
   // println!("WindowProc ({:p}, {:#X}, {:#X} {:#X})", hwnd, msg, wparam, lparam);
    // Fantastic resource for a comprehensive list of window messages:
    // https://wiki.winehq.org/List_Of_Windows_Messages
    match msg {
        // No-op event, usable for pinging the event loop, stubbing, etc. Returns 0.
        WM_NULL => 0,

        // Received after the client area has been created, but before the window is made visible.
        // This event comes after `WM_NCCREATE`, the event sent after the non-client area is created.
        // wParam: Unused, ignore.
        // lParam: `CREATESTRUCTW *` (in)
        // Return 0 to succeed `CreateWindowExW`, or -1 to destroy the window and return NULL.
        // See also: `WM_NCCREATE`
        WM_CREATE => 0,

        // Received as the client area is being destroyed.
        // This event is received, then `WM_NCDESTROY`, and the window is gone after that.
        // Nothing can actually be done once this message is received, and you always return 0.
        WM_DESTROY => {
            // However, to uphold the invariant that as long as there's a `Window` there really is one,
            // we make sure this message was received due to the window being dropped,
            // and not from a silly third party program sneaking a message in there.
            if (&*user_state(hwnd)).destroy.load(atomic::Ordering::Acquire) {
                PostQuitMessage(0);
            }
            0
        },

        // TODO
        WM_MOVE => DefWindowProcW(hwnd, msg, wparam, lparam),

        // << Event 0x0004 is not known to exist. >>

        // TODO
        WM_SIZE => DefWindowProcW(hwnd, msg, wparam, lparam),

        WM_PAINT => {
            // windows will not stop spamming this event until you process it like this
            // we don't actually draw anything, of course
            let mut paintstruct = mem::MaybeUninit::<PAINTSTRUCT>::uninit();
            let _ = BeginPaint(hwnd, paintstruct.as_mut_ptr());
            let _ = EndPaint(hwnd, paintstruct.as_mut_ptr());
            0
        },

        // Received when a window is requested to close. Return 0.
        WM_CLOSE => {
            let state = &mut *user_state(hwnd);
            let reason = state.close_reason.take().unwrap_or(CloseReason::Unknown);
            state.dispatch_event(Event::CloseRequest(reason));
            0
        },

        WM_NCCREATE => {
            // `lpCreateParams` is the first field, so `CREATESTRUCTW *` is `WindowCreateParams **`
            let params = &**(lparam as *const *const WindowCreateParams);
            let _ = set_instance_storage(hwnd, GWL_USERDATA, params.state as usize);
            DefWindowProcW(hwnd, msg, wparam, lparam)
        },

        // Received when the user clicks a window menu control (formerly "system menu").
        // wParam: Command enum.
        // lParam: Mouse position.
        // Return 0.
        WM_SYSCOMMAND => {
            if wparam == SC_CLOSE {
                (&mut *user_state(hwnd)).close_reason = Some(CloseReason::SystemMenu);
            }
            DefWindowProcW(hwnd, msg, wparam, lparam)
        },

        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            let state = &*user_state(self.hwnd);
            state.destroy.store(true, atomic::Ordering::Release);
            if let Some(result) = self.thread.take().map(thread::JoinHandle::join) {
                if let Err(_err) = result {
                    // TODO: Not sure what to do here, honestly.
                    // Like it shouldn't panic, but what about GetMessageW?
                    // I guess remove that panic. It shouldn't fail, no?
                    panic!("window thread panicked");
                }
            } else {
                hint::unreachable_unchecked()
            }
        }
    }
}
