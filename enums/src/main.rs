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
 #[derive(Debug)]
enum UsState {
    Alaska,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red
        }
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

    // match enum

    let coin = Coin::Quarter(UsState::Alaska);

    println!("{}", value_in_coin(coin));

    // if let syntax

    let some_value = Some(9);

    if let Some(9) = some_value {
        println!("nine");
    };

    let mut light = TrafficLight::Red;

    for _ in 0..5 {
        println!("current light : {:?}", light);
        light = light.next();
    }




}

fn route(_ip: IpAddrKind) {

}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        }
    }
}