use data_structs::linked_list::DoublyLinkedList;

#[test]
fn test_push() {
    let mut link_list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    link_list.push(3);
    link_list.push(5);

    let res1 = link_list.get(0).unwrap();
    let res2 = link_list.get(1).unwrap();

    assert_eq!(*res1.borrow(), 3);
    assert_eq!(*res2.borrow(), 5);
}

#[test]
fn test_push_first() {
    let mut link_list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    link_list.push_first(3);
    link_list.push_first(5);

    let res1 = link_list.get(0).unwrap();
    let res2 = link_list.get(1).unwrap();

    assert_eq!(*res1.borrow(), 5);
    assert_eq!(*res2.borrow(), 3);
}

#[test]
fn test_combine_push() {
    let mut link_list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    link_list.push(3);
    link_list.push_first(23);
    link_list.push(10);
    link_list.push_first(5);

    let res1 = link_list.get(0).unwrap();
    let res2 = link_list.get(1).unwrap();
    let res3 = link_list.get(2).unwrap();
    let res4 = link_list.get(3).unwrap();
    let res5 = link_list.get(4);

    assert_eq!(*res1.borrow(), 5);
    assert_eq!(*res2.borrow(), 23);
    assert_eq!(*res3.borrow(), 3);
    assert_eq!(*res4.borrow(), 10);
    assert!(res5.is_none());
}


#[test]
fn test_remove() {
    let mut link_list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    link_list.push(3);
    link_list.push(10);
    link_list.push(40);

    assert_eq!(link_list.length(), 3);

    link_list.remove(2).unwrap();

    assert_eq!(link_list.length(), 2);
    let n = link_list.get(1).unwrap();
    assert_eq!(*n.borrow(), 10);
    assert!(link_list.get(2).is_none());
}

