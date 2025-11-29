use std::fs::File;

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
        Err(error) => panic!("problem opening the file: {:?}", error),
    };


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
