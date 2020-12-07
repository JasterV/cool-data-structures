use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node<T> {
    elem: Rc<RefCell<T>>,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            elem: Rc::new(RefCell::new(value)),
            left: None,
            right: None,
        }
    }

    pub fn elem(&self) -> Rc<RefCell<T>> {
        Rc::clone(&self.elem)
    }

    pub fn left(&self) -> Option<&Box<Node<T>>> {
        self.left.as_ref()
    }

    pub fn right(&self) -> Option<&Box<Node<T>>> {
        self.right.as_ref()
    }

    pub fn size(&self) -> u32 {
        1 + match self.left {
            Some(ref elem) => elem.size(),
            _ => 0,
        } + match self.right {
            Some(ref elem) => elem.size(),
            _ => 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        if value <= *self.elem.borrow() {
            match self.left {
                Some(ref mut elem) => elem.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        } else {
            match self.right {
                Some(ref mut elem) => elem.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }
}