// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for the predicate module.

use qubit_function::predicates::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
    RcPredicate,
};
use std::cell::RefCell;
use std::sync::{
    Arc,
    Mutex,
};

struct PositivePredicate;

impl Predicate<i32> for PositivePredicate {
    fn test(&self, value: &i32) -> bool {
        *value > 0
    }
}

#[cfg(test)]
mod custom_predicate_tests {
    use super::Predicate;

    // Custom predicate struct that only implements the test method,
    // relying on default implementations for into_xxx methods.
    struct ThresholdPredicate {
        threshold: i32,
    }

    impl Predicate<i32> for ThresholdPredicate {
        fn test(&self, value: &i32) -> bool {
            *value > self.threshold
        }
        // All into_xxx methods use default implementations
    }

    #[test]
    fn test_custom_predicate_test() {
        let pred = ThresholdPredicate { threshold: 10 };

        assert!(pred.test(&15));
        assert!(pred.test(&100));
        assert!(!pred.test(&10));
        assert!(!pred.test(&5));
        assert!(!pred.test(&-5));
    }

    // Custom predicate with generic type parameter
    struct LengthPredicate {
        min_length: usize,
    }

    impl Predicate<String> for LengthPredicate {
        fn test(&self, value: &String) -> bool {
            value.len() >= self.min_length
        }
    }

    #[test]
    fn test_generic_custom_predicate() {
        let pred = LengthPredicate { min_length: 5 };

        assert!(pred.test(&"hello".to_string()));
        assert!(pred.test(&"world!".to_string()));
        assert!(!pred.test(&"hi".to_string()));
        assert!(!pred.test(&"".to_string()));
    }
}

// ============================================================================
// Display and Debug Tests
// ============================================================================
