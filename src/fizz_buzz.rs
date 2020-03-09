// Function that returns a boolean value
pub fn fizz_buzz(min: i32, max: i32) -> Result<Vec<String>, String> {
    if min == max {
        return Err(String::from("InvalidParameters: The 2 strings should be different."));
    } else if min > max {
        return Err(String::from("InvalidParameters: The min should be lower than the max."));
    }

    let mut vec: Vec<String> = Vec::new();
    for n in min..max + 1 {
        if can_be_divide(n, 15) {
            vec.push(String::from("FizzBuzz"));
        } else if can_be_divide(n, 3) {
            vec.push(String::from("Fizz"));
        } else if can_be_divide(n, 5) {
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
        let res = fizz_buzz(1, 15);
        assert!(!res.is_err());
        assert_eq!(res.ok().unwrap(), ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"]);
    }

    #[test]
    fn invalid_fizz_buzz_min_equals_max() {
        let res = fizz_buzz(15, 15);
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "InvalidParameters: The 2 strings should be different.");
    }

    #[test]
    fn invalid_fizz_buzz_min_greater_than_max() {
        let res = fizz_buzz(150, 15);
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "InvalidParameters: The min should be lower than the max.");
    }
}
