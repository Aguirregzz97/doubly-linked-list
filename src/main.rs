use std::collections::LinkedList;
use std::cmp;
use std::fmt::Debug;
use std::ops::Deref;

// Structures
#[derive(Debug)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
    size: i32,
}

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

// Iterator Structures
pub struct IntoIter<T> (List<T>);

pub struct Iter<'a,T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

// Implementations
impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, elem:T) {
        self.head = Some(Box::new(Node{ elem: elem, next: self.head.take() }));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.size = cmp::max(self.size - 1, 0);
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            },
            None => None,
        }
        // self.head.take().map(|node|{
        //     self.head = node.next;
        //     node.elem
        // })
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_top(&self) -> Option<&T> {
        match self.head.as_ref() {
            Some(node) => {
                Some(&node.elem)
            },
            None => None
        }
    }

    // Iterator Methods
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter (self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| { &**node }),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|node| { &mut **node }),
        }
    }
}

// Traits
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Traits Implementation
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|next_node| { &** next_node });
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|next_node| { &mut ** next_node });
            &mut node.elem
        })
    }
}

// Doubly Linked List
#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    left: List<T>,
    right: List<T>,
    size: i32,
}

impl<T> DoublyLinkedList<T> {

    pub fn new() -> Self {
        let mut left_list = List::new();
        let mut right_list = List::new();
        DoublyLinkedList {
            left: left_list,
            right: right_list,
            size: 0,
        }
    }

    pub fn next(&mut self) {
        if (self.right.get_size() != 0) {
            self.right.pop().map(|node| { self.left.push(node); } );
        }
    }

    pub fn previous(&mut self) {
        if (self.left.get_size() != 0) {
            self.left.pop().map( |node| { self.right.push(node); } );
        }
    }

    pub fn push_back(&mut self, elem: T) {
        while (self.right.get_size() != 0) {
            self.next();
        }
        self.left.push(elem);
        self.size += 1;
    }

    pub fn shift(&mut self, index: i32) {
        while (self.left.get_size() != index + 1) {
            if (index + 1 > self.left.get_size()) {
                self.next();
            } else {
                self.previous();
            }
        }
    }

    pub fn push(&mut self, elem: T, index: i32) {
        self.shift(index);
        self.left.push(elem);
        self.size += 1;
    }

    pub fn pop_back(&mut self) {
        self.shift(*self.get_size().unwrap() - 1);
        self.left.pop();
        self.size = cmp::max(0, self.size - 1);
    }

    pub fn pop(&mut self, index: i32) {
        self.shift(index);
        self.left.pop();
        self.size = cmp::max(0, self.size - 1);
    }

    pub fn get_size(&self) -> Option<&i32> {
        Some(&self.size)
    }

    pub fn get_current(&mut self) -> Option<&T> {
        self.left.get_top()
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.shift(index);
        self.left.get_top()
    }

}

pub fn print_dll(dl: &mut DoublyLinkedList<i32>) {
    let sz = *dl.get_size().unwrap();
    for x in 0..sz {
        print!("{} ", dl.get(x).unwrap());
    }
}

fn main() {
    // let mut list:List<i32> = List::new();
    // list.push(1);
    // list.push(2);
    // list.push(3);
    //
    // let mut it = list.iter_mut();
    //
    // let mut elem = it.next();
    // elem.take().map(|node| {
    //     *node += 2;
    //     elem = Some(node);
    // });
    // it = list.iter_mut();
    // elem = it.next();
    // // print!("{} ", elem.unwrap());
    // while (elem != None) {
    //     elem.map(|value| {
    //         println!("{}", value);
    //     });
    //     elem = it.next();
    // }

    let mut dl: DoublyLinkedList<i32> = DoublyLinkedList::new();
    dl.push_back(1);
    dl.push_back(2);
    dl.push_back(3);
    dl.push_back(4);
    dl.push_back(5);
    dl.push_back(6);

    dl.previous();
    dl.previous();
    dl.previous();
    dl.next();
    print!("{} ", dl.get_current().unwrap());




    // match dl.get_size() {
    //     Some()
    // }



}


