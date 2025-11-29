fn main() {
    // panic stop the code execution immediatley
    // panic!("code crashed");

    // println!("this line never prints");

    // backtrace example
    a();

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
