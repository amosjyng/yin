use std::any::Any;
use std::rc::{Rc, Weak};

/// Wrapper for KB values, because Rust doesn't support upcasting at the moment, and the KB should
/// support referring to external data structures that it doesn't own itself.
///
/// This is an implementation of
/// [https://stackoverflow.com/a/42057047/257583](https://stackoverflow.com/a/42057047/257583).
pub trait KBWrapper: Any {
    /// Because Rust doesn't support upcasting at the moment, this allows us to manually upcast to
    /// `Any` and then downcast to the desired struct thereafter.
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

/// Similar to unwrap_weak, returns the value held by a String-valued StrongWrapper.
pub fn unwrap_strong<'a>(wrapper: Option<Rc<Box<dyn KBWrapper + 'a>>>) -> Option<Rc<String>> {
    // todo: see if lifetime ugliness can be cleaned up without cloning. Ownership transfer may be
    // best here, seeing as CypherGraph doesn't care to own any of these strings.
    wrapper.map(|v| {
        Rc::new(
            v.as_any()
                .downcast_ref::<StrongWrapper<String>>()
                .unwrap()
                .value()
                .clone(),
        )
    })
}

/// KBWrapper for weak references to data.
#[derive(Debug)]
pub struct WeakWrapper<T: Any> {
    item: Weak<T>,
}

impl<T: Any> WeakWrapper<T> {
    /// Create a new KB wrapper that contains a weak reference to the given data.
    pub fn new(rc: &Rc<T>) -> Self {
        WeakWrapper {
            item: Rc::downgrade(rc),
        }
    }

    /// Retrieve the value that this wrapper points to -- if it still exists, because the KB does
    /// not own the data.
    pub fn value(&self) -> Option<Rc<T>> {
        self.item.upgrade()
    }
}

impl<'a, T: Any + 'static> KBWrapper for WeakWrapper<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// KBWrapper for owned data.
#[derive(Debug)]
pub struct StrongWrapper<T: Any> {
    item: T,
}

impl<T: Any> StrongWrapper<T> {
    /// Create a new KB wrapper that owns the given data.
    pub fn new(t: T) -> Self {
        StrongWrapper { item: t }
    }

    /// Borrow the value that this wrapper owns. Guaranteed to still exist because the KB owns this
    /// data.
    pub fn value(&self) -> &T {
        &self.item
    }
}

impl<'a, T: Any + 'static> KBWrapper for StrongWrapper<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weak_wrapper() {
        let item = Rc::new("something expensive".to_string());
        let weak = WeakWrapper::new(&item);
        assert_eq!(unwrap_weak(Some(Rc::new(Box::new(weak)))), Some(item));
    }

    #[test]
    fn test_strong_wrapper() {
        let item = "something owned".to_string();
        let strong = StrongWrapper::new(item);
        assert_eq!(
            unwrap_strong(Some(Rc::new(Box::new(strong)))),
            Some(Rc::new("something owned".to_string()))
        );
    }
}
