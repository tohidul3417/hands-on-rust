fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {

}

#[cfg(test)]

mod test {
    use crate::*;
    #[test]
    fn check_all_caps() {
        let result =  all_caps("hello");
        let expected_result = String::from("HELLO");
        assert_eq!(result, expected_result, "All characters must be uppercase!");
    }
}