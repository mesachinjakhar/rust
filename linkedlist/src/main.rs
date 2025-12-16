use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::vec;


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
    let mut a = String::from("hello");

    let b = &mut a; 

    // let c = *b; // moving it would leave "a" uninitialized.
    println!("{}", a); 
    let mut third = Node::new(20);

    let mut second = Node::new(10);
    second.next = Some(Box::new(third));

    let mut first = Node::new(5); 
    first.next = Some(Box::new(second));

    let head = Some(Box::new(first));

    let length = length(&head);
    println!("length is: {}", length);
    print_all_values(&head);

    let new_head = remove_head(head);
    print_all_values(&new_head);


    // let a = Rc::new(10);
    // let b = Rc::clone(&a);

    // println!("count {}", Rc::strong_count(&a)); // 2 , multiple ownership allowed

    // refcell , muttable borrows at runtime

    // let x = RefCell::new(10);
    // *x.borrow_mut() = 20;
    // println!("{:?}", x);

}


fn print_list(mut head: &Option<Box<Node>>) {
    while let Some(node) = head {
        println!("{}", node.val);
        head = &node.next;
    }
}

fn length(head: &Option<Box<Node>>) -> i32 {
    let mut count = 0; 
    let mut curr = head.as_ref();

    while let Some(node) = curr {
        count = count + 1;
        curr = node.next.as_ref();
    }
    count
}

fn print_all_values(head: &Option<Box<Node>>) {
    let mut curr = head.as_ref();

    while let Some(node) = curr {
        println!("{}", node.val);
        curr = node.next.as_ref();
    }
}

fn remove_head(head: Option<Box<Node>>) -> Option<Box<Node>>{
    let new_head = match head {
        Some(node) => node.next,
        None => None
    };

    new_head
}

fn remove_second(head: Option<Box<Node>>) -> Option<Box<Node>> {
    match head {
        Some(mut first) => {
            if let Some(second) = first.next {
                first.next = second.next
            }
            Some(first)
        },
        None => None,
    }
}

fn to_vec(head: &Option<Box<Node>>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let mut curr = head.as_ref();

    while let Some(node) = curr  {
        v.push(node.val);
        curr = node.next.as_ref();
    }
    v
}

fn remove_node_with_val(head: Option<Box<Node>>, x: i32) -> Option<Box<Node>> {
    let mut dummy = Box::new(Node {
        val: 0,
        next: head,
    });
    let mut curr = &mut dummy;

    while let Some(next_node) = curr.next.as_mut() {
        if next_node.val == x {
            curr.next = next_node.next.take();
        } else {
            curr = curr.next.as_mut().unwrap();
        }
    }

    dummy.next

} 
