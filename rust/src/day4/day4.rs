#[derive(Debug)]
struct Password {
    raw: i32,
}

impl Password {
    fn new(raw: i32) -> Password {
        Password { raw }
    }

    fn contains_only_two_adjacent_digits(&self) -> bool {
        let mut last_digit = std::i32::MAX;
        let mut current = self.raw.clone();
        let mut count = 0;
        loop {
            let digit = current % 10;
            if digit == last_digit {
                count += 1;
            } else {
                if count == 2 {
                    return true;
                }
                count = 1;
            }
            last_digit = digit;
            if current < 10 {
                return count == 2;
            }
            current = current / 10;
        }
    }

    fn digits_increase(&self) -> bool {
        let mut last_digit = std::i32::MAX;
        let mut current = self.raw.clone();
        loop {
            let digit = current % 10;
            if digit > last_digit {
                return false;
            }
            last_digit = digit;
            if current < 10 {
                return true;
            }
            current = current / 10;
        }
    }
}


fn main() {
    let mut count = 0;
    for i in 367479..893698 {
        let password = Password::new(i);
        if password.contains_only_two_adjacent_digits() && password.digits_increase() {
            count += 1;
        }
    }
    println!("count: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_adjacent_digits() {
        assert_eq!(Password::new(111111).contains_only_two_adjacent_digits(), false);
        assert_eq!(Password::new(223450).contains_only_two_adjacent_digits(), true);
        assert_eq!(Password::new(111122).contains_only_two_adjacent_digits(), true);
        assert_eq!(Password::new(123444).contains_only_two_adjacent_digits(), false);
    }

    #[test]
    fn test_digits_increase() {
        assert_eq!(Password::new(111111).digits_increase(), true);
        assert_eq!(Password::new(223450).digits_increase(), false);
        assert_eq!(Password::new(123789).digits_increase(), true);
        assert_eq!(Password::new(101010).digits_increase(), false);
    }
}
