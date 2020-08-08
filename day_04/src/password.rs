const MAX_ADJACENT_DIGITS: u8 = 2;

pub fn is_valid(password: &u32) -> bool {
    if password < &100_000 || password > &999_999 {
        return false;
    }

    let digits: Vec<u8> = password
        .to_string()
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect();

    let mut last_digit = digits.get(0).unwrap();
    let mut adjacent_digits_count = 1u8;
    let mut had_adjacent_digits = false;

    for digit in digits.iter().skip(1) {
        if digit < last_digit {
            // Values must always increase or stay the same
            return false;
        }

        if !had_adjacent_digits {
            if last_digit == digit {
                adjacent_digits_count += 1;
            } else {
                if adjacent_digits_count == MAX_ADJACENT_DIGITS {
                    had_adjacent_digits = true;
                }

                adjacent_digits_count = 1;
            }
        }

        last_digit = digit;
    }

    had_adjacent_digits || adjacent_digits_count == MAX_ADJACENT_DIGITS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let password: u32 = 111111;
        assert_eq!(is_valid(&password), false);
    }

    #[test]
    fn test_case_2() {
        let password: u32 = 112233;
        assert_eq!(is_valid(&password), true);
    }

    #[test]
    fn test_case_3() {
        let password: u32 = 123444;
        assert_eq!(is_valid(&password), false);
    }

    #[test]
    fn test_case_4() {
        let password: u32 = 111122;
        assert_eq!(is_valid(&password), true);
    }

    #[test]
    fn test_case_5() {
        let password: u32 = 444567;
        assert_eq!(is_valid(&password), false);
    }

    #[test]
    fn test_case_6() {
        let password: u32 = 444467;
        assert_eq!(is_valid(&password), false);
    }

    #[test]
    fn test_fail_decreasing_value() {
        let password: u32 = 111110;
        assert_eq!(is_valid(&password), false);
    }

    #[test]
    fn test_fail_no_adjacent_digits() {
        let password: u32 = 123456;
        assert_eq!(is_valid(&password), false);
    }
}
