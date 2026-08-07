// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for the predicate module.

use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;

use qubit_function::predicates::ArcPredicate;
use qubit_function::predicates::BoxPredicate;
use qubit_function::predicates::Predicate;
use qubit_function::predicates::RcPredicate;

struct PositivePredicate;

impl Predicate<i32> for PositivePredicate {
    fn test(&self, value: &i32) -> bool {
        *value > 0
    }
}

#[cfg(test)]
mod arc_predicate_tests {
    use super::ArcPredicate;
    use super::Predicate;

    #[test]
    fn test_new() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        assert!(pred.test(&5));
        assert!(!pred.test(&-3));
    }

    #[test]
    fn test_new_with_name() {
        let pred = ArcPredicate::new_with_name("is_positive", |x: &i32| *x > 0);
        assert_eq!(pred.name(), Some("is_positive"));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_set_name() {
        let mut pred = ArcPredicate::new(|x: &i32| *x > 0);
        assert_eq!(pred.name(), None);
        pred.set_name("is_positive");
        assert_eq!(pred.name(), Some("is_positive"));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_clone() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let pred_clone = pred.clone();

        assert!(pred.test(&5));
        assert!(pred_clone.test(&5));
        assert!(!pred_clone.test(&-3));
    }

    #[test]
    fn test_send_sync() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);

        std::thread::spawn(move || {
            assert!(pred.test(&5));
            assert!(!pred.test(&-3));
        })
        .join()
        .expect("thread should not panic");
    }

    #[test]
    fn test_and_composition() {
        let pred1 = ArcPredicate::new(|x: &i32| *x > 0);
        let pred2 = ArcPredicate::new(|x: &i32| x % 2 == 0);

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
        let pred1 = ArcPredicate::new(|x: &i32| *x < 0);
        let pred2 = ArcPredicate::new(|x: &i32| x % 2 == 0);

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
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let negated = !&pred;

        // Original predicate is still usable
        assert!(pred.test(&5));

        // Negated predicate works correctly
        assert!(!negated.test(&5));
        assert!(negated.test(&-3));
    }

    #[test]
    fn test_thread_safe_composition() {
        let pred1 = ArcPredicate::new(|x: &i32| *x > 0);
        let pred2 = ArcPredicate::new(|x: &i32| x % 2 == 0);

        let combined = pred1.and(pred2.clone());
        let combined_clone = combined.clone();

        let handle = std::thread::spawn(move || {
            assert!(combined_clone.test(&4));
            assert!(!combined_clone.test(&3));
        });

        assert!(combined.test(&4));
        handle.join().expect("thread should not panic");
    }
}
