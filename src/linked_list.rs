use std::rc::Rc;

enum Node<T> {
    Header {
        first: Option<Rc<Node<T>>>,
        last: Option<Rc<Node<T>>>,
    },
    Child {
        elem: T,
        next: Rc<Node<T>>,
        prev: Rc<Node<T>>,
    },
}

pub struct DoublyLinkedList<T> {
    length: u64,
    root: Node<T>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            length: 0,
            root: Node::Header {
                first: None,
                last: None,
            },
        }
    }
}
