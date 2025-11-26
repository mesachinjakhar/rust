use std::collections::VecDeque;

fn main() {
    // init empty queue 
    let mut queue: VecDeque<i32> = VecDeque::new();

    // push back to queue
    queue.push_back(10);
    queue.push_back(20);
    queue.push_back(30);
    println!("{:?}", queue);
}
