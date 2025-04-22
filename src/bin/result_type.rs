// Topic: Result

// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * Implement Debug print functionality using `derive`
#[derive(Debug)]
struct Adult {
    //   * The structure must contain the person's name and age
    name: String,
    age: u8,
}

// * Implement a `new` function for the `Adult` structure that returns a Result:
impl Adult {
    fn new(name: String, age: u8) -> Result<Self, &'static str> {
        //   * The Ok variant should contain the initialized structure, but only
        //     if the person is aged 21 or older
        if age >= 21 {
            Ok(Self {
                name,
                age
            })
        } else {
            //   * The Err variant should contain a String (or &str) that explains why
            //     the structure could not be created
            Err("Error creating an instance of the Adult. Minimum required age is 21")
        }
    }
}

fn main() {
    // * Instantiate two `Adult` structures:
    //   * One should be aged under 21
    let child: Result<_, &str> = Adult::new(String::from("Rusty"), 13);
    //   * One should be 21 or over
    let adult2: Result<_, &str> = Adult::new(String::from("Christy"), 34);


    // * Use `match` to print out a message for each `Adult`:
    match child {
        //   * For the Ok variant, print any message you want
        Ok(res) => println!("{} is {} years old.", res.name, res.age),
         //   * For the Err variant, print out the error message
        Err(e) => println!("{:?}", e),
    }
    match adult2 {
        //   * For the Ok variant, print any message you want
        Ok(res) => println!("{} is {} years old.", res.name, res.age),
         //   * For the Err variant, print out the error message
        Err(e) => println!("{:?}", e),
    }
}

