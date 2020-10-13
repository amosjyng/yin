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
pub trait KBValue: Any {
    /// Because Rust doesn't support upcasting at the moment, this allows us to manually upcast to
    /// `Any` and then downcast to the desired struct thereafter.
    fn as_any(&self) -> &dyn Any;
}

/// Helper function for unwrapping values contained inside a WeakValue.
///
/// There are three layers of options here:
///
///  1. The option of whether or not there is a value associated with this KB node. Necessary
///     because not every node has a value to it.
///  3. The option of whether or not the value associated with this KB node is contained inside a
///     WeakValue, as opposed to some other wrapper. This should always succeed if the code is
///     correct.
///  2. The option of whether the value referred to here still exists. Since a WeakValue refers
///     to values outside of the KB that might stop existing at any given time, this is not
///     guaranteed to return a value even if there was originally one associated with the node.
///
/// This function encapsulates all of the above into one simpler return value.
#[allow(clippy::redundant_allocation)]
pub fn unwrap_weak<'a, T: 'a>(wrapper: Option<Rc<Box<dyn KBValue + 'a>>>) -> Option<Rc<T>> {
    wrapper
        .map(|v| v.as_any().downcast_ref::<WeakValue<T>>().unwrap().value())
        .flatten()
}

/// Similar to unwrap_weak, returns the value held by a StrongValue.
#[allow(clippy::redundant_allocation)]
pub fn unwrap_strong<'a, 'b, T: 'static>(
    wrapper: &'b Option<Rc<Box<dyn KBValue + 'a>>>,
) -> Option<&'b T> {
    // todo: see if lifetime ugliness can be cleaned up without cloning. Ownership transfer may be
    // best here, seeing as CypherGraph doesn't care to own any of these strings.
    wrapper
        .as_ref()
        .map(|v| v.as_any().downcast_ref::<StrongValue<T>>().unwrap().value())
}

/// Returns the value held by a closure StrongValue. We need a RefCell here because somehow
/// closures are mutable when called, and the Rc that wraps a KBValue prevents mutability.
#[allow(clippy::redundant_allocation)]
pub fn unwrap_closure<'a, 'b>(
    wrapper: &'b Option<Rc<Box<dyn KBValue + 'a>>>,
) -> Option<RefMut<'b, KBClosure>> {
    wrapper.as_ref().map(|v| {
        let any: &'b dyn Any = v.as_any();
        let value_wrappers: &'b StrongValue<RefCell<KBClosure>> = any
            .downcast_ref::<StrongValue<RefCell<KBClosure>>>()
            .unwrap();
        let closure_ref: &'b RefCell<KBClosure> = value_wrappers.value();
        let closure: RefMut<'b, KBClosure> = closure_ref.borrow_mut();
        closure
    })
}

/// Unwrap a StrongValue holding a closure, and return the result after running on the input.
#[macro_export]
macro_rules! define_closure {
    ($closure:expr) => {{
        // explicitly declare the type to help the Rust compiler understand
        let strong: Box<StrongValue<RefCell<KBClosure>>> =
            Box::new(StrongValue::new(RefCell::new(Box::new($closure))));
        strong
    }};
}

/// Unwrap a StrongValue holding a closure, and return the result after running on the input.
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

/// KBValue for weak references to data.
#[derive(Debug)]
pub struct WeakValue<T: Any> {
    item: Weak<T>,
}

impl<T: Any> WeakValue<T> {
    /// Create a new KB wrapper that contains a weak reference to the given data.
    pub fn new(rc: &Rc<T>) -> Self {
        WeakValue {
            item: Rc::downgrade(rc),
        }
    }

    /// Retrieve the value that this wrapper points to -- if it still exists, because the KB does
    /// not own the data.
    pub fn value(&self) -> Option<Rc<T>> {
        self.item.upgrade()
    }
}

impl<'a, T: Any + 'static> KBValue for WeakValue<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// KBValue for owned data.
#[derive(Debug)]
pub struct StrongValue<T: Any> {
    item: T,
}

impl<T: Any> StrongValue<T> {
    /// Create a new KB wrapper that owns the given data.
    pub fn new(t: T) -> Self {
        StrongValue { item: t }
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

impl<'a, T: Any + 'static> KBValue for StrongValue<T> {
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
    use crate::node_wrappers::CommonNodeTrait;

    #[test]
    fn test_weak_value() {
        let item = Rc::new("something expensive".to_string());
        let weak = WeakValue::new(&item);
        assert_eq!(unwrap_weak(Some(Rc::new(Box::new(weak)))), Some(item));
    }

    #[test]
    fn test_strong_value() {
        let item = "something owned".to_string();
        let strong = StrongValue::new(item);
        assert_eq!(
            unwrap_strong(&Some(Rc::new(Box::new(strong)))),
            Some(&"something owned".to_string())
        );
    }

    #[test]
    fn test_strong_value_int() {
        let item: i64 = -5;
        let strong = StrongValue::new(item);
        assert_eq!(
            unwrap_strong::<i64>(&Some(Rc::new(Box::new(strong)))),
            Some(&-5)
        );
    }

    #[test]
    fn test_function_value() {
        bind_in_memory_graph();
        let i = Inherits::archetype();
        let kb_result: Option<Rc<Box<dyn KBValue>>> = Some(Rc::new(define_closure!(|t: Tao| {
            Box::new(t.internal_name().unwrap())
        })));
        assert_eq!(
            run_closure!(&kb_result, i, Rc<String>),
            Some(Box::new(Rc::new("inherits".to_string())))
        );
    }
}
