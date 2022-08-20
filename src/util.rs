macro_rules! cstr {
    ($($x:tt)+) => {{ concat!($($x)+, "\0").as_ptr().cast() }};
}

macro_rules! load {
    ($($vis:vis $name:ident($type_name:ident) $($so_name:literal),+ {
        $(fn $fn_name:ident($($arg_name:ident:$arg_ty:ty),+$(,)?) $(-> $ret:ty)?;)+
    })+) => {
        pub(self) enum __anyopaque {}
        $(
            #[link_section = ".bss"]
            $vis static mut $name: ::std::mem::MaybeUninit<$type_name> = ::std::mem::MaybeUninit::uninit();
            #[repr(C)]
            $vis struct $type_name {
                $($fn_name: unsafe extern "system" fn($($arg_ty),+) $(-> $ret)?),+,
            }
            impl $type_name {
                $vis unsafe fn load() -> bool {
                    static INIT: ::std::sync::Once = ::std::sync::Once::new();
                    static mut LOADED: bool = false;
                    INIT.call_once(|| {
                        let mut fp = $name.as_mut_ptr() as *mut *mut __anyopaque;
                        let mut handle = ::std::ptr::null_mut();
                        for name in [$(cstr!($so_name)),+] {
                            handle = dlopen(name);
                            if !handle.is_null() { break; }
                        }
                        let _ = dlerror();
                        if handle.is_null() { return; }
                        for sym in [$(cstr!(stringify!($fn_name))),+] {
                            *fp = dlsym(handle, sym).cast();
                            fp = fp.offset(1);
                        }
                        LOADED = dlerror().is_null();
                    });
                    LOADED
                }
            }
            $(#[inline(always)] $vis unsafe fn $fn_name($($arg_name:$arg_ty),+) $(-> $ret)? {
                ((&*$name.as_ptr()).$fn_name)($($arg_name),+)
            })*
        )*
    };
}










// fucking mess below





use std::collections::TryReserveError;

pub(crate) trait TryPush<T> {
    fn _try_push(&mut self, value: T) -> Result<(), TryReserveError>;
}

impl<T> TryPush<T> for Vec<T> {
    fn _try_push(&mut self, value: T) -> Result<(), TryReserveError> {
        if self.len() == self.capacity() {
            // same allocation as .push()'s impl
            // it usually allocates more, 1 is a hint
            self.try_reserve(1)?;
        }
        self.push(value);
        Ok(())
    }
}

#[cfg(not(target_os = "linux"))]
pub(crate) mod sync {
    pub(crate) use self::imp::{cvar_notify_one, cvar_wait, mutex_lock, Condvar, Mutex, MutexGuard};

    #[cfg(not(feature = "parking-lot"))]
    pub(crate) mod imp {
        use std::ptr;
        pub(crate) use std::sync::{Condvar, Mutex, MutexGuard};

        #[inline]
        pub(crate) fn cvar_notify_one(cvar: &Condvar) {
            cvar.notify_one();
        }

        pub(crate) fn cvar_wait<T>(cvar: &Condvar, guard: &mut MutexGuard<T>) {
            // The signature in `std` is quite a pain and CONSUMES the guard
            // We "move it out" for the duration of the wait as a hack
            unsafe {
                let guard_copy = ptr::read(guard);
                let result = cvar.wait(guard_copy).expect("cvar mutex poisoned (this is a bug)");
                ptr::write(guard, result);
            }
        }

        pub(crate) fn mutex_lock<T>(mtx: &Mutex<T>) -> MutexGuard<T> {
            mtx.lock().expect("mutex poisoned (this is a bug)")
        }
    }

    #[cfg(feature = "parking-lot")]
    pub(crate) mod imp {
        pub(crate) use parking_lot::{Condvar, Mutex, MutexGuard};

        #[inline]
        pub(crate) fn cvar_notify_one(cvar: &Condvar) {
            let _ = cvar.notify_one();
        }

        #[inline]
        pub(crate) fn cvar_wait<T>(cvar: &Condvar, guard: &mut MutexGuard<T>) {
            cvar.wait(guard);
        }

        #[inline]
        pub(crate) fn mutex_lock<T>(mtx: &Mutex<T>) -> MutexGuard<T> {
            mtx.lock()
        }
    }
}

#[cfg(target_os = "linux")]
pub(crate) mod sync {
    pub(crate) use self::imp::{mutex_lock, Mutex};

    #[cfg(not(feature = "parking-lot"))]
    pub(crate) mod imp {
        pub(crate) use std::sync::{Mutex, MutexGuard};

        pub(crate) fn mutex_lock<T>(mtx: &Mutex<T>) -> MutexGuard<T> {
            mtx.lock().expect("mutex poisoned (this is a bug)")
        }
    }

    #[cfg(feature = "parking-lot")]
    pub(crate) mod imp {
        pub(crate) use parking_lot::{Mutex, MutexGuard};

        #[inline]
        pub(crate) fn mutex_lock<T>(mtx: &Mutex<T>) -> MutexGuard<T> {
            mtx.lock()
        }
    }
}
