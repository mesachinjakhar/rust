use std::collections::VecDeque;

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
}
