use super::ffi::{HINSTANCE, IMAGE_DOS_HEADER};

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
