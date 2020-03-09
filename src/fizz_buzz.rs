// Function that returns a boolean value
pub fn fizz_buzz(string1: String, string2: String, int1: i32, int2: i32, limit: i32) -> Result<Vec<String>, String> {
    if limit < 1 {
        return Err(String::from("InvalidParameters: Limit should be greater than 1."));
    }

    if int1 == int2 {
        return Err(String::from("InvalidParameters: The 2 multiples should be different."));
    }

    if string1 == string2 {
        return Err(String::from("InvalidParameters: The 2 strings should be different."));
    }

    let mut vec: Vec<String> = Vec::new();
    for n in 1..limit + 1 {
        if can_be_divide(n, int1 * int2) {
            vec.push(String::from("FizzBuzz"));
        } else if can_be_divide(n, int1) {
            vec.push(String::from("Fizz"));
        } else if can_be_divide(n, int2) {
            vec.push(String::from("Buzz"));
        } else {
            vec.push(n.to_string());
        }
    }
    return Ok(vec);
}

fn can_be_divide(value: i32, divider: i32) -> bool {
    return value % divider == 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_divide_ok() {
        assert!(can_be_divide(6, 3));
    }

    #[test]
    fn can_be_divide_ko() {
        assert!(!can_be_divide(5, 3));
    }

    #[test]
    fn valid_fizz_buzz() {
        let res = fizz_buzz(String::from("Fizz"), String::from("Buzz"), 3, 5, 15);
        assert!(!res.is_err());
        assert_eq!(res.ok().unwrap(), ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"]);
    }

    #[test]
    fn invalid_fizz_buzz_no_limit() {
        let res = fizz_buzz(String::from("Fizz"), String::from("Buzz"), 3, 5, 0);
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "InvalidParameters: Limit should be greater than 1.");
    }

    #[test]
    fn invalid_fizz_buzz_int1_equals_int2() {
        let res = fizz_buzz(String::from("Fizz"), String::from("Buzz"), 3, 3, 15);
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "InvalidParameters: The 2 multiples should be different.");
    }

    #[test]
    fn invalid_fizz_buzz_string1_equals_string2() {
        let res = fizz_buzz(String::from("Fizz"), String::from("Fizz"), 3, 5, 15);
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "InvalidParameters: The 2 strings should be different.");
    }
}
