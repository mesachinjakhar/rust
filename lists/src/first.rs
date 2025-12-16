use std::mem;

#[derive(Debug)]

pub struct List {
    head: Link,
}
#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>)
} 
#[derive(Debug)]
struct Node {
    ele: i32,
    next: Link
}



impl List {
    pub fn new() -> Self {
        List { head: Link::Empty}
    }
    pub fn push(&mut self, val: i32) {
        let new_node = Node {
            ele: val,
            next: mem::replace(&mut self.head, Link::Empty)
        };
        self.head = Link::More(Box::new(new_node));
    }
}