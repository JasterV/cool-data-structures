pub use crate::models::linked_list::{Node, SharedNode};
use std::{cell::RefCell, rc::Rc};

pub struct DoublyLinkedList<T> {
    length: u64,
    root: SharedNode<T>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            length: 0,
            root: Rc::new(RefCell::new(Some(Node {
                elem: None,
                next: Node::new_shared(None),
                prev: Node::new_shared(None),
            }))),
        }
    }

    pub fn length(&self) -> u64 {
        self.length
    }

    pub fn push(&mut self, value: T) {
        let node = Node::new_shared(Some(value));

        if self.length == 0 {
            Node::set_next(&self.root, &node);
            Node::set_prev(&node, &self.root);
        } else {
            let last = Node::get_prev(&self.root);
            Node::set_next(&last, &node);
            Node::set_prev(&node, &last);
        }
        Node::set_prev(&self.root, &node);
        Node::set_next(&node, &self.root);
        self.length += 1;
    }

    pub fn push_first(&mut self, value: T) {
        let node = Node::new_shared(Some(value));
        if self.length == 0 {
            Node::set_prev(&self.root, &node);
            Node::set_next(&node, &self.root);
        } else {
            let first = Node::get_next(&self.root);
            Node::set_prev(&first, &node);
            Node::set_next(&node, &first);
        }
        Node::set_next(&self.root, &node);
        Node::set_prev(&node, &self.root);
        self.length += 1;
    }

    pub fn get(&self, index: u64) -> SharedNode<T> {
        if self.length == 0 || index >= self.length {
            return Node::new_shared(None);
        }

        let mut current = Rc::clone(&self.root);
        if index < self.length / 2 {
            for _ in 0..index + 1 {
                current = Node::get_next(&current);
            }
        } else {
            for _ in 0..self.length - index {
                current = Node::get_prev(&current);
            }
        }

        current
    }
}
