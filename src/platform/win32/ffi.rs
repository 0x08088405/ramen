#![allow(nonstandard_style, overflowing_literals)]
// TODO
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
#![allow(dead_code)] // <------------
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// THIS NEEDS TO BE REMOVED BEFORE RELEASE
// Don't you dare forget...

// C Typedefs
pub(crate) use std::os::raw::{
    c_void,
    c_uchar, c_char,
    c_ushort, c_short,
    c_uint, c_int,
    c_ulong, c_long,
    c_ulonglong, c_longlong,
};
pub(crate) type wchar_t = u16;

// Windows Typedefs
macro_rules! opaque {
    ($( $(#[$attr:meta])* $v:vis $name:ident = $void:ident ),+ $(,)?) => {
        $(
            $(#[$attr])* $v enum $void {}
            $(#[$attr])* $v type $name = *mut $void;
        )+
    };
}
opaque! {
    pub(crate) FARPROC = __some_function,
    pub(crate) HBRUSH = HBRUSH__,
    pub(crate) HDC = HDC__,
    pub(crate) HHOOK = HHOOK__,
    pub(crate) HICON = HICON__,
    pub(crate) HMENU = HMENU__,
    pub(crate) HMODULE = HMODULE__,

    /// Opaque handle to a module in memory.
    pub HINSTANCE = HINSTANCE__,

    /// Opaque handle to a window.
    pub HWND = HWND__,
}
pub(crate) type ATOM = WORD;
pub(crate) type BOOL = c_int;
pub(crate) type BYTE = c_uchar;
pub(crate) type CHAR = c_char;
pub(crate) enum DPI_AWARENESS_CONTEXT__ {}
pub(crate) type DPI_AWARENESS_CONTEXT = *mut DPI_AWARENESS_CONTEXT__;
pub(crate) type DWORD = c_ulong;
pub(crate) type HANDLE = *mut c_void;
pub(crate) type HCURSOR = HICON;
pub(crate) type HLOCAL = HANDLE;
pub(crate) type HOOKPROC = unsafe extern "system" fn(c_int, WPARAM, LPARAM) -> LRESULT;
pub(crate) type HRESULT = c_long;
pub(crate) type INT = c_int;
pub(crate) type LANGID = USHORT;
pub(crate) type LONG = c_long;
pub(crate) type LONG_PTR = isize;
pub(crate) type LPARAM = LONG_PTR;
pub(crate) type LRESULT = LONG_PTR;
pub(crate) type NTSTATUS = LONG;
pub(crate) type PROCESS_DPI_AWARENESS = u32;
pub(crate) type SHORT = c_short;
pub(crate) type UINT = c_uint;
pub(crate) type UINT_PTR = usize;
pub(crate) type ULONG_PTR = usize;
pub(crate) type USHORT = c_ushort;
pub(crate) type WCHAR = wchar_t;
pub(crate) type WORD = c_ushort;
pub(crate) type WPARAM = UINT_PTR;
/// A user-defined application window callback procedure.
pub type WNDPROC = unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT;

// Constants
pub(crate) const _WIN32_WINNT_VISTA: WORD = 0x0600;
pub(crate) const _WIN32_WINNT_WINBLUE: WORD = 0x0603;
pub(crate) const CCHILDREN_TITLEBAR: usize = 5;
pub(crate) const CP_UTF8: DWORD = 65001;
pub(crate) const CS_OWNDC: UINT = 0x0020;
pub(crate) const CW_USEDEFAULT: c_int = 0x80000000;
pub(crate) const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = -4isize as _;
pub(crate) const ERROR_SUCCESS: DWORD = 0; // lol
pub(crate) const FALSE: BOOL = 0;
pub(crate) const GCL_CBCLSEXTRA: c_int = -20;
pub(crate) const GWL_EXSTYLE: c_int = -20;
pub(crate) const GWL_STYLE: c_int = -16;
pub(crate) const GWL_USERDATA: c_int = -21;
pub(crate) const HCBT_DESTROYWND: c_int = 4;
pub(crate) const HTCAPTION: LRESULT = 2;
pub(crate) const IDC_APPSTARTING: *const WCHAR = 32650 as _;
pub(crate) const IDC_ARROW: *const WCHAR = 32512 as _;
pub(crate) const IDC_CROSS: *const WCHAR = 32515 as _;
pub(crate) const IDC_HAND: *const WCHAR = 32649 as _;
pub(crate) const IDC_HELP: *const WCHAR = 32651 as _;
pub(crate) const IDC_IBEAM: *const WCHAR = 32513 as _;
pub(crate) const IDC_ICON: *const WCHAR = 32641 as _;
pub(crate) const IDC_NO: *const WCHAR = 32648 as _;
pub(crate) const IDC_SIZE: *const WCHAR = 32640 as _;
pub(crate) const IDC_SIZEALL: *const WCHAR = 32646 as _;
pub(crate) const IDC_SIZENESW: *const WCHAR = 32643 as _;
pub(crate) const IDC_SIZENS: *const WCHAR = 32645 as _;
pub(crate) const IDC_SIZENWSE: *const WCHAR = 32642 as _;
pub(crate) const IDC_SIZEWE: *const WCHAR = 32644 as _;
pub(crate) const IDC_UPARROW: *const WCHAR = 32516 as _;
pub(crate) const IDC_WAIT: *const WCHAR = 32514 as _;
pub(crate) const IMAGE_CURSOR: UINT = 2;
pub(crate) const HTCLIENT: LRESULT = 1;
pub(crate) const LR_DEFAULTSIZE: UINT = 0x00000040;
pub(crate) const LR_SHARED: UINT = 0x00008000;
pub(crate) const MF_BYCOMMAND: UINT = 0x00000000;
pub(crate) const MF_DISABLED: UINT = 0x00000002;
pub(crate) const MF_ENABLED: UINT = 0x00000000;
pub(crate) const MF_GRAYED: UINT = 0x00000001;
pub(crate) const PROCESS_PER_MONITOR_DPI_AWARE: PROCESS_DPI_AWARENESS = 2;
pub(crate) const PROCESS_SYSTEM_DPI_AWARE: PROCESS_DPI_AWARENESS = 1;
pub(crate) const SC_CLOSE: WPARAM = 0xF060;
pub(crate) const SC_MAXIMIZE: WPARAM = 0xF030;
pub(crate) const SC_RESTORE: WPARAM = 0xF120;
pub(crate) const SIZE_RESTORED: WPARAM = 0;
pub(crate) const SIZE_MINIMIZED: WPARAM = 1;
pub(crate) const SIZE_MAXIMIZED: WPARAM = 2;
pub(crate) const SIZE_MAXSHOW: WPARAM = 3;
pub(crate) const SIZE_MAXHIDE: WPARAM = 4;
pub(crate) const SW_HIDE: c_int = 0;
pub(crate) const SW_SHOW: c_int = 5;
pub(crate) const SWP_ASYNCWINDOWPOS: UINT = 0x4000;
pub(crate) const SWP_DEFERERASE: UINT = 0x2000;
pub(crate) const SWP_DRAWFRAME: UINT = SWP_FRAMECHANGED;
pub(crate) const SWP_FRAMECHANGED: UINT = 0x0020;
pub(crate) const SWP_HIDEWINDOW: UINT = 0x0080;
pub(crate) const SWP_NOACTIVATE: UINT = 0x0010;
pub(crate) const SWP_NOCOPYBITS: UINT = 0x0100;
pub(crate) const SWP_NOMOVE: UINT = 0x0002;
pub(crate) const SWP_NOOWNERZORDER: UINT = 0x0200;
pub(crate) const SWP_NOREDRAW: UINT = 0x0008;
pub(crate) const SWP_NOREPOSITION: UINT = SWP_NOOWNERZORDER;
pub(crate) const SWP_NOSENDCHANGING: UINT = 0x0400;
pub(crate) const SWP_NOSIZE: UINT = 0x0001;
pub(crate) const SWP_NOZORDER: UINT = 0x0004;
pub(crate) const SWP_SHOWWINDOW: UINT = 0x0040;
pub(crate) const TRUE: BOOL = 1;
pub(crate) const VER_BUILDNUMBER: DWORD = 0x0000004;
pub(crate) const VER_GREATER_EQUAL: BYTE = 3;
pub(crate) const VER_MAJORVERSION: DWORD = 0x0000002;
pub(crate) const VER_MINORVERSION: DWORD = 0x0000001;
pub(crate) const VER_SERVICEPACKMAJOR: DWORD = 0x0000020;
pub(crate) const VER_SERVICEPACKMINOR: DWORD = 0x0000010;
pub(crate) const WH_CBT: c_int = 5;
pub(crate) const WM_NULL: UINT = 0x0000;
pub(crate) const WM_CREATE: UINT = 0x0001;
pub(crate) const WM_DESTROY: UINT = 0x0002;
pub(crate) const WM_MOVE: UINT = 0x0003;
pub(crate) const WM_SIZE: UINT = 0x0005;
pub(crate) const WM_ACTIVATE: UINT = 0x0006;
pub(crate) const WM_SETFOCUS: UINT = 0x0007;
pub(crate) const WM_KILLFOCUS: UINT = 0x0008;
pub(crate) const WM_ENABLE: UINT = 0x000A;
pub(crate) const WM_SETREDRAW: UINT = 0x000B;
pub(crate) const WM_SETTEXT: UINT = 0x000C;
pub(crate) const WM_PAINT: UINT = 0x000F;
pub(crate) const WM_CLOSE: UINT = 0x0010;
pub(crate) const WM_ERASEBKGND: UINT = 0x0014;
pub(crate) const WM_SHOWWINDOW: UINT = 0x0018;
pub(crate) const WM_ACTIVATEAPP: UINT = 0x001C;
pub(crate) const WM_SETCURSOR: UINT = 0x0020;
pub(crate) const WM_NCCREATE: UINT = 0x0081;
pub(crate) const WM_NCDESTROY: UINT = 0x0082;
pub(crate) const WM_NCLBUTTONDOWN: UINT = 0x00A1;
pub(crate) const WM_KEYDOWN: UINT = 0x0100;
pub(crate) const WM_KEYUP: UINT = 0x0101;
pub(crate) const WM_SYSKEYDOWN: UINT = 0x0104;
pub(crate) const WM_SYSKEYUP: UINT = 0x0105;
pub(crate) const WM_SYSCOMMAND: UINT = 0x0112;
pub(crate) const WM_MOUSEMOVE: UINT = 0x0200;
pub(crate) const WM_LBUTTONDOWN: UINT = 0x0201;
pub(crate) const WM_LBUTTONUP: UINT = 0x0202;
pub(crate) const WM_RBUTTONDOWN: UINT = 0x0204;
pub(crate) const WM_RBUTTONUP: UINT = 0x0205;
pub(crate) const WM_MBUTTONDOWN: UINT = 0x0207;
pub(crate) const WM_MBUTTONUP: UINT = 0x0208;
pub(crate) const WM_MOUSEWHEEL: UINT = 0x020A;
pub(crate) const WM_XBUTTONDOWN: UINT = 0x020B;
pub(crate) const WM_XBUTTONUP: UINT = 0x020C;
pub(crate) const WM_MOVING: UINT = 0x0216;
pub(crate) const WM_EXITSIZEMOVE: UINT = 0x0232;
pub(crate) const WM_USER: UINT = 0x0400;
pub(crate) const WS_BORDER: DWORD = 0x00800000;
pub(crate) const WS_CAPTION: DWORD = 0x00C00000;
pub(crate) const WS_CHILD: DWORD = 0x40000000;
pub(crate) const WS_CLIPCHILDREN: DWORD = 0x02000000;
pub(crate) const WS_CLIPSIBLINGS: DWORD = 0x04000000;
pub(crate) const WS_DISABLED: DWORD = 0x08000000;
pub(crate) const WS_DLGFRAME: DWORD = 0x00400000;
pub(crate) const WS_EX_LAYOUTRTL: DWORD = 0x00400000;
pub(crate) const WS_EX_TOOLWINDOW: DWORD = 0x00000080;
pub(crate) const WS_GROUP: DWORD = 0x00020000;
pub(crate) const WS_HSCROLL: DWORD = 0x00100000;
pub(crate) const WS_ICONIC: DWORD = WS_MINIMIZE;
pub(crate) const WS_MAXIMIZE: DWORD = 0x01000000;
pub(crate) const WS_MAXIMIZEBOX: DWORD = 0x00010000;
pub(crate) const WS_MINIMIZE: DWORD = 0x20000000;
pub(crate) const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub(crate) const WS_OVERLAPPED: DWORD = 0x00000000;
pub(crate) const WS_OVERLAPPEDWINDOW: DWORD =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub(crate) const WS_POPUP: DWORD = 0x80000000;
pub(crate) const WS_SIZEBOX: DWORD = WS_THICKFRAME;
pub(crate) const WS_SYSMENU: DWORD = 0x00080000;
pub(crate) const WS_TABSTOP: DWORD = 0x00010000;
pub(crate) const WS_THICKFRAME: DWORD = 0x00040000;
pub(crate) const WS_TILED: DWORD = WS_OVERLAPPED;
pub(crate) const WS_TILEDWINDOW: DWORD = WS_OVERLAPPEDWINDOW;
pub(crate) const WS_VISIBLE: DWORD = 0x10000000;
pub(crate) const WS_VSCROLL: DWORD = 0x00200000;
pub(crate) const XBUTTON1: WORD = 0x0001;
pub(crate) const XBUTTON2: WORD = 0x0002;

// Structs
#[repr(C)]
pub(crate) struct IMAGE_DOS_HEADER {
    pub(crate) e_magic: WORD,
    pub(crate) e_cblp: WORD,
    pub(crate) e_cp: WORD,
    pub(crate) e_crlc: WORD,
    pub(crate) e_cparhdr: WORD,
    pub(crate) e_minalloc: WORD,
    pub(crate) e_maxalloc: WORD,
    pub(crate) e_ss: WORD,
    pub(crate) e_sp: WORD,
    pub(crate) e_csum: WORD,
    pub(crate) e_ip: WORD,
    pub(crate) e_cs: WORD,
    pub(crate) e_lfarlc: WORD,
    pub(crate) e_ovno: WORD,
    pub(crate) e_res: [WORD; 4],
    pub(crate) e_oemid: WORD,
    pub(crate) e_oeminfo: WORD,
    pub(crate) e_res2: [WORD; 10],
    pub(crate) e_lfanew: LONG,
}
#[repr(C)]
pub(crate) struct MSG {
    pub(crate) hwnd: HWND,
    pub(crate) message: UINT,
    pub(crate) wParam: WPARAM,
    pub(crate) lParam: LPARAM,
    pub(crate) time: DWORD,
    pub(crate) pt: POINT,
}
#[repr(C)]
pub(crate) struct POINT {
    pub(crate) x: LONG,
    pub(crate) y: LONG,
}
#[repr(C)]
pub(crate) struct RECT {
    pub(crate) left: LONG,
    pub(crate) top: LONG,
    pub(crate) right: LONG,
    pub(crate) bottom: LONG,
}
#[repr(C)]
pub(crate) struct TITLEBARINFO {
    pub(crate) cbSize: DWORD,
    pub(crate) rcTitleBar: RECT,
    pub(crate) rgstate: [DWORD; CCHILDREN_TITLEBAR + 1],
}
#[repr(C)]
pub(crate) struct WNDCLASSEXW {
    pub(crate) cbSize: UINT,
    pub(crate) style: UINT,
    pub(crate) lpfnWndProc: WNDPROC,
    pub(crate) cbClsExtra: c_int,
    pub(crate) cbWndExtra: c_int,
    pub(crate) hInstance: HINSTANCE,
    pub(crate) hIcon: HICON,
    pub(crate) hCursor: HCURSOR,
    pub(crate) hbrBackground: HBRUSH,
    pub(crate) lpszMenuName: *const WCHAR,
    pub(crate) lpszClassName: *const WCHAR,
    pub(crate) hIconSm: HICON,
}

// Static-linked Functions
#[link(name = "Kernel32")]
extern "system" {
    // Global state error code API mess
    pub(crate) fn GetLastError() -> DWORD;
    pub(crate) fn SetLastError(dwErrCode: DWORD);
    pub(crate) fn ExitProcess(uExitCode: UINT);

    // Threading
    pub(crate) fn GetCurrentThreadId() -> DWORD;

    // String conversion
    pub(crate) fn MultiByteToWideChar(
        CodePage: UINT,
        dwFlags: DWORD,
        lpMultiByteStr: *const CHAR,
        cbMultiByte: c_int,
        lpWideCharStr: *mut WCHAR,
        cchWideChar: c_int,
    ) -> c_int;

    // Dynamic linking
    pub(crate) fn GetProcAddress(hModule: HMODULE, lpProcName: *const CHAR) -> FARPROC;
    pub(crate) fn LoadLibraryExW(lpLibFileName: *const WCHAR, hFile: HANDLE, dwFlags: DWORD) -> HMODULE;

    // Operating system version
    pub(crate) fn VerSetConditionMask(ConditionMask: c_ulonglong, TypeMask: DWORD, Condition: BYTE) -> c_ulonglong;
}
#[link(name = "User32")]
extern "system" {
    // Window class management
    pub(crate) fn GetClassInfoExW(hinst: HINSTANCE, lpszClass: *const WCHAR, lpwcx: *mut WNDCLASSEXW) -> BOOL;
    pub(crate) fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> ATOM;

    // Window management
    pub(crate) fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: *const WCHAR,
        lpWindowName: *const WCHAR,
        dwStyle: DWORD,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: *mut c_void,
    ) -> HWND;
    pub(crate) fn AdjustWindowRectEx(lpRect: *mut RECT, dwStyle: DWORD, bMenu: BOOL, dwExStyle: DWORD) -> BOOL;
    pub(crate) fn ClientToScreen(hWnd: HWND, lpPoint: *mut POINT) -> BOOL;
    pub(crate) fn GetClientRect(hWnd: HWND, lpRect: *mut RECT) -> BOOL;
    pub(crate) fn GetWindowRect(hWnd: HWND, lpRect: *mut RECT) -> BOOL;
    pub(crate) fn GetTitleBarInfo(hwnd: HWND, pti: *mut TITLEBARINFO) -> BOOL;
    pub(crate) fn SetWindowPos(
        hWnd: HWND,
        hWndInsertAfter: HWND,
        X: c_int,
        Y: c_int,
        cx: c_int,
        cy: c_int,
        uFlags: UINT,
    ) -> BOOL;
    pub(crate) fn WindowFromPoint(Point: POINT) -> HWND;
    pub(crate) fn DestroyWindow(hWnd: HWND) -> BOOL;

    // Hooking API
    pub(crate) fn CallNextHookEx(hhk: HHOOK, nCode: c_int, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub(crate) fn SetWindowsHookExW(idHook: c_int, lpfn: HOOKPROC, hmod: HINSTANCE, dwThreadId: DWORD) -> HHOOK;
    pub(crate) fn UnhookWindowsHookEx(hhk: HHOOK) -> BOOL;

    // Message loop
    pub(crate) fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub(crate) fn GetMessageW(lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    pub(crate) fn PostMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
    pub(crate) fn SendMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub(crate) fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;
    pub(crate) fn PostQuitMessage(nExitCode: c_int);

    // Message loop utility
    pub(crate) fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    pub(crate) fn ShowWindowAsync(hWnd: HWND, nCmdShow: c_int) -> BOOL;

    // Keyboard & mouse related
    pub(crate) fn ClipCursor(lpRect: *const RECT) -> BOOL;
    pub(crate) fn SetCursorPos(X: c_int, Y: c_int) -> BOOL;
    pub(crate) fn GetCursorPos(lpPoint: *mut POINT) -> BOOL;
    pub(crate) fn SetCapture(hWnd: HWND) -> HWND;
    pub(crate) fn GetCapture() -> HWND;
    pub(crate) fn ReleaseCapture() -> BOOL;
    pub(crate) fn GetAsyncKeyState(vKey: c_int) -> SHORT;
    pub(crate) fn GetSystemMetrics(nIndex: c_int) -> c_int;
    pub(crate) fn SetCursor(hCursor: HCURSOR) -> HCURSOR;

    // Misc legacy garbage
    pub(crate) fn EnableMenuItem(hMenu: HMENU, uIDEnableItem: UINT, uEnable: UINT) -> BOOL;
    pub(crate) fn GetSystemMenu(hWnd: HWND, bRevert: BOOL) -> HMENU;

    // Yeah, whatever
    pub(crate) fn LoadImageW(
        hInst: HINSTANCE,
        name: *const WCHAR,
        type_: UINT,
        cx: c_int,
        cy: c_int,
        fuLoad: UINT,
    ) -> HANDLE;

    // Class/instance storage manipulation
    #[cfg(target_pointer_width = "32")]
    pub(crate) fn GetClassLongW(hWnd: HWND, nIndex: c_int) -> DWORD;
    #[cfg(target_pointer_width = "32")]
    pub(crate) fn SetClassLongW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> DWORD;
    #[cfg(target_pointer_width = "32")]
    pub(crate) fn GetWindowLongW(hWnd: HWND, nIndex: c_int) -> LONG;
    #[cfg(target_pointer_width = "32")]
    pub(crate) fn SetWindowLongW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> LONG;
    #[cfg(target_pointer_width = "64")]
    pub(crate) fn GetClassLongPtrW(hWnd: HWND, nIndex: c_int) -> ULONG_PTR;
    #[cfg(target_pointer_width = "64")]
    pub(crate) fn SetClassLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> ULONG_PTR;
    #[cfg(target_pointer_width = "64")]
    pub(crate) fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
    #[cfg(target_pointer_width = "64")]
    pub(crate) fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;
}

// Wrapper for the API that manipulates window class and instance storage.
//
// History lesson: `SetClassLongW` and friends all took `LONG`, a 32-bit type.
// Only, when Micro$oft upgraded from 32 to 64 bit, they realised pointers needed to fit into them.
// They added a new set of functions, `LongPtr` taking `LONG_PTR`, but *only available for 64-bit*.
// Their solution for using those functions on 32-bit was to define C preprocessor macros, like so:
// #define SetClassLongPtrW SetClassLongW
// The problem is that the signatures are incompatible in a language with a good integer type system.
// These functions wrap both function sets to `usize`, which lines up with the intended sizes.
#[cfg(target_pointer_width = "32")]
#[inline]
pub(crate) unsafe fn class_storage(hwnd: HWND, offset: c_int) -> usize {
    GetClassLongW(hwnd, offset) as usize
}
#[cfg(target_pointer_width = "64")]
#[inline]
pub(crate) unsafe fn class_storage(hwnd: HWND, offset: c_int) -> usize {
    GetClassLongPtrW(hwnd, offset) as usize
}
#[cfg(target_pointer_width = "32")]
#[inline]
pub(crate) unsafe fn set_class_storage(hwnd: HWND, offset: c_int, data: usize) -> usize {
    SetClassLongW(hwnd, offset, data as LONG) as usize
}
#[cfg(target_pointer_width = "64")]
#[inline]
pub(crate) unsafe fn set_class_storage(hwnd: HWND, offset: c_int, data: usize) -> usize {
    SetClassLongPtrW(hwnd, offset, data as LONG_PTR) as usize
}
#[cfg(target_pointer_width = "32")]
#[inline]
pub(crate) unsafe fn instance_storage(hwnd: HWND, offset: c_int) -> usize {
    GetWindowLongW(hwnd, offset) as usize
}
#[cfg(target_pointer_width = "64")]
#[inline]
pub(crate) unsafe fn instance_storage(hwnd: HWND, offset: c_int) -> usize {
    GetWindowLongPtrW(hwnd, offset) as usize
}
#[cfg(target_pointer_width = "32")]
#[inline]
pub(crate) unsafe fn set_instance_storage(hwnd: HWND, offset: c_int, data: usize) -> usize {
    SetWindowLongW(hwnd, offset, data as LONG) as usize
}
#[cfg(target_pointer_width = "64")]
#[inline]
pub(crate) unsafe fn set_instance_storage(hwnd: HWND, offset: c_int, data: usize) -> usize {
    SetWindowLongPtrW(hwnd, offset, data as LONG_PTR) as usize
}
