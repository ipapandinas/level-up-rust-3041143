use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum ParseIsbnError {
    TooShort,
    TooLong,
    InvalidLastDigit,
}

impl FromStr for Isbn {
    type Err = ParseIsbnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 17 {
            return Err(ParseIsbnError::TooShort);
        } else if s.len() > 17 {
            return Err(ParseIsbnError::TooLong);
        }

        let digits = s.replace("-", "").into_bytes();
        let check_digit = calculate_check_digit(&digits[..digits.len() - 1]);

        if digits.last().unwrap_or(&0) != &check_digit {
            return Err(ParseIsbnError::InvalidLastDigit);
        }

        Ok(Isbn {
            raw: String::from(s),
            digits,
        })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let mut res = 0;
    for (idx, digit) in digits.iter().enumerate() {
        if idx % 2 == 0 {
            res += digit - b'0';
        } else {
            res += (digit - b'0') * 3;
        }
    }
    (10 - res % 10) % 10
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();
    let test = calculate_check_digit(&rust_in_action.digits);
    println!(
        "Rust in Action's ISBN-13 ({}) is valid with digit calculated: {}!",
        rust_in_action, test
    );
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
