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
mod not_composition_tests {
    use super::{
        ArcPredicate,
        BoxPredicate,
        Predicate,
        RcPredicate,
    };

    #[test]
    fn test_box_not_and_composition() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let is_even = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !is_positive;
        let combined = not_positive.and(is_even);

        assert!(combined.test(&-2));
        assert!(!combined.test(&-3));
        assert!(!combined.test(&4));
    }

    #[test]
    fn test_box_not_or_composition() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let is_even = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !is_positive;
        let combined = not_positive.or(is_even);

        assert!(combined.test(&-3));
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_rc_not_and_composition() {
        let is_positive = RcPredicate::new(|x: &i32| *x > 0);
        let is_even = RcPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !&is_positive;
        let combined = not_positive.and(is_even);

        assert!(combined.test(&-2));
        assert!(!combined.test(&-3));
        assert!(!combined.test(&4));
    }

    #[test]
    fn test_rc_not_or_composition() {
        let is_positive = RcPredicate::new(|x: &i32| *x > 0);
        let is_even = RcPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !&is_positive;
        let combined = not_positive.or(is_even);

        assert!(combined.test(&-3));
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_arc_not_and_composition() {
        let is_positive = ArcPredicate::new(|x: &i32| *x > 0);
        let is_even = ArcPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !&is_positive;
        let combined = not_positive.and(is_even);

        assert!(combined.test(&-2));
        assert!(!combined.test(&-3));
        assert!(!combined.test(&4));
    }

    #[test]
    fn test_arc_not_or_composition() {
        let is_positive = ArcPredicate::new(|x: &i32| *x > 0);
        let is_even = ArcPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !&is_positive;
        let combined = not_positive.or(is_even);

        assert!(combined.test(&-3));
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_double_not() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let not_positive = !is_positive;
        let double_not = !not_positive;

        assert!(double_not.test(&5));
        assert!(!double_not.test(&-5));
    }

    #[test]
    fn test_not_with_nand() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let is_even = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !is_positive;
        let combined = not_positive.nand(is_even);

        assert!(combined.test(&-3));
        assert!(combined.test(&3));
        assert!(!combined.test(&-2));
    }

    #[test]
    fn test_not_with_xor() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let is_even = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !is_positive;
        let combined = not_positive.xor(is_even);

        assert!(combined.test(&-3));
        assert!(combined.test(&4));
        assert!(!combined.test(&-2));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_not_with_nor() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let is_even = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !is_positive;
        let combined = not_positive.nor(is_even);

        assert!(combined.test(&3));
        assert!(!combined.test(&-3));
        assert!(!combined.test(&4));
        assert!(!combined.test(&-2));
    }
}

// ============================================================================
// Additional Type Conversion Tests
// ============================================================================

// ============================================================================
// Custom Predicate Type Tests (Default Implementation)
// ============================================================================
