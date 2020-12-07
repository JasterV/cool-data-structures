use data_structs::binary_tree::BinaryTree;

#[test]
fn test_insert() {
    let mut btree: BinaryTree<i32> = BinaryTree::new();

    btree.insert(10);
    btree.insert(8);
    btree.insert(11);
    btree.insert(7);

    assert_eq!(btree.size(), 4);
}
#[test]
fn test_inorder() {
    let mut btree: BinaryTree<i32> = BinaryTree::new();

    btree.insert(10);
    btree.insert(8);
    btree.insert(11);
    btree.insert(7);

    let inorder = btree.inorder();
    
    assert_eq!(inorder.len(), 4);
    assert_eq!(*inorder[0].borrow(), 7);
    assert_eq!(*inorder[3].borrow(), 11);
}

#[test]
fn test_postorder() {
    let mut btree: BinaryTree<i32> = BinaryTree::new();

    btree.insert(10);
    btree.insert(8);
    btree.insert(11);
    btree.insert(7);

    let inorder = btree.inorder();
    
    assert_eq!(inorder.len(), 4);
    assert_eq!(*inorder[0].borrow(), 7);
    assert_eq!(*inorder[3].borrow(), 11);
}

#[test]
fn test_preorder() {
    let mut btree: BinaryTree<i32> = BinaryTree::new();

    btree.insert(10);
    btree.insert(8);
    btree.insert(11);
    btree.insert(7);

    let preorder = btree.preorder();
    
    assert_eq!(preorder.len(), 4);
    assert_eq!(*preorder[0].borrow(), 10);
    assert_eq!(*preorder[3].borrow(), 11);
    assert_eq!(*preorder[2].borrow(), 7);
}

