fn main() {
    let may_be_user = Some("Johnny");

    match may_be_user {
        Some(user) => println!("{user}"),
        None => println!("No user found!"),
    };

    // Better to use if let instead of match if we are interested in only one scenario
    let may_be_number = Some(345);
    if let Some(number) = may_be_number {
        println!("{number}");
    };
}