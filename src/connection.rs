use crate::{error::Error, platform::imp};
use crate::util::sync::Mutex;
use std::sync::Arc;

/// A connection to a windowing backend, used as a factory object to create [`Window`](crate::window::Window)s and
/// [`Builder`](crate::window::Builder)s.
/// 
/// This is a wrapper for an atomically reference-counted object, so it can be easily cloned for access from multiple
/// places, or different threads. It can be dropped without needing to drop any windows or builders created from it:
/// the internal connection will stay alive until all clones of this object **and** all such windows and builders
/// have been dropped.
#[derive(Clone)]
pub struct Connection(pub(crate) Arc<Mutex<imp::Connection>>);

impl Connection {
    /// Attempts to connect to the backend for the target platform.
    pub fn new() -> Result<Self, Error> {
        imp::Connection::new().map(|x| Self(Arc::new(Mutex::new(x))))
    }
}
