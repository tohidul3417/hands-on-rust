use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    content: String,
}
fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents {content: "shirt".to_owned()});
    lockers.insert(2, Contents {content: "pants".to_owned()});
    lockers.insert(3, Contents {content: "cap".to_owned()});
    lockers.insert(4, Contents {content: "shoes".to_owned()});

    for (locker_number, content) in lockers.iter() {
        println!("Key: {:?}, Val: {:?}", locker_number, content);
    };
}
