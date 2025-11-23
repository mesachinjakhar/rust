fn main() {
    // ------ Ownership rules -----
    // 1. Each value in Rust has a variable thats called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped. 

    { // s is not valid here, its not ye declared
        let s1 = String::from("hello"); // s1 is valid from this point forward
        let s2 = s1.clone();
        takes_ownership(s2);

        let mut s3 = String::from("hello"); // s is valid from this point forward
        takes_reference(&s3);
        takes_mut_reference(&mut s3);

        println!("{}", s3) // we can use s3 here also
    } // this scope is now over, and s is now no longer valid
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_reference(some_string: &String) {
    println!("{}", some_string);
}

fn takes_mut_reference(some_string: &mut  String ) {
    println!("{}", some_string);
}