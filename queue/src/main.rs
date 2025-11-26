use std::collections::VecDeque;
use std::collections::vec_deque::Iter;
use serde::Serialize;

pub struct CircularQueue<T> {
    inner: VecDeque<T>,
    capacity: usize
}

impl <T> CircularQueue<T> where T : Clone {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
            capacity;
        }
    }

    pub fn len(&self) -> usize{
        &self.inner.len()
    }

    pub fn push (&mut self, item: T) {
        if self.capacity == 0 {
            return;
        }
        if self.capacity == self.len() {
            self.pop_back();
        }
        self.inner.push_front(item);
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.inner.iter()
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.inner.iter().map(|x | x.clone()).collect()
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
