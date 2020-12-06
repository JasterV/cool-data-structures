use std::{cell::RefCell, rc::Rc};

pub type SharedNode<T> = Rc<RefCell<Option<Node<T>>>>;

#[allow(dead_code)]
pub struct Node<T> {
    pub elem: Option<T>,
    pub next: SharedNode<T>,
    pub prev: SharedNode<T>,
}

impl<T> Node<T> {
    pub fn new_shared(value: Option<T>) -> SharedNode<T> {
        Rc::new(RefCell::new(Some(Node {
            elem: value,
            next: Rc::new(RefCell::new(None)),
            prev: Rc::new(RefCell::new(None)),
        })))
    }
    
    pub fn is_none(node: &SharedNode<T>) -> bool {
        node.borrow_mut().is_none()
    }

    pub fn get_prev(node: &SharedNode<T>) -> SharedNode<T> {
        Rc::clone(&(*node.borrow_mut()).as_mut().unwrap().prev)
    }

    pub fn get_next(node: &SharedNode<T>) -> SharedNode<T> {
        Rc::clone(&(*node.borrow_mut()).as_mut().unwrap().next)
    }

    pub fn set_prev(node: &SharedNode<T>, to: &SharedNode<T>) {
        let node = Rc::clone(node);
        let ref mut node = *node.borrow_mut();
        let node = node.as_mut().unwrap();
        node.prev = Rc::clone(to);
    }

    pub fn set_next(node: &SharedNode<T>, to: &SharedNode<T>) {
        let node = Rc::clone(node);
        let ref mut node = *node.borrow_mut();
        let node = node.as_mut().unwrap();
        node.next = Rc::clone(to);
    }
}
