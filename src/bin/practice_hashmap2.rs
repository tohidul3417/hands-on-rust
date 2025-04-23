// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main () {
    let store = HashMap::from([
        ("Chairs", 5),
        ("Beds", 3),
        ("Tables", 2),
        ("Couches", 0)
    ]);

    let mut stock_total = 0;
    for (item, number) in store.iter() {
        stock_total += number;
        let stock_count = if *number == 0 {
            "out of stock".to_owned()
        } else {
            format!("{number:?}")
        };

        println!("{item}: {stock_count}");
    }

    println!("Total number of all items: {stock_total}");
}