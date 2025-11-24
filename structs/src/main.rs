 #[derive(Debug)]
struct User {
    username: String,
    age: u32
}


fn main() {
    let user1 = User {
        username: "mesachinjakahr".to_string(), 
        age: 23
    };
    println!("{:?}", user1);
}
