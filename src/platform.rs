#[cfg_attr(feature = "nightly-rustdoc", doc(cfg(target_os = "windows")))]
#[cfg_attr(not(feature = "nightly-rustdoc"), cfg(target_os = "windows"))]
pub mod win32;

#[cfg(target_os = "windows")]
pub(crate) use win32 as imp;
