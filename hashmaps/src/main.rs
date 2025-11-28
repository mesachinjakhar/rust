use std::collections::HashMap;

fn main() {
    // 1 make empty hashmap 
    let mut map: HashMap <String, usize> = HashMap::new();
    // we make it mut because later we want to insert remove entries

    // 2 insert
    map.insert("apple".to_string(), 3);

    // 3 get value
    if let Some(&count) = map.get("apple") {
        println!("apple: {}", count);
    }
    // NOTICE! we borrowed the value. we didnt take the ownership
    // .get method return the option , so we have to use Some() to check if the key is avaible or not
    // if key is not there, we gets None in return 

    // 4 Update entry
    *map.entry("apple".to_string()).or_insert(0) +=1;
    // entry returns the mut &V so we deref and then insert + 1; 

    // 5 iterate without taking ownership use &map 
    for (k, v) in &map {
        println!("key: {}, value: {}", k, v);
    }

    // 6 insert returns Option<old_value> if existed
    let old = map.insert("apple".to_string(), 7); 
    println!("old value {}", old.unwrap());
 
}

