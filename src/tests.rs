fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn times_two(num: i32) -> i32 {
    num * 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(true);
    }

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(1, 1);
    }

    #[test]
    fn is_true_when_even() {
        use super::is_even;
        assert!(is_even(2));
    }

    #[test]
    fn is_true_when_uneven() {
        use super::is_even;
        assert_eq!(false, is_even(5));
    }

    #[test]
    fn return_twice_of_positive_numbers() {
        use super::times_two;
        assert_eq!(2*2, times_two(2));
    }

    #[test]
    fn return_twice_of_negative_numbers() {
        use super::times_two;
        assert_eq!(2*-2, times_two(-2));
    }
}
