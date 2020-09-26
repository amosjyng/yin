use std::any::Any;
use std::rc::{Rc, Weak};

/// Wrapper for KB values, because Rust doesn't support upcasting at the moment, and the KB should
/// support referring to external data structures that it doesn't own itself.
///
/// This is an implementation of
/// [https://stackoverflow.com/a/42057047/257583](https://stackoverflow.com/a/42057047/257583).
pub trait KBWrapper: Any {
    fn as_any(&self) -> &dyn Any;
}

/// Helper function for unwrapping values contained inside a WeakWrapper.
///
/// There are three layers of options here:
///
///  1. The option of whether or not there is a value associated with this KB node. Necessary
///     because not every node has a value to it.
///  3. The option of whether or not the value associated with this KB node is contained inside a
///     WeakWrapper, as opposed to some other wrapper. This should always succeed if the code is
///     correct.
///  2. The option of whether the value referred to here still exists. Since a WeakWrapper refers
///     to values outside of the KB that might stop existing at any given time, this is not
///     guaranteed to return a value even if there was originally one associated with the node.
///
/// This function encapsulates all of the above into one simpler return value.
pub fn unwrap_weak<'a, T: 'a>(wrapper: Option<Rc<Box<dyn KBWrapper + 'a>>>) -> Option<Rc<T>> {
    wrapper
        .map(|v| v.as_any().downcast_ref::<WeakWrapper<T>>().unwrap().value())
        .flatten()
}

/// KBWrapper for weak references to data.
#[derive(Debug)]
pub struct WeakWrapper<T: Any> {
    pub item: Weak<T>,
}

impl<T: Any> WeakWrapper<T> {
    pub fn new(rc: &Rc<T>) -> Self {
        WeakWrapper {
            item: Rc::downgrade(rc),
        }
    }

    pub fn value(&self) -> Option<Rc<T>> {
        self.item.upgrade()
    }
}

impl<'a, T: Any + 'static> KBWrapper for WeakWrapper<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
