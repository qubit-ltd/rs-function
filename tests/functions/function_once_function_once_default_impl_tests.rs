// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Comprehensive tests for FunctionOnce trait and BoxFunctionOnce

// ============================================================================
// FunctionOnce Trait Tests - Core Functionality
// ============================================================================

#[cfg(test)]
mod function_once_default_impl_tests {
    use qubit_function::BoxFunctionOnce;
    use qubit_function::FunctionOnce;

    /// Cloneable custom one-time function for testing to_xxx_once() methods
    #[derive(Clone)]
    struct CloneableCustomFunctionOnce {
        multiplier: i32,
    }

    impl FunctionOnce<i32, i32> for CloneableCustomFunctionOnce {
        fn apply(self, input: &i32) -> i32 {
            input * self.multiplier
        }
        // Does not override any to_xxx_once() methods, testing default
        // implementations
    }

    #[test]
    fn test_custom_with_captured_value() {
        let captured = [1, 2, 3];
        let custom = CloneableCustomFunctionOnce { multiplier: 2 };

        let func = BoxFunctionOnce::new(move |x: &i32| {
            let base = custom.apply(x);
            base + captured.len() as i32
        });

        assert_eq!(func.apply(&10), 23); // 10 * 2 + 3
    }
}

// ============================================================================
// FunctionOnce Debug and Display Tests
// ============================================================================
