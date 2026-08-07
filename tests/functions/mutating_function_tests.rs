// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for MutatingFunction types (stateless Fn(&mut T) -> R)

use qubit_function::ArcMutatingFunction;
use qubit_function::BoxMutatingFunction;
use qubit_function::MutatingFunction;
use qubit_function::MutatingFunctionOnce;
use qubit_function::RcMutatingFunction;

// ============================================================================
// BoxMutatingFunction Tests
// ============================================================================

/// Test struct that implements MutatingFunction to test default methods
struct TestMutatingFunction {
    multiplier: i32,
}

impl TestMutatingFunction {
    fn new(multiplier: i32) -> Self {
        TestMutatingFunction { multiplier }
    }
}

impl MutatingFunction<i32, i32> for TestMutatingFunction {
    fn apply(&self, input: &mut i32) -> i32 {
        let old_value = *input;
        *input *= self.multiplier;
        old_value
    }
}

impl Clone for TestMutatingFunction {
    fn clone(&self) -> Self {
        TestMutatingFunction {
            multiplier: self.multiplier,
        }
    }
}

// ============================================================================
// MutatingFunction Debug and Display Tests
// ============================================================================

#[cfg(test)]
mod test_box_mutating_function {
    use super::BoxMutatingFunction;
    use super::MutatingFunction;

    #[test]
    fn test_new() {
        let func = BoxMutatingFunction::new(|x: &mut i32| {
            *x += 1;
            *x
        });
        let mut value = 5;
        assert_eq!(func.apply(&mut value), 6);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_with_different_types() {
        // String
        let string_func = BoxMutatingFunction::new(|s: &mut String| {
            let old_len = s.len();
            s.push('!');
            old_len
        });
        let mut text = String::from("hello");
        assert_eq!(string_func.apply(&mut text), 5);
        assert_eq!(text, "hello!");

        // Vec
        let vec_func = BoxMutatingFunction::new(|v: &mut Vec<i32>| {
            let old_len = v.len();
            v.push(42);
            old_len
        });
        let mut numbers = vec![1, 2, 3];
        assert_eq!(vec_func.apply(&mut numbers), 3);
        assert_eq!(numbers, vec![1, 2, 3, 42]);

        // bool
        let bool_func = BoxMutatingFunction::new(|b: &mut bool| {
            let old = *b;
            *b = !*b;
            old
        });
        let mut flag = true;
        assert!(bool_func.apply(&mut flag));
        assert!(!flag);
    }

    #[test]
    fn test_and_then() {
        let func = BoxMutatingFunction::new(|x: &mut i32| {
            *x *= 2;
            *x
        })
        .and_then(|x: &i32| *x + 10);

        let mut value = 5;
        let result = func.apply(&mut value);
        assert_eq!(result, 20); // (5 * 2) + 10
        assert_eq!(value, 10); // Input only modified by first function
    }

    #[test]
    fn test_and_then_multiple_chains() {
        let func = BoxMutatingFunction::new(|x: &mut i32| {
            *x += 1;
            *x
        })
        .and_then(|x: &i32| *x * 2)
        .and_then(|x: &i32| *x - 5);

        let mut value = 10;
        assert_eq!(func.apply(&mut value), 17); // ((10 + 1) * 2) - 5
        assert_eq!(value, 11); // Input only modified by first function
    }

    #[test]
    fn test_identity() {
        let identity = BoxMutatingFunction::<i32, i32>::identity();
        let mut value = 42;
        assert_eq!(identity.apply(&mut value), 42);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_map() {
        let func = BoxMutatingFunction::new(|x: &mut i32| {
            *x *= 2;
            *x
        });
        let mapped = func.and_then(|result: &i32| result.to_string());

        let mut value = 5;
        assert_eq!(mapped.apply(&mut value), "10");
        assert_eq!(value, 10);
    }
}

// ============================================================================
// RcMutatingFunction Tests
// ============================================================================
