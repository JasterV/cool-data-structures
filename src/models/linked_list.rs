use std::{cell::RefCell, rc::Rc};

pub type SharedNode<T> = Rc<RefCell<Option<Node<T>>>>;

#[allow(dead_code)]
pub struct Node<T> {
    elem: Option<T>,
    next: SharedNode<T>,
    prev: SharedNode<T>,
}

#[allow(dead_code)]
impl<T> Node<T> {
    pub fn new(value: T, next: &SharedNode<T>, prev: &SharedNode<T>) -> Self {
        Node {
            elem: Some(value),
            next: Rc::clone(next),
            prev: Rc::clone(prev),
        }
    }

    pub fn new_head() -> Self {
        Node {
            elem: None,
            next: Rc::new(RefCell::new(None)),
            prev: Rc::new(RefCell::new(None)),
        }
    }

    pub fn elem(&self) -> Option<&T> {
        self.elem.as_ref()
    }

    pub fn elem_mut(&mut self) -> Option<&mut T> {
        self.elem.as_mut()
    }

    pub fn next(&self) -> SharedNode<T> {
        Rc::clone(&self.next)
    }

    pub fn prev(&self) -> SharedNode<T> {
        Rc::clone(&self.prev)
    }
}
