use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;


// custom type for validation
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value <1 || value > 100 {
            panic!("guess value must be between 1 and 100, get: {}.", value)
        }

        Guess {value: value}
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    // panic stop the code execution immediatley
    // panic!("code crashed");

    // println!("this line never prints");

    // backtrace example
    // a();

    // run with:  RUST_BACKTRACE=1 cargo run 

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f, 
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }
            other_err => {
                panic!("problem opening the file: {:?}", other_err);
            }
        }
    };

    let v = vec![1, 2, 3];

    v[99];




    // unwrap 
    // let f2 = File::open("hello2.txt").unwrap();
    // unwrap will panic if .open method return error

    // expect
    // let f3 = File::open("hello2.txt").expect("Failed to open hello2.txt");
    // using expect we pass the custom error message to panic macro


}

// backtrace example 
fn a() {
    b();
}

fn b() {
    c(22)
}

fn c(num: i32) {
    if num == 22 {
        panic!("dont pass 22");
    }
}

// the ? operator
fn read_username_from_file() -> Result<String, io::Error>{
    let mut f = File::open("hello.txt")?;
    // "?" return the error immediately

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}