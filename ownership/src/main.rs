fn main() {
    // ------ Ownership rules -----
    // 1. Each value in Rust has a variable thats called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped. 

    { // s is not valid here, its not ye declared
        let s = "hello"; // s is valid from this point forward
        // do stuff with s 
    } // this scope is now over, and s is now no longer valid
}
