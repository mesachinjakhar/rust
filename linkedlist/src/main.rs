use std::rc::{Rc, Weak};
use std::cell::RefCell;


// Doubly linked list 
pub struct DNode {
    val: i32,
    next: Option<Rc<RefCell<DNode>>>,
    prev: Option<Weak<RefCell<DNode>>>
}

impl DNode {
    pub fn new(val: i32) -> Rc<RefCell<DNode>> {
        Rc::new(RefCell::new(DNode { val: val, 
            next: None, 
            prev: None }))
    }

    pub fn push_front(head: &mut Option<Rc<RefCell<DNode>>>, val: i32) {
        let new_node = Self::new(val);

        match head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head);
                *head = Some(new_node);
            }
            None => {
                *head = Some(new_node)
            }
        }
    }
}





pub struct Node {
    val: i32,
    next: Option<Box<Node>>
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val: val,
            next: None,
        }
    }
}

fn main() {
    // let mut third = Node::new(20);

    // let mut second = Node::new(10);
    // second.next = Some(Box::new(third));

    // let mut first = Node::new(5); 
    // first.next = Some(Box::new(second));

    // let head = Some(Box::new(first));
    // print_list(&head);

    let a = Rc::new(10);
    let b = Rc::clone(&a);

    println!("count {}", Rc::strong_count(&a)); // 2 , multiple ownership allowed

    // refcell , muttable borrows at runtime

    let x = RefCell::new(10);
    *x.borrow_mut() = 20;
    println!("{:?}", x);
}


fn print_list(mut head: &Option<Box<Node>>) {
    while let Some(node) = head {
        println!("{}", node.val);
        head = &node.next;
    }
}