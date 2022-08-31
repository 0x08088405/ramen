use crate::{error::Error, platform::imp};
use crate::util::sync::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct Connection(pub(crate) Arc<Mutex<imp::Connection>>);

impl Connection {
    pub fn new() -> Result<Self, Error> {
        imp::Connection::new().map(|x| Self(Arc::new(Mutex::new(x))))
    }
}
