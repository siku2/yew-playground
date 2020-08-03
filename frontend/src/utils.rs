use std::{cell::RefCell, rc::Rc};
use yew::{Component, ComponentLink};

pub trait NeqAssign {
    fn neq_assign(&mut self, other: Self) -> bool;
}

impl<T> NeqAssign for T
where
    T: PartialEq,
{
    fn neq_assign(&mut self, other: Self) -> bool {
        if *self == other {
            false
        } else {
            *self = other;
            true
        }
    }
}

/// Reference to a `Component` much like `NodeRef` is for a `Node`.
/// Using this directly is an anti-pattern. It should only be exposed through a
/// wrapper type.
#[derive(Debug)]
pub struct ComponentRef<C: Component>(Rc<RefCell<Option<ComponentLink<C>>>>);
impl<C: Component> ComponentRef<C> {
    /// Populate the internal link.
    /// This should be called in the component's `create` method.
    pub fn populate(&self, link: ComponentLink<C>) {
        let prev = self.0.borrow_mut().replace(link);
        debug_assert!(
            prev.is_none(),
            "populated the same component reference twice"
        );
    }

    fn with_link<T>(&self, f: impl FnOnce(&ComponentLink<C>) -> T) -> Option<T> {
        self.0.borrow().as_ref().map(f)
    }

    /// Send a message to the referenced component.
    /// The return value indicates if the message was sent successfully.
    pub fn send_message(&self, msg: C::Message) -> bool {
        self.with_link(|link| link.send_message(msg)).is_some()
    }
}
impl<C: Component> Clone for ComponentRef<C> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<C: Component> Default for ComponentRef<C> {
    fn default() -> Self {
        Self(Rc::new(RefCell::default()))
    }
}
impl<C: Component> PartialEq for ComponentRef<C> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
