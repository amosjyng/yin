use crate::concepts::Tao;
use std::any::Any;
use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

/// Closure stored inside the KB.
pub type KBClosure = Box<dyn FnMut(Tao) -> Box<dyn Any>>;

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
pub fn unwrap_strong<'a, 'b>(
    wrapper: &'b Option<Rc<Box<dyn KBWrapper + 'a>>>,
) -> Option<&'b String> {
    // todo: see if lifetime ugliness can be cleaned up without cloning. Ownership transfer may be
    // best here, seeing as CypherGraph doesn't care to own any of these strings.
    wrapper.as_ref().map(|v| {
        v.as_any()
            .downcast_ref::<StrongWrapper<String>>()
            .unwrap()
            .value()
    })
}

/// Returns the value held by a closure StrongWrapper. We need a RefCell here because somehow
/// closures are mutable when called, and the Rc that wraps a KBWrapper prevents mutability.
pub fn unwrap_closure<'a, 'b, 'c>(
    wrapper: &'b Option<Rc<Box<dyn KBWrapper + 'a>>>,
) -> Option<RefMut<'b, KBClosure>> {
    wrapper.as_ref().map(|v| {
        let any: &'b dyn Any = v.as_any();
        let kb_wrapper: &'b StrongWrapper<RefCell<KBClosure>> = any
            .downcast_ref::<StrongWrapper<RefCell<KBClosure>>>()
            .unwrap();
        let closure_ref: &'b RefCell<KBClosure> = kb_wrapper.value();
        let closure: RefMut<'b, KBClosure> = closure_ref.borrow_mut();
        closure
    })
}

/// Unwrap a StrongWrapper holding a closure, and return the result after running on the input.
#[macro_export]
macro_rules! define_closure {
    ($closure:expr) => {{
        // explicitly declare the type to help the Rust compiler understand
        let strong: Box<StrongWrapper<RefCell<KBClosure>>> =
            Box::new(StrongWrapper::new(RefCell::new(Box::new($closure))));
        strong
    }};
}

/// Unwrap a StrongWrapper holding a closure, and return the result after running on the input.
#[macro_export]
macro_rules! run_closure {
    ($wrapper:expr, $input:expr, $t:ty) => {
        unwrap_closure($wrapper).map(|mut c: RefMut<'_, KBClosure>| {
            let result: Box<dyn Any> = c($input.ego_death());
            let cast_result: Box<$t> = result.downcast().expect("Downcast type failure");
            cast_result
        })
    };
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

    /// Get value that this wrapper owns. Guaranteed to still exist because the KB owns this data.
    pub fn value(&self) -> &T {
        &self.item
    }

    /// Get mutable value that this wrapper owns. Guaranteed to still exist because the KB owns
    /// this data.
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.item
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
    use crate::concepts::attributes::Inherits;
    use crate::concepts::{ArchetypeTrait, FormTrait};
    use crate::graph::bind_in_memory_graph;
    use crate::wrappers::CommonNodeTrait;

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
            unwrap_strong(&Some(Rc::new(Box::new(strong)))),
            Some(&"something owned".to_string())
        );
    }

    #[test]
    fn test_function_wrapper() {
        bind_in_memory_graph();
        let i = Inherits::archetype();
        let kb_result: Option<Rc<Box<dyn KBWrapper>>> = Some(Rc::new(define_closure!(|t: Tao| {
            Box::new(t.internal_name().unwrap())
        })));
        assert_eq!(
            run_closure!(&kb_result, i, Rc<String>),
            Some(Box::new(Rc::new("Inherits".to_string())))
        );
    }
}
