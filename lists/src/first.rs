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

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                None
            }
            Link::More(node) => {
                self.head = node.next;
                Some(node.ele)
            }
        }
    }
}

#[cfg(test)]

mod test {
    use crate::first::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(5);
        list.push(4);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Push some more 
        list.push(2);
        list.push(1);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

    }
}