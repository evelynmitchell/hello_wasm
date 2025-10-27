use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for greet function
    #[test]
    fn test_greet_normal_name() {
        let result = greet("World");
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_greet_empty_string() {
        let result = greet("");
        assert_eq!(result, "Hello, !");
    }

    #[test]
    fn test_greet_single_char() {
        let result = greet("A");
        assert_eq!(result, "Hello, A!");
    }

    #[test]
    fn test_greet_long_name() {
        let result = greet("Alexander");
        assert_eq!(result, "Hello, Alexander!");
    }

    #[test]
    fn test_greet_special_chars() {
        let result = greet("José");
        assert_eq!(result, "Hello, José!");
    }

    #[test]
    fn test_greet_numbers() {
        let result = greet("User123");
        assert_eq!(result, "Hello, User123!");
    }

    #[test]
    fn test_greet_spaces() {
        let result = greet("John Doe");
        assert_eq!(result, "Hello, John Doe!");
    }

    // Tests for add function - combinatorial testing
    #[test]
    fn test_add_positive_positive() {
        assert_eq!(add(5, 3), 8);
        assert_eq!(add(100, 200), 300);
    }

    #[test]
    fn test_add_positive_negative() {
        assert_eq!(add(5, -3), 2);
        assert_eq!(add(100, -50), 50);
    }

    #[test]
    fn test_add_negative_negative() {
        assert_eq!(add(-5, -3), -8);
        assert_eq!(add(-100, -200), -300);
    }

    #[test]
    fn test_add_negative_positive() {
        assert_eq!(add(-5, 3), -2);
        assert_eq!(add(-100, 50), -50);
    }

    #[test]
    fn test_add_with_zero() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(5, 0), 5);
        assert_eq!(add(0, 5), 5);
        assert_eq!(add(-5, 0), -5);
        assert_eq!(add(0, -5), -5);
    }

    #[test]
    fn test_add_boundary_values() {
        // Test with large positive numbers
        assert_eq!(add(1000000, 2000000), 3000000);
        // Test with large negative numbers
        assert_eq!(add(-1000000, -2000000), -3000000);
        // Test crossing zero
        assert_eq!(add(-5, 10), 5);
        assert_eq!(add(10, -10), 0);
    }

    #[test]
    fn test_add_max_values() {
        // Test near i32 limits (avoid overflow)
        let max_half = i32::MAX / 2;
        let result = add(max_half, max_half);
        assert_eq!(result, max_half * 2);
    }
}
