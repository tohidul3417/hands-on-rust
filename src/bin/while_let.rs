fn main() {
    let mut data = Some(5);
    while let Some(num) = data {
        println!("loop");
        data = None;
    }
    println!("Done!");

    let numbers = vec![32, 553, 3342, 55343 , 323];
    let mut number_iterator = numbers.iter();
    while let Some(num) = number_iterator.next() {
        dbg!(num);
    }

}