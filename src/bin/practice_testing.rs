// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}


// * Create at least two test cases for each function.
#[cfg(test)]

mod test {
    use crate::*;
    #[test]
    fn check_clamp() {
        assert_eq!(clamp(45, 55, 98), 55, "The result should be 55");
        assert_eq!(clamp(313, 32, 245), 245, "The result should be 245");
    }

    #[test]
    fn check_div() {
        assert_eq!(div(54, 2), Some(27), "The result should be Some(27)");
        assert_eq!(div(100, 4), Some(25), "The result should be Some(25)");
    }

    #[test]
    fn check_zero_division() {
        assert_eq!(div(4, 0), None, "The result should be 'None'");
    }

    #[test]
    fn check_concat() {
        assert_eq!(concat("a", "b"), String::from("ab"), "The result should be 'ab'");
        assert_eq!(concat("And", "rew"), String::from("Andrew"), "The result should be 'Andrew'");
    }
}