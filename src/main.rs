use std::collections::LinkedList;
use std::cmp;
use std::fmt::Debug;
use std::ops::Deref;

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
}

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

    pub fn get_size(&self) -> Option<&i32> {
        Some(&self.size)
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.shift(index);
        self.left.get_top()
    }


}


fn main() {
    // let mut list:List<i32> = List::new();
    // list.push(1);
    // list.push(2);
    // list.push(3);
    // // let mut head = list.pop();
    //
    // let mut head = list.pop();
    // let mut x = list.get_top();
    // match x {
    //     Some(value) => println!("{}", value),
    //     None => {}
    // }

    let mut dl: DoublyLinkedList<i32> = DoublyLinkedList::new();
    dl.push_back(1);
    dl.push_back(2);
    dl.push_back(3);
    dl.shift(0);
    dl.get_size().map(|size| {
        for index in 0..*size {
            dl.left.get_top().map( |node| { println!("{}", node) });
            dl.shift(index);
        }
    });
    // dl.left.get_top().map( |node| { println!("{}", node) });
    // dl.shift(1);
    // dl.left.get_top().map( |node| { println!("{}", node) });

    // }
    // dl.shift(0);
    // dl.left.get_top().map( |node| { println!("{}", node) });
    // dl.shift(1);
    // dl.left.get_top().map( |node| { println!("{}", node) });
    // dl.shift(1);
    // dl.left.get_top().map( |node| { println!("{}", node) });

    // dl.left.get_top().map( |node| { println!("{}", node) });
    // match x {
    //     Some(value) => println!("{}", value),
    //     None => {}
    // }

}


