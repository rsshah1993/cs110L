use std::fmt;
use std::option::Option;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node <T>{
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value: value, next: next}
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        let cloned_node: Node<T> = Node::new(self.value.clone(), self.next.clone());
        cloned_node
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.next == other.next
    }
}

impl<T> LinkedList <T>{
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}


impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {},", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut cloned: LinkedList<T> = LinkedList::new();
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut vec: Vec<T> = Vec::new();
        loop {
            match current {
                Some(node) => {
                    let cloned_node: Box<Node<T>> = node.clone();
                    vec.push(cloned_node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        for i in vec.iter().rev() {
            let after = i.clone();
            cloned.push_front(after);
        }
        cloned
    }
}

impl <T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.get_size() != other.get_size() {
            return false
        }
        else if &self.head != &other.head {
            return false
        }
        else {
            return true
        }
    }
}

pub trait ComputeNorm {
    fn compute_norm(&self) -> f64 {
        0.0
    }
}

impl ComputeNorm for LinkedList<f64> {
    fn compute_norm(&self) -> f64 {
        let mut norm: f64 = 0.0;
        let mut current: &Option<Box<Node<f64>>> = &self.head;
        loop {
            match current {
                Some(node) => {
                     norm += node.value * node.value;
                     current = &node.next;
                },
                None => break,
            }
        }
        norm.sqrt()
    }
}

pub struct LinkedListIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}


impl<T: Copy> Iterator for LinkedListIter<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        match self.current {
            Some(node) => {
                self.current = &node.next;
                Some(node.value)
            },
            None => None
        }
    }
}

impl<'a, T: Copy> IntoIterator for &'a LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIter<'a, T>;
    fn into_iter(self) -> LinkedListIter<'a, T> {
        LinkedListIter {current: &self.head}
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}