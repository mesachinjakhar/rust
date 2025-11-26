use std::collections::VecDeque;
use std::collections::vec_deque::Iter;
use serde::Serialize;

// pub struct CircularQueue<T> {
//     inner: VecDeque<T>,
//     capacity: usize
// }

// impl <T> CircularQueue<T> where T : Clone { // why clone , because we only accept data which implement the clone trait on it
//     pub fn with_capacity(capacity: usize) -> Self {
//         Self {
//             inner: VecDeque::with_capacity(capacity),
//             capacity: capacity
//         }
//     }

//     pub fn len(&self) -> usize{
//         self.inner.len()
//     }

//     pub fn push (&mut self, item: T) {
//         if self.capacity == 0 {
//             return;
//         }
//         if self.capacity == self.len() {
//             self.inner.pop_back();
//         }
//         self.inner.push_front(item);
//     }

//     pub fn iter(&self) -> Iter<'_, T> {
//         self.inner.iter()
//     }

//     pub fn to_vec(&self) -> Vec<T> {
//         self.inner.iter().map(|x | x.clone()).collect()
//     }
// }


// Queue from scratch 
struct CircularQueue <T> {
    buff: Vec<Option<T>>,
    len: usize,
    capacity: usize,
    head: usize
}

impl <T> CircularQueue<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buff: vec![None; capacity];
            head: 0,
            len: 0,
            capacity: capacity,
        }
    }

    // number of elements stored 
    pub fn len (&self) -> usize {
        self.len 
    }

    // queue empty 

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // is this queue full 
    pub fn is_full (&self) -> bool {
        self.len == self.capacity  && self.capacity != 0
    }

    // compute tail index 
    pub fn tail_index(&self) -> usize {
        if self.capacity == 0 {
            return 0;
        }
        else {
        (self.head + self.len) % self.capacity
        }
    }

    // return peek element
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        } else {
            self.buff[self.head].as_ref();
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        else {
            let out = self.buff[self.head].take();
            self.head = (self.head + 1) % self.capacity;
            self.len -= 1;
            out
        }

    }

    // Push but overwrite oldest element if queue is full.
    // Returns Some(overwritten_item) if something was overwritten.
    pub fn push(&mut self, item: T) -> Option<T> {
        if self.capacity == 0  {
            return Some(item);
        }

        let idx = self.tail_index();
        let overwritten = self.buff[idx].take();
        self.buff[idx] = Some(item);

        if self.is_full() {
            self.head =  (self.head + 1) % self.capacity;
            overwritten
        } else {
            self.len  += 1;
            None
        }
    }
}

fn main() {
    // init empty queue 
    let mut queue: VecDeque<i32> = VecDeque::new();

    // push back to queue
    queue.push_back(10);
    queue.push_back(20);
    queue.push_back(30);
    println!("{:?}", queue);

    // remove from front 
    if let Some(front) = queue.pop_front() { // pop_front returns the Option Some with Remove item if avaible or None if nothing is at front of queue. 
        println!("removed {}", front);
    }

    // peek front element 
    if let Some(front) = queue.front() {
        println!("front element is: {}", front);
    }

    // queue size 
    println!("size of queue: {}", queue.len());

    // check if empty
    println!("is empty:  {}", queue.is_empty());


    // Circular queue implementation

}
