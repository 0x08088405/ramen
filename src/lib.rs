#![cfg_attr(feature = "nightly-rustdoc", feature(doc_cfg))]
#![warn(unreachable_pub, unused_import_braces)]
#![deny(unused_results)]

pub mod error;
pub mod event;
#[cfg_attr(feature = "nightly-docs", doc(cfg(feature = "input")))]
#[cfg_attr(not(feature = "nightly-docs"), cfg(feature = "input"))]
pub mod input;
pub mod platform;
pub(crate) mod util;
pub mod window;

#[cfg(test)]
mod tests {
    #[test]
    fn has_send_sync() {
        fn test<T: Send + Sync>() {}

        test::<crate::error::Error>();
        test::<crate::event::CloseReason>();
        test::<crate::event::Event>();
        test::<crate::window::Builder>();
        test::<crate::window::Controls>();
        test::<crate::window::Style>();
        test::<crate::window::Window>();
    }
}
