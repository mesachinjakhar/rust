enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Associated function
impl Message {
    fn some_function() {
        println!("Hello from enum fn");
    }
}

fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    Message::some_function();

    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // Option enum

    let x = 5;
    // let y: Option<i32> = Some(5);
        let y: Option<i32> = None;

    let sum = x + y.unwrap_or(0);

}

fn route(_ip: IpAddrKind) {

}