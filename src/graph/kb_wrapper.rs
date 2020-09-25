use std::any::Any;
use std::rc::Weak;

/// Wrapper for KB values, because Rust doesn't support upcasting at the moment.
///
/// This is an implementation of
/// [https://stackoverflow.com/a/42057047/257583](https://stackoverflow.com/a/42057047/257583).
pub trait KBWrapper: Any {
    fn as_any(&self) -> &dyn Any;
    fn value(&self) -> &dyn Any;
}

/// KBWrapper for weak references to data.
pub struct WeakWrapper<T: Any> {
    pub item: Weak<T>,
}

impl<T: Any> WeakWrapper<T> {
    pub fn new(weak: Weak<T>) -> Self {
        WeakWrapper { item: weak }
    }
}

impl<'a, T: Any + 'static> KBWrapper for WeakWrapper<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn value(&self) -> &dyn Any {
        &self.item
    }
}
