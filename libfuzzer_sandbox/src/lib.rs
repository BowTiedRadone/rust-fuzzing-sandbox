//! A simple library to test cargo fuzz using feature gating.

/// This function is always available
pub fn hello_basic() -> String {
    "Hello from basic function!".to_string()
}

/// This function is only available during tests or when the "testing" feature
/// is enabled.
#[cfg(any(test, feature = "testing"))]
pub fn hello_testing() -> String {
    "Hello from testing feature function!".to_string()
}

/// This module contains test utilities.
#[cfg(any(test, feature = "testing"))]
pub mod test_utils {
    pub fn create_test_data() -> Vec<i32> {
        vec![1, 2, 3, 4, 5]
    }

    pub fn validate_test_data(data: &[i32]) -> bool {
        data.len() == 5 && data[0] == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_function() {
        assert_eq!(hello_basic(), "Hello from basic function!");
    }

    #[test]
    fn test_testing_function() {
        assert_eq!(hello_testing(), "Hello from testing feature function!");
    }

    #[test]
    fn test_utils_work() {
        let data = test_utils::create_test_data();
        assert!(test_utils::validate_test_data(&data));
    }
}
