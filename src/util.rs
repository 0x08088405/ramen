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
