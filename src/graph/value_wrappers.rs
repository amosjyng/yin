use crate::tao::form::Form;
use std::any::Any;
use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

/// Closure stored inside the KB.
pub type KBClosure = Box<dyn FnMut(Form) -> Box<dyn Any>>;

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
///     WeakValue, as opposed to some other wrapper. If this fails, this will try a StrongValue
///     next. This should always succeed if the code is correct.
///  2. The option of whether the value referred to here still exists. Since a WeakValue refers
///     to values outside of the KB that might stop existing at any given time, this is not
///     guaranteed to return a value even if there was originally one associated with the node.
///
/// This function encapsulates all of the above into one simpler return value.
pub fn unwrap_value<'a, T: 'a>(wrapper: Option<Rc<dyn KBValue + 'a>>) -> Option<Rc<T>> {
    wrapper
        .map(|v| {
            let any_value = v.as_any();
            match any_value.downcast_ref::<WeakValue<T>>() {
                Some(weak_value) => weak_value.value(),
                None => Some(any_value.downcast_ref::<StrongValue<T>>().unwrap().value()),
            }
        })
        .flatten()
}

/// Unwrap a StrongValue holding a closure, and return the result after running on the input.
pub fn run_closure<'a, 'b, T: 'static>(
    wrapper: &'b Option<Rc<dyn KBValue + 'a>>,
    input: Form,
) -> Option<Box<T>> {
    wrapper.as_ref().map(|v| {
        let any: &'b dyn Any = v.as_any();
        let value_wrappers: &'b StrongValue<RefCell<KBClosure>> = any
            .downcast_ref::<StrongValue<RefCell<KBClosure>>>()
            .unwrap();
        let closure_ref: Rc<RefCell<KBClosure>> = value_wrappers.value();
        let mut closure: RefMut<'_, KBClosure> = closure_ref.borrow_mut();
        let result: Box<dyn Any> = closure(input);
        let cast_result: Box<T> = result.downcast().expect("Downcast type failure");
        cast_result
    })
}

/// Unwrap a StrongValue holding a closure, and return the result after running on the input.
#[macro_export]
macro_rules! define_closure {
    ($closure:expr) => {{
        // explicitly declare the type to help the Rust compiler understand
        let strong: Rc<StrongValue<RefCell<KBClosure>>> =
            Rc::new(StrongValue::new(RefCell::new(Box::new($closure))));
        strong
    }};
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

/// KBValue for owned immutable data.
#[derive(Debug)]
pub struct StrongValue<T: Any> {
    item: Rc<T>,
}

impl<T: Any> StrongValue<T> {
    /// Create a new KB wrapper that owns the given data.
    pub fn new(t: T) -> Self {
        StrongValue { item: Rc::new(t) }
    }

    /// Get value that this wrapper owns. Guaranteed to still exist because the KB owns this data.
    pub fn value(&self) -> Rc<T> {
        self.item.clone()
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
    use crate::node_wrappers::CommonNodeTrait;
    use crate::tao::archetype::ArchetypeTrait;
    use crate::tao::form::FormTrait;
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::Inherits;

    #[test]
    fn test_weak_value() {
        let item = Rc::new("something expensive".to_string());
        let weak = WeakValue::new(&item);
        assert_eq!(unwrap_value(Some(Rc::new(weak))), Some(item));
    }

    #[test]
    fn test_strong_value() {
        let item = "something owned".to_string();
        let strong = StrongValue::new(item);
        assert_eq!(
            unwrap_value(Some(Rc::new(strong))),
            Some(Rc::new("something owned".to_string()))
        );
    }

    #[test]
    fn test_strong_value_int() {
        let item: i64 = -5;
        let strong = StrongValue::new(item);
        assert_eq!(
            unwrap_value::<i64>(Some(Rc::new(strong))),
            Some(Rc::new(-5))
        );
    }

    #[test]
    fn test_function_value() {
        initialize_kb();
        let i = Inherits::archetype();
        let kb_result: Option<Rc<dyn KBValue>> = Some(define_closure!(|t: Form| {
            Box::new(t.internal_name_str().unwrap())
        }));
        assert_eq!(
            run_closure::<Rc<str>>(&kb_result, i.as_form()),
            Some(Box::new(Rc::from("inherits")))
        );
    }
}
