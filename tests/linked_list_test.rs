use data_structs::linked_list::{DoublyLinkedList, SharedNode, Node};


fn get_elem(node: SharedNode<i32>) -> i32 {
    let ref result = node.borrow();
    let opt = result.as_ref().unwrap();
    opt.elem.unwrap()
}

#[test]
fn test_push() {
    let mut link_list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    link_list.push(3);
    link_list.push(5);

    let res1 = get_elem(link_list.get(0));
    let res2 = get_elem(link_list.get(1));

    assert_eq!(res1, 3);
    assert_eq!(res2, 5);
}

#[test]
fn test_push_first() {
    let mut link_list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    link_list.push_first(3);
    link_list.push_first(5);

    let res1 = get_elem(link_list.get(0));
    let res2 = get_elem(link_list.get(1));

    assert_eq!(res1, 5);
    assert_eq!(res2, 3);
}

