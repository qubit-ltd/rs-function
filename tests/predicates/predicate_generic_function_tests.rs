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
mod generic_function_tests {
    use super::{
        ArcPredicate,
        BoxPredicate,
        Predicate,
        RcPredicate,
    };

    fn filter_by_predicate<T, P>(items: Vec<T>, pred: P) -> Vec<T>
    where
        P: Predicate<T>,
    {
        items.into_iter().filter(|item| pred.test(item)).collect()
    }

    #[test]
    fn test_with_box_predicate() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let result = filter_by_predicate(vec![-2, -1, 0, 1, 2], pred);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_with_rc_predicate() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let pred_clone = pred.clone();
        let result = filter_by_predicate(vec![-2, -1, 0, 1, 2], pred_clone);
        assert_eq!(result, vec![1, 2]);

        // pred is still usable
        assert!(pred.test(&5));
    }

    #[test]
    fn test_with_arc_predicate() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let pred_clone = pred.clone();
        let result = filter_by_predicate(vec![-2, -1, 0, 1, 2], pred_clone);
        assert_eq!(result, vec![1, 2]);

        // pred is still usable
        assert!(pred.test(&5));
    }

    #[test]
    fn test_with_closure() {
        let pred = |x: &i32| *x > 0;
        let result = filter_by_predicate(vec![-2, -1, 0, 1, 2], pred);
        assert_eq!(result, vec![1, 2]);
    }
}
