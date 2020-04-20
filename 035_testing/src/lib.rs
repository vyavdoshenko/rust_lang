#[cfg(test)]
mod tests {
    use crate::module;

    #[test]
    fn test_equality() {
        assert_eq!(module::assert_eq_fn(), 5);
    }

    #[test]
    fn test_not_equality() {
        assert_ne!(module::assert_eq_fn(), 0);
    }

    #[test]
    fn test_bool() {
        assert!(module::assert_fn());
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        module::panic_fn();
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        panic!("Test panic in unit test");
    }
}

pub mod module {
    pub fn assert_fn() -> bool {
        return true;
    }

    pub fn assert_eq_fn() -> u32 {
        5
    }

    pub fn panic_fn() {
        panic!("Test panic in unit test")
    }
}