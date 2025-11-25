use unicode_segmentation::{ UnicodeSegmentation};

fn main() {
    let mut s = "Hello World".to_string();

    // let result = first_word(&s);

    // s.clear();
    // above wont work because clear method wants mutable reference but since we already taken & in first_word , we cant borrow it as mut further

    // println!("{result}");

    s.push_str(" Again");
    println!("{}", s);
    s.replace_range(.., "nothing");
    println!("{}", s);

    // concatinate

    let s1 = String::from("hello");
    let s2 = String::from(" world");

    let s3 = s1.clone() + &s2;

    println!("{}", s3);

    let s4 = format!("{} -{}", s1, s2);
    println!("{}", s4);

    let s5 = ["first", "second"].concat();
    let s6 = concat!("first", "second");
    println!("{}", s6);

    // indexing
    let c1 = "🦀🦀🦀🦀";
    let c2 = &c1[0..4];
    println!("{}", c2);

    for i in "नमस्ते".bytes() {
        println!("{}", i);
    }

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
    }
    

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