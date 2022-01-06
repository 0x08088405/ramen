//! Wrapper that uses either `parking_lot` or `std` for synchronisation primitives.

pub use self::imp::{cvar_notify_one, cvar_wait, mutex_lock, Condvar, Mutex, MutexGuard};

#[cfg(not(feature = "parking-lot"))]
pub(crate) mod imp {
    use std::ptr;
    pub use std::sync::{Condvar, Mutex, MutexGuard};

    #[inline]
    pub fn cvar_notify_one(cvar: &Condvar) {
        cvar.notify_one();
    }

    pub fn cvar_wait<T>(cvar: &Condvar, guard: &mut MutexGuard<T>) {
        // The signature in `std` is quite a pain and CONSUMES the guard
        // We "move it out" for the duration of the wait as a hack
        unsafe {
            let guard_copy = ptr::read(guard);
            let result = cvar.wait(guard_copy).expect("cvar mutex poisoned (this is a bug)");
            ptr::write(guard, result);
        }
    }

    pub fn mutex_lock<T>(mtx: &Mutex<T>) -> MutexGuard<T> {
        mtx.lock().expect("mutex poisoned (this is a bug)")
    }
}

#[cfg(feature = "parking-lot")]
pub(crate) mod imp {
    pub use parking_lot::{Condvar, Mutex, MutexGuard};

    #[inline]
    pub fn cvar_notify_one(cvar: &Condvar) {
        let _ = cvar.notify_one();
    }

    #[inline]
    pub fn cvar_wait<T>(cvar: &Condvar, guard: &mut MutexGuard<T>) {
        cvar.wait(guard);
    }

    #[inline]
    pub fn mutex_lock<T>(mtx: &Mutex<T>) -> MutexGuard<T> {
        mtx.lock()
    }
}
