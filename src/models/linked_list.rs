use std::{cell::RefCell, rc::Rc};

pub type SharedNode<T> = Rc<RefCell<Node<T>>>;

#[allow(dead_code)]
pub struct Node<T> {
    pub elem: Option<Rc<RefCell<T>>>,
    pub next: Option<SharedNode<T>>,
    pub prev: Option<SharedNode<T>>,
}

impl<T> Node<T> {
    pub fn new_shared(value: Option<T>) -> SharedNode<T> {
        Rc::new(RefCell::new(Node {
            elem: value.map(|elem| Rc::new(RefCell::new(elem))),
            next: None,
            prev: None,
        }))
    }

    pub fn get_elem(node: &SharedNode<T>) -> Option<Rc<RefCell<T>>> {
        let node = node.borrow();
        match &node.elem {
            Some(rc) => Some(Rc::clone(&rc)),
            None => None,
        }
    }

    pub fn get_prev(node: &SharedNode<T>) -> Option<SharedNode<T>> {
        let ref mut node = *node.borrow_mut();
        node.prev.as_ref().and_then(|prev| Some(Rc::clone(prev)))
    }

    pub fn get_next(node: &SharedNode<T>) -> Option<SharedNode<T>> {
        let ref mut node = *node.borrow_mut();
        node.next.as_ref().and_then(|next| Some(Rc::clone(next)))
    }

    pub fn set_prev(node: &SharedNode<T>, to: Option<&SharedNode<T>>) {
        let node = Rc::clone(node);
        let ref mut node = *node.borrow_mut();
        node.prev = to.map(|rc| Rc::clone(rc));
    }

    pub fn set_next(node: &SharedNode<T>, to: Option<&SharedNode<T>>) {
        let node = Rc::clone(node);
        let ref mut node = *node.borrow_mut();
        node.next = to.map(|rc| Rc::clone(rc));
    }
}
