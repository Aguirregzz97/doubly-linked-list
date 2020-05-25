use std::collections::LinkedList;

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{head: None}
    }

    pub fn push(&mut self, elem:T) {
        self.head = Some(Box::new(Node{ elem: elem, next: self.head.take(), }));
    }

    pub fn pop(&mut self) -> Option<T> {
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

    pub fn get_top(self) -> Option<T> {
        match self.head {
            Some(node) => {
                Some(node.elem)
            },
            None => None
        }
    }

}

pub struct DoublyList<T> {
    left: List<T>,
    right: List<T>,
}

impl<T> DoublyList<T> {



}


fn main() {
    let mut list:List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    // let mut head = list.pop();

    let mut head = list.pop();
    let mut x = list.get_top();
    match x {
        Some(value) => println!("{}", value),
        None => {}
    }
}


