use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::io::Read;


fn normalize_and_split(text: &str) -> Vec<String> {

    let mut cleaned = String::with_capacity(text.len());

    for ch in text.chars() {
        if ch.is_alphanumeric() || ch == '\''{
            cleaned.push(ch)
    } else {
        cleaned.push(' ');
    }

}

cleaned.split_whitespace()
    .map(|s| s.to_string())
    .collect()
}

fn word_freq_map(text: &str) -> HashMap<String, usize> {
    let mut freq: HashMap<String, usize> = HashMap::new();

    let words = normalize_and_split(text);

    for word in words {
        *freq.entry(word).or_insert(0) += 1;
    }
    freq
}


fn print_sorted(freq: &HashMap<String, usize>) {
    // We borrow counts & copy because usize is Copy.
    let mut vec: Vec<(String, usize)> = freq
        .iter()
        .map(|(k, &v)| (k.clone(), v))
        .collect();


    vec.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then_with(|| a.0.cmp(&b.0))
    });

    for (word, count) in vec {
        println!("{:>6}  {}", count, word);
    }
}

fn main() -> io::Result<()> {
    // 1: make empty hashmap 
    let mut map: HashMap <String, usize> = HashMap::new();
    // we make it mut because later we want to insert remove entries

    // 2: insert
    map.insert("apple".to_string(), 3);

    // 3: get value
    if let Some(&count) = map.get("apple") {
        println!("apple: {}", count);
    }
    // NOTICE! we borrowed the value. we didnt take the ownership
    // .get method return the option , so we have to use Some() to check if the key is avaible or not
    // if key is not there, we gets None in return 

    // 4: Update entry
    *map.entry("apple".to_string()).or_insert(0) +=1;
    // entry returns the mut &V so we deref and then insert + 1; 

    // 5: iterate without taking ownership use &map 
    for (k, v) in &map {
        println!("key: {}, value: {}", k, v);
    }

    // 6: insert returns Option<old_value> if existed
    let old = map.insert("apple".to_string(), 7); 
    println!("old value {}", old.unwrap());

    // Hashing and the Hash trait
    // 1: A key type must implement std::hash::Hash and Eq.
    // 2: Standard types (String, &str, integers) already implement these.

    #[derive(Hash, Eq, PartialEq)]
    struct MyKey {
        a: i32,
        b: i32
    }

    // Internally HashMap uses a hashing algorithm and a randomized hash state by default (to avoid certain denial-of-service attacks).
    // Average-case HashMap operations (insert, get, remove) are O(1) time.

    // Code to take str as input and create hashmap of each word
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let freq = word_freq_map(&input);

    print_sorted(&freq);

    Ok(())

}

