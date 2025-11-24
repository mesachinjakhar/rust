 #[derive(Debug)]
struct User {
    username: String,
    age: u32
}

// method syntax 

impl User {
    fn greet(&self) {
        println!("Hello {}!", self.username);
    }
}

// Associated Functions (Self::new pattern) 
impl User {
    fn new(username: String, age: u32) -> Self {
        Self { username, age }
    }
}

// tuple struct 
struct Color (u8,u8,u8);

// Unit-Like Structs
// Often used for:
// 1: Traits
// 2: Zero-sized types
// 3: Phantom types

struct Marker;


fn main() {
    let user1 = User {
        username: "mesachinjakahr".to_string(), 
        age: 23
    };

    user1.greet();

    let user2 = User {
        username: "medeepakjakhar".to_string(),
        ..user1
    };

    let user = User::new(String::from("Dave"), 42);

    println!("{:?}", user2);

    let red = Color(255,0,0);

    println!("{}", red.0);

    let m = Marker;



}
