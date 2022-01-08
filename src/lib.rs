#![cfg_attr(feature = "nightly-rustdoc", feature(doc_cfg))]
#![warn(unreachable_pub, unused_import_braces)]
#![deny(unused_results)]

pub mod error;
pub mod platform;
pub(crate) mod sync;
pub mod window;

#[cfg(test)]
mod tests {
    #[test]
    fn has_send_sync() {
        fn test<T: Send + Sync>() {}

        test::<crate::error::Error>();
        test::<crate::window::Builder>();
        test::<crate::window::Controls>();
        test::<crate::window::Style>();
        test::<crate::window::Window>();
    }
}
