// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for MutatingFunction types (stateless Fn(&mut T) -> R)

use qubit_function::{
    ArcMutatingFunction,
    BoxMutatingFunction,
    MutatingFunction,
    MutatingFunctionOnce,
    RcMutatingFunction,
};

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
mod test_arc_mutating_function {
    use super::{
        ArcMutatingFunction,
        MutatingFunction,
    };
    use std::thread;

    #[test]
    fn test_new() {
        let func = ArcMutatingFunction::new(|x: &mut i32| {
            *x += 1;
            *x
        });
        let mut value = 5;
        assert_eq!(func.apply(&mut value), 6);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_clone() {
        let func = ArcMutatingFunction::new(|x: &mut i32| {
            *x *= 2;
            *x
        });
        let clone = func.clone();

        let mut value1 = 5;
        assert_eq!(func.apply(&mut value1), 10);

        let mut value2 = 3;
        assert_eq!(clone.apply(&mut value2), 6);
    }

    #[test]
    fn test_thread_safe() {
        let func = ArcMutatingFunction::new(|x: &mut i32| {
            *x *= 2;
            *x
        });
        let func_clone = func.clone();

        let handle = thread::spawn(move || {
            let mut value = 5;
            func_clone.apply(&mut value)
        });

        let result = handle.join().expect("thread should not panic");
        assert_eq!(result, 10);
    }

    #[test]
    fn test_identity() {
        let identity = ArcMutatingFunction::<i32, i32>::identity();
        let mut value = 42;
        assert_eq!(identity.apply(&mut value), 42);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_map() {
        let func = ArcMutatingFunction::new(|x: &mut i32| {
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
// Closure Tests
// ============================================================================
