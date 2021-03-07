use linked_list::LinkedList;
use crate::linked_list::ComputeNorm;
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

    let mut norm_list: LinkedList<f64> = LinkedList::new();
    norm_list.push_front(4.0);
    norm_list.push_front(3.0);
    println!("Norm of norm_list is {}", norm_list.compute_norm());


    // If you implement iterator trait:
    for val in &list {
       println!("{}", val);
    }

}
