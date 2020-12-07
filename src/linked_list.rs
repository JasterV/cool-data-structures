use crate::models::linked_list::{Node, SharedNode};
use std::{cell::RefCell, rc::Rc};

pub struct DoublyLinkedList<T> {
    length: u64,
    root: SharedNode<T>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            length: 0,
            root: Rc::new(RefCell::new(Node {
                elem: None,
                next: None,
                prev: None,
            })),
        }
    }

    pub fn length(&self) -> u64 {
        self.length
    }

    pub fn push(&mut self, value: T) {
        let node = Node::new_shared(Some(value));

        if self.length == 0 {
            Node::set_next(&self.root, Some(&node));
            Node::set_prev(&node, Some(&self.root));
        } else {
            let last = Node::get_prev(&self.root).unwrap();
            Node::set_next(&last, Some(&node));
            Node::set_prev(&node, Some(&last));
        }
        Node::set_prev(&self.root, Some(&node));
        Node::set_next(&node, Some(&self.root));
        self.length += 1;
    }

    pub fn push_first(&mut self, value: T) {
        let node = Node::new_shared(Some(value));
        if self.length == 0 {
            Node::set_prev(&self.root, Some(&node));
            Node::set_next(&node, Some(&self.root));
        } else {
            let first = Node::get_next(&self.root).unwrap();
            Node::set_prev(&first, Some(&node));
            Node::set_next(&node, Some(&first));
        }
        Node::set_next(&self.root, Some(&node));
        Node::set_prev(&node, Some(&self.root));
        self.length += 1;
    }

    pub fn get(&self, index: u64) -> Option<Rc<RefCell<T>>> {
        self._get_node(index).and_then(|rc| Node::get_elem(&rc))
    }

    pub fn set(&self, index: u64, value: T) {
        let elem = self._get_node(index)
            .and_then(|rc| Node::get_elem(&rc))
            .expect("Index out of bounds");
        *elem.borrow_mut() = value;
    }

    pub fn remove(&mut self, index: u64) -> Result<(), String> {
        let node = self._get_node(index).ok_or(String::from("Invalid index"))?;

        let prev = Node::get_prev(&node).unwrap();
        let next = Node::get_next(&node).unwrap();

        Node::set_next(&prev, Some(&next));
        Node::set_prev(&next, Some(&prev));

        if self.length == 1 {
            Node::set_next(&self.root, None);
            Node::set_prev(&self.root, None);
        }
        self.length -= 1;
        Ok(())
    }

    fn _get_node(&self, index: u64) -> Option<SharedNode<T>> {
        if self.length == 0 || index >= self.length {
            return None;
        }

        let mut current = Rc::clone(&self.root);
        if index < self.length / 2 {
            for _ in 0..index + 1 {
                current = Node::get_next(&current).unwrap();
            }
        } else {
            for _ in 0..self.length - index {
                current = Node::get_prev(&current).unwrap();
            }
        }

        Some(Rc::clone(&current))
    }
}
