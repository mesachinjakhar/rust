use std::fs::File;
use std::io::ErrorKind;

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

    // unwrap 
    let f2 = File::open("hello2.txt").unwrap();


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
