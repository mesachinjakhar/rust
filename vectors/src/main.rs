fn main() {
    let v1: Vec<i32> = Vec::new();

    // using macro!
    let v2: Vec<i32> = vec![];

    // mutable vec
    let mut v3: Vec<i32> = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    v3.push(9);

    println!("{:?}", v3);
}
