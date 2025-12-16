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

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty}
    }
}