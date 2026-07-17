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
mod rc_predicate_tests {
    use super::{
        Predicate,
        RcPredicate,
    };

    #[test]
    fn test_new() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        assert!(pred.test(&5));
        assert!(!pred.test(&-3));
    }

    #[test]
    fn test_new_with_name() {
        let pred = RcPredicate::new_with_name("is_positive", |x: &i32| *x > 0);
        assert_eq!(pred.name(), Some("is_positive"));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_set_name() {
        let mut pred = RcPredicate::new(|x: &i32| *x > 0);
        assert_eq!(pred.name(), None);
        pred.set_name("is_positive");
        assert_eq!(pred.name(), Some("is_positive"));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_clone() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let pred_clone = pred.clone();

        assert!(pred.test(&5));
        assert!(pred_clone.test(&5));
        assert!(!pred_clone.test(&-3));
    }

    #[test]
    fn test_and_composition() {
        let pred1 = RcPredicate::new(|x: &i32| *x > 0);
        let pred2 = RcPredicate::new(|x: &i32| x % 2 == 0);

        let combined = pred1.and(pred2.clone());

        // Original predicates are still usable
        assert!(pred1.test(&5));
        assert!(pred2.test(&4));

        // Combined predicate works correctly
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_or_composition() {
        let pred1 = RcPredicate::new(|x: &i32| *x < 0);
        let pred2 = RcPredicate::new(|x: &i32| x % 2 == 0);

        let combined = pred1.or(pred2.clone());

        // Original predicates are still usable
        assert!(pred1.test(&-5));
        assert!(pred2.test(&4));

        // Combined predicate works correctly
        assert!(combined.test(&-5));
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_not_composition() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let negated = !&pred;

        // Original predicate is still usable
        assert!(pred.test(&5));

        // Negated predicate works correctly
        assert!(!negated.test(&5));
        assert!(negated.test(&-3));
    }

    #[test]
    fn test_complex_reuse() {
        let positive = RcPredicate::new(|x: &i32| *x > 0);
        let even = RcPredicate::new(|x: &i32| x % 2 == 0);

        let combined1 = positive.and(even.clone());
        let combined2 = positive.or(even.clone());

        // All predicates are still usable
        assert!(positive.test(&5));
        assert!(even.test(&4));
        assert!(combined1.test(&4));
        assert!(combined2.test(&5));
    }
}
