#[cfg(test)]
mod tests {
    #[test]
    fn test_equality() {
        assert_eq!(super::assert_eq_fn(), 5);
    }

    #[test]
    fn test_not_equality() {
        assert_ne!(super::assert_eq_fn(), 0);
    }

    #[test]
    fn test_bool() {
        assert!(super::assert_fn());
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        super::panic_fn();
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        panic!("Test panic in unit test");
    }
}


pub fn assert_fn() -> bool {
    return true;
}

fn assert_eq_fn() -> u32 {
    5
}

fn panic_fn() {
    panic!("Test panic in unit test")
}
