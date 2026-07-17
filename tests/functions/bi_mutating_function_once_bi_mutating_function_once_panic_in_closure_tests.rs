// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Comprehensive tests for BiMutatingFunctionOnce trait and its implementations

use qubit_function::{
    BiMutatingFunctionOnce,
    BoxBiMutatingFunctionOnce,
};

// ============================================================================
// Helper Functions and Data Structures
// ============================================================================

fn append_strings_once(x: &mut String, y: &mut String) -> usize {
    x.push_str("_modified");
    y.push_str("_changed");
    x.len() + y.len()
}

#[derive(Clone, Debug, PartialEq)]
struct TestStruct {
    value: i32,
}

impl TestStruct {
    fn new(value: i32) -> Self {
        Self { value }
    }

    fn modify(&mut self, other: &mut Self) -> i32 {
        self.value += other.value;
        other.value *= 2;
        self.value + other.value
    }
}

fn modify_structs_once(a: &mut TestStruct, b: &mut TestStruct) -> i32 {
    a.modify(b)
}

// ============================================================================
// BiMutatingFunctionOnce Trait Tests - Core Functionality
// ============================================================================

#[test]
#[should_panic]
fn test_bi_mutating_function_once_panic_in_closure() {
    let panic_func = |x: &mut i32, y: &mut i32| {
        if *x < 0 {
            panic!("Negative value not allowed");
        }
        *x + *y
    };

    let mut a = -5;
    let mut b = 10;
    let _ = panic_func.apply(&mut a, &mut b);
}

#[test]
fn test_bi_mutating_function_once_with_option_modification() {
    let modify_option = |x: &mut Option<i32>, y: &mut Option<i32>| {
        if let (Some(val1), Some(val2)) = (*x, *y) {
            *x = Some(val1 + val2);
            *y = Some(val1 * val2);
            val1 + val2
        } else {
            0
        }
    };

    let mut a = Some(10);
    let mut b = Some(5);
    let result = modify_option.apply(&mut a, &mut b);
    assert_eq!(result, 15);
    assert_eq!(a, Some(15));
    assert_eq!(b, Some(50));

    let mut c = None;
    let mut d = Some(5);
    let result2 = modify_option.apply(&mut c, &mut d);
    assert_eq!(result2, 0);
    assert_eq!(c, None);
    assert_eq!(d, Some(5));
}

// ============================================================================
// One-Time Use Semantics Tests
// ============================================================================

#[test]
fn test_box_bi_mutating_function_once_consumption() {
    // Test that BoxBiMutatingFunctionOnce is consumed after use
    let create_func = || {
        BoxBiMutatingFunctionOnce::new(|x: &mut i32, y: &mut i32| {
            *x += 1;
            *y += 1;
            *x + *y
        })
    };

    let func = create_func();
    let mut a = 10;
    let mut b = 20;

    // Use the function once
    let result = func.apply(&mut a, &mut b);
    assert_eq!(result, 32); // (11) + (21) = 32
    assert_eq!(a, 11);
    assert_eq!(b, 21);

    // Create another function and use it
    let func2 = create_func();
    let mut c = 30;
    let mut d = 40;
    let result2 = func2.apply(&mut c, &mut d);
    assert_eq!(result2, 72); // (31) + (41) = 72
    assert_eq!(c, 31);
    assert_eq!(d, 41);
}

#[test]
fn test_bi_mutating_function_once_with_moving_data() {
    // Test with data that gets moved into the function
    let data = vec![1, 2, 3];
    let func = |x: &mut Vec<i32>, y: &mut Vec<i32>| {
        x.extend_from_slice(&data);
        y.push(42);
        x.len() + y.len()
    };

    let mut v1 = vec![10];
    let mut v2 = vec![20];
    let result = func.apply(&mut v1, &mut v2);

    assert_eq!(result, 6); // [10,1,2,3].len() + [20,42].len() = 4 + 2 = 6
    assert_eq!(v1, vec![10, 1, 2, 3]);
    assert_eq!(v2, vec![20, 42]);
}

// ============================================================================
// Complex Composition Scenarios
// ============================================================================

// ============================================================================
// Integration Tests
// ============================================================================

// ============================================================================
// Custom BiMutatingFunctionOnce Implementation Tests - Test Trait Default
// Methods
// ============================================================================
