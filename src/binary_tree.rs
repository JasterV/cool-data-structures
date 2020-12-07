use std::boxed::Box;
use std::rc::Rc;
use std::{cell::RefCell, vec};

use crate::models::binary_tree::Node;

pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn size(&self) -> u32 {
        match self.root {
            Some(ref node) => node.size(),
            _ => 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut elem) => elem.insert(value),
            _ => self.root = Some(Box::new(Node::new(value))),
        }
    }

    pub fn inorder(&self) -> Vec<Rc<RefCell<T>>> {
        Self::_inorder(self.root.as_ref())
    }

    pub fn postorder(&self) -> Vec<Rc<RefCell<T>>> {
        Self::_postorder(self.root.as_ref())
    }

    pub fn preorder(&self) -> Vec<Rc<RefCell<T>>> {
        Self::_preorder(self.root.as_ref())
    }

    fn _inorder(node: Option<&Box<Node<T>>>) -> Vec<Rc<RefCell<T>>> {
        let mut result = Vec::new();
        if let Some(ref node) = node {
            result.append(&mut Self::_inorder(node.left()));
            result.append(&mut vec![node.elem()]);
            result.append(&mut Self::_inorder(node.right()));
        };
        result
    }

    fn _postorder(node: Option<&Box<Node<T>>>) -> Vec<Rc<RefCell<T>>> {
        let mut result = Vec::new();
        if let Some(ref node) = node {
            result.append(&mut Self::_postorder(node.left()));
            result.append(&mut Self::_postorder(node.right()));
            result.append(&mut vec![node.elem()])
        };
        result
    }

    fn _preorder(node: Option<&Box<Node<T>>>) -> Vec<Rc<RefCell<T>>> {
        let mut result = Vec::new();
        if let Some(ref node) = node {
            result.append(&mut vec![node.elem()]);
            result.append(&mut Self::_preorder(node.left()));
            result.append(&mut Self::_preorder(node.right()));
        };
        result
    }
}
