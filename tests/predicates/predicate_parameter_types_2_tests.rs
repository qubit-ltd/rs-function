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
mod parameter_types_tests {
    use super::{
        ArcPredicate,
        BoxPredicate,
        Predicate,
        RcPredicate,
    };

    // Helper functions
    fn is_even(x: &i32) -> bool {
        x % 2 == 0
    }

    fn is_large(x: &i32) -> bool {
        *x > 100
    }

    // ============================================================================
    // BoxPredicate::and parameter type tests
    // ============================================================================

    #[test]
    fn test_arc_or_with_function() {
        let pred = ArcPredicate::new(|x: &i32| *x < 0);
        let combined = pred.or(is_large);

        assert!(combined.test(&-5));
        assert!(combined.test(&150));
        assert!(pred.test(&-10));
    }

    #[test]
    fn test_arc_or_with_arc_predicate() {
        let pred1 = ArcPredicate::new(|x: &i32| *x < 0);
        let pred2 = ArcPredicate::new(|x: &i32| *x > 100);
        let combined = pred1.or(pred2.clone());

        assert!(combined.test(&-5));
        assert!(combined.test(&150));
        assert!(pred1.test(&-10));
        assert!(pred2.test(&150));
    }

    // ============================================================================
    // ArcPredicate::nand parameter type tests
    // ============================================================================

    #[test]
    fn test_arc_nand_with_closure() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let nand = pred.nand(|x: &i32| x % 2 == 0);

        assert!(nand.test(&3));
        assert!(!nand.test(&4));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_arc_nand_with_function() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let nand = pred.nand(is_even);

        assert!(nand.test(&3));
        assert!(!nand.test(&4));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_arc_nand_with_arc_predicate() {
        let pred1 = ArcPredicate::new(|x: &i32| *x > 0);
        let pred2 = ArcPredicate::new(|x: &i32| x % 2 == 0);
        let nand = pred1.nand(pred2.clone());

        assert!(nand.test(&3));
        assert!(!nand.test(&4));
        assert!(pred1.test(&5));
        assert!(pred2.test(&6));
    }

    // ============================================================================
    // ArcPredicate::xor parameter type tests
    // ============================================================================

    #[test]
    fn test_arc_xor_with_closure() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let xor = pred.xor(|x: &i32| x % 2 == 0);

        assert!(xor.test(&3));
        assert!(!xor.test(&4));
        assert!(!xor.test(&-1));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_arc_xor_with_function() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let xor = pred.xor(is_even);

        assert!(xor.test(&3));
        assert!(!xor.test(&4));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_arc_xor_with_arc_predicate() {
        let pred1 = ArcPredicate::new(|x: &i32| *x > 0);
        let pred2 = ArcPredicate::new(|x: &i32| x % 2 == 0);
        let xor = pred1.xor(pred2.clone());

        assert!(xor.test(&3));
        assert!(!xor.test(&4));
        assert!(pred1.test(&5));
        assert!(pred2.test(&6));
    }

    // ============================================================================
    // ArcPredicate::nor parameter type tests
    // ============================================================================

    #[test]
    fn test_arc_nor_with_closure() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let nor = pred.nor(|x: &i32| x % 2 == 0);

        assert!(nor.test(&-3));
        assert!(!nor.test(&4));
        assert!(!nor.test(&3));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_arc_nor_with_function() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let nor = pred.nor(is_even);

        assert!(nor.test(&-3));
        assert!(!nor.test(&4));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_arc_nor_with_arc_predicate() {
        let pred1 = ArcPredicate::new(|x: &i32| *x > 0);
        let pred2 = ArcPredicate::new(|x: &i32| x % 2 == 0);
        let nor = pred1.nor(pred2.clone());

        assert!(nor.test(&-3));
        assert!(!nor.test(&4));
        assert!(pred1.test(&5));
        assert!(pred2.test(&6));
    }

    // ============================================================================
    // Box wrapper parameter type tests
    // ============================================================================
}
