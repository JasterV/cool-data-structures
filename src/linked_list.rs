use std::{cell::RefCell, rc::Rc};
use crate::models::linked_list::{Node, SharedNode};

pub struct DoublyLinkedList<T> {
    length: u64,
    root: Rc<RefCell<Node<T>>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            length: 0,
            root: Rc::new(RefCell::new(Node::new_head())),
        }
    }

    pub fn length(&self) -> u64 {
        self.length
    }
}
