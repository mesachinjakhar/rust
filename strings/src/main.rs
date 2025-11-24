fn main() {
    let mut s = "Hello World".to_string();

    let result = first_word(&s);

    // s.clear();
    // above wont work because clear method wants mutable reference but since we already taken & in first_word , we cant borrow it as mut further

    println!("{result}");


}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}