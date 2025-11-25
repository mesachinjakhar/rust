enum VectorData {
    Int(i32),
    Float(f32),
}

fn main() {
    let v1: Vec<i32> = Vec::new();

    // using macro!
    let v2: Vec<i32> = vec![];

    // mutable vec
    let mut v3: Vec<i32> = Vec::new();

    // push at end of vec
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    v3.push(9);
    
    // via insert
    // put 10 at index 1 
    v3.insert(1, 10);

    // remove element at a specfic location 
    v3.remove(1); // remove 10

    match v3.get(2) {
        Some(v) => println!("{}", v),
        None => println!("there is no index"),
    };

    // imm iterator on vector
    let v3_iter = v3.iter();

    // loop iter
    for i in v3_iter {
        println!("{}", i);
    }

    // store diff data types 
    let vec4 = vec![VectorData::Int(32), VectorData::Float(54.43)];

    let third = &v3[2];
    println!("the third element is: {}", third);

    println!("{:?}", v3);
}
