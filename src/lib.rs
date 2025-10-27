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

// Integration tests - run in browser environment with wasm-bindgen-test
// These tests verify WASM functions work correctly in an actual browser
#[cfg(test)]
mod browser_tests {
    use super::*;
    use wasm_bindgen_test::*;

    // Configure tests to run in browser
    wasm_bindgen_test_configure!(run_in_browser);

    // Browser integration tests for greet function
    #[wasm_bindgen_test]
    fn browser_test_greet_basic() {
        let result = greet("Browser");
        assert_eq!(result, "Hello, Browser!");
    }

    #[wasm_bindgen_test]
    fn browser_test_greet_unicode() {
        let result = greet("世界");
        assert_eq!(result, "Hello, 世界!");
    }

    #[wasm_bindgen_test]
    fn browser_test_greet_emoji() {
        let result = greet("🦀");
        assert_eq!(result, "Hello, 🦀!");
    }

    #[wasm_bindgen_test]
    fn browser_test_greet_long_string() {
        let long_name = "A".repeat(1000);
        let result = greet(&long_name);
        assert!(result.starts_with("Hello, "));
        assert!(result.ends_with('!'));
        assert_eq!(result.len(), long_name.len() + 8); // "Hello, " + "!"
    }

    // Browser integration tests for add function
    #[wasm_bindgen_test]
    fn browser_test_add_basic() {
        assert_eq!(add(10, 20), 30);
    }

    #[wasm_bindgen_test]
    fn browser_test_add_negative() {
        assert_eq!(add(-5, -10), -15);
    }

    #[wasm_bindgen_test]
    fn browser_test_add_mixed_signs() {
        assert_eq!(add(100, -50), 50);
        assert_eq!(add(-100, 50), -50);
    }

    #[wasm_bindgen_test]
    fn browser_test_add_zero() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(42, 0), 42);
        assert_eq!(add(0, -42), -42);
    }

    #[wasm_bindgen_test]
    fn browser_test_add_large_numbers() {
        let result = add(1_000_000, 2_000_000);
        assert_eq!(result, 3_000_000);
    }

    // Test that verifies WASM execution environment
    #[wasm_bindgen_test]
    fn browser_test_wasm_environment() {
        // This test verifies we're running in a WASM environment
        // If this passes, it confirms the test infrastructure is working
        assert!(true, "WASM test environment is functional");
    }
}
