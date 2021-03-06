use linked_list::LinkedList;
use linked_list::Node;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    // println!("{}", list);
    // println!("list size: {}", list.get_size());
    // println!("top element: {}", list.pop_front().unwrap());
    // println!("{}", list);
    // println!("size: {}", list.get_size());
    // println!("{}", list.to_string()); // ToString impl for anything impl Display

    let cloned_list = list.clone();
    println!("{}", cloned_list);

    println!("{}", list == cloned_list);
    
    let mut empty_list: LinkedList<i32> = LinkedList::new();
    for i in 12..24 {
        empty_list.push_front(i);
    }
    println!("{}", list == empty_list);

    // println!("{:?}", list.head);

    // let node: Node<i32> = Node::new(5, None);
    // println!("{:?}", node);
    // let cloned_node = node.clone();
    // println!("{:?}", cloned_node)

    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}
