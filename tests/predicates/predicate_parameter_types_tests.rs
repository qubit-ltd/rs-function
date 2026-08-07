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
mod parameter_types_tests {
    use super::ArcPredicate;
    use super::BoxPredicate;
    use super::Predicate;
    use super::RcPredicate;

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
    fn test_box_and_with_closure() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let combined = pred.and(|x: &i32| x % 2 == 0);

        assert!(combined.test(&4));
        assert!(!combined.test(&3));
        assert!(!combined.test(&-2));
    }

    #[test]
    fn test_box_and_with_function() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let combined = pred.and(is_even);

        assert!(combined.test(&4));
        assert!(!combined.test(&3));
        assert!(!combined.test(&-2));
    }

    #[test]
    fn test_box_and_with_box_predicate() {
        let pred1 = BoxPredicate::new(|x: &i32| *x > 0);
        let pred2 = BoxPredicate::new(|x: &i32| x % 2 == 0);
        let combined = pred1.and(pred2);

        assert!(combined.test(&4));
        assert!(!combined.test(&3));
        assert!(!combined.test(&-2));
    }

    #[test]
    fn test_box_and_with_rc_predicate() {
        let pred1 = BoxPredicate::new(|x: &i32| *x > 0);
        let pred2 = RcPredicate::new(|x: &i32| x % 2 == 0);
        let combined = pred1.and(pred2);

        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    // ============================================================================
    // BoxPredicate::or parameter type tests
    // ============================================================================

    #[test]
    fn test_box_or_with_closure() {
        let pred = BoxPredicate::new(|x: &i32| *x < 0);
        let combined = pred.or(|x: &i32| *x > 100);

        assert!(combined.test(&-5));
        assert!(combined.test(&150));
        assert!(!combined.test(&50));
    }

    #[test]
    fn test_box_or_with_function() {
        let pred = BoxPredicate::new(|x: &i32| *x < 0);
        let combined = pred.or(is_large);

        assert!(combined.test(&-5));
        assert!(combined.test(&150));
        assert!(!combined.test(&50));
    }

    #[test]
    fn test_box_or_with_box_predicate() {
        let pred1 = BoxPredicate::new(|x: &i32| *x < 0);
        let pred2 = BoxPredicate::new(|x: &i32| *x > 100);
        let combined = pred1.or(pred2);

        assert!(combined.test(&-5));
        assert!(combined.test(&150));
        assert!(!combined.test(&50));
    }

    // ============================================================================
    // BoxPredicate::nand parameter type tests
    // ============================================================================

    #[test]
    fn test_box_nand_with_closure() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let nand = pred.nand(|x: &i32| x % 2 == 0);

        assert!(nand.test(&3)); // !(true && false)
        assert!(!nand.test(&4)); // !(true && true)
    }

    #[test]
    fn test_box_nand_with_function() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let nand = pred.nand(is_even);

        assert!(nand.test(&3));
        assert!(!nand.test(&4));
    }

    #[test]
    fn test_box_nand_with_box_predicate() {
        let pred1 = BoxPredicate::new(|x: &i32| *x > 0);
        let pred2 = BoxPredicate::new(|x: &i32| x % 2 == 0);
        let nand = pred1.nand(pred2);

        assert!(nand.test(&3));
        assert!(!nand.test(&4));
    }

    // ============================================================================
    // BoxPredicate::xor parameter type tests
    // ============================================================================

    #[test]
    fn test_box_xor_with_closure() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let xor = pred.xor(|x: &i32| x % 2 == 0);

        assert!(xor.test(&3)); // true ^ false
        assert!(!xor.test(&4)); // true ^ true
        assert!(!xor.test(&-1)); // false ^ false
    }

    #[test]
    fn test_box_xor_with_function() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let xor = pred.xor(is_even);

        assert!(xor.test(&3));
        assert!(!xor.test(&4));
        assert!(!xor.test(&-1));
    }

    #[test]
    fn test_box_xor_with_box_predicate() {
        let pred1 = BoxPredicate::new(|x: &i32| *x > 0);
        let pred2 = BoxPredicate::new(|x: &i32| x % 2 == 0);
        let xor = pred1.xor(pred2);

        assert!(xor.test(&3));
        assert!(!xor.test(&4));
        assert!(!xor.test(&-1));
    }

    // ============================================================================
    // BoxPredicate::nor parameter type tests
    // ============================================================================

    #[test]
    fn test_box_nor_with_closure() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let nor = pred.nor(|x: &i32| x % 2 == 0);

        assert!(nor.test(&-3));
        assert!(!nor.test(&4));
        assert!(!nor.test(&3));
    }

    #[test]
    fn test_box_nor_with_function() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let nor = pred.nor(is_even);

        assert!(nor.test(&-3));
        assert!(!nor.test(&4));
    }

    #[test]
    fn test_box_nor_with_box_predicate() {
        let pred1 = BoxPredicate::new(|x: &i32| *x > 0);
        let pred2 = BoxPredicate::new(|x: &i32| x % 2 == 0);
        let nor = pred1.nor(pred2);

        assert!(nor.test(&-3));
        assert!(!nor.test(&4));
        assert!(!nor.test(&3));
    }

    // ============================================================================
    // RcPredicate::and parameter type tests
    // ============================================================================

    #[test]
    fn test_rc_and_with_closure() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let combined = pred.and(|x: &i32| x % 2 == 0);

        assert!(combined.test(&4));
        assert!(!combined.test(&3));

        // Original predicate is still usable
        assert!(pred.test(&5));
    }

    #[test]
    fn test_rc_and_with_function() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let combined = pred.and(is_even);

        assert!(combined.test(&4));
        assert!(!combined.test(&3));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_rc_and_with_rc_predicate() {
        let pred1 = RcPredicate::new(|x: &i32| *x > 0);
        let pred2 = RcPredicate::new(|x: &i32| x % 2 == 0);
        let combined = pred1.and(pred2.clone());

        assert!(combined.test(&4));
        assert!(!combined.test(&3));

        // Both original predicates are still usable
        assert!(pred1.test(&5));
        assert!(pred2.test(&6));
    }

    #[test]
    fn test_rc_and_with_box_predicate() {
        let pred1 = RcPredicate::new(|x: &i32| *x > 0);
        let pred2 = BoxPredicate::new(|x: &i32| x % 2 == 0);
        let combined = pred1.and(pred2);

        assert!(combined.test(&4));
        assert!(!combined.test(&3));
        assert!(pred1.test(&5));
    }

    // ============================================================================
    // RcPredicate::or parameter type tests
    // ============================================================================

    #[test]
    fn test_rc_or_with_closure() {
        let pred = RcPredicate::new(|x: &i32| *x < 0);
        let combined = pred.or(|x: &i32| *x > 100);

        assert!(combined.test(&-5));
        assert!(combined.test(&150));
        assert!(!combined.test(&50));
        assert!(pred.test(&-10));
    }

    #[test]
    fn test_rc_or_with_function() {
        let pred = RcPredicate::new(|x: &i32| *x < 0);
        let combined = pred.or(is_large);

        assert!(combined.test(&-5));
        assert!(combined.test(&150));
        assert!(pred.test(&-10));
    }

    #[test]
    fn test_rc_or_with_rc_predicate() {
        let pred1 = RcPredicate::new(|x: &i32| *x < 0);
        let pred2 = RcPredicate::new(|x: &i32| *x > 100);
        let combined = pred1.or(pred2.clone());

        assert!(combined.test(&-5));
        assert!(combined.test(&150));
        assert!(pred1.test(&-10));
        assert!(pred2.test(&150));
    }

    // ============================================================================
    // RcPredicate::nand parameter type tests
    // ============================================================================

    #[test]
    fn test_rc_nand_with_closure() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let nand = pred.nand(|x: &i32| x % 2 == 0);

        assert!(nand.test(&3));
        assert!(!nand.test(&4));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_rc_nand_with_function() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let nand = pred.nand(is_even);

        assert!(nand.test(&3));
        assert!(!nand.test(&4));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_rc_nand_with_rc_predicate() {
        let pred1 = RcPredicate::new(|x: &i32| *x > 0);
        let pred2 = RcPredicate::new(|x: &i32| x % 2 == 0);
        let nand = pred1.nand(pred2.clone());

        assert!(nand.test(&3));
        assert!(!nand.test(&4));
        assert!(pred1.test(&5));
        assert!(pred2.test(&6));
    }

    // ============================================================================
    // RcPredicate::xor parameter type tests
    // ============================================================================

    #[test]
    fn test_rc_xor_with_closure() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let xor = pred.xor(|x: &i32| x % 2 == 0);

        assert!(xor.test(&3));
        assert!(!xor.test(&4));
        assert!(!xor.test(&-1));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_rc_xor_with_function() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let xor = pred.xor(is_even);

        assert!(xor.test(&3));
        assert!(!xor.test(&4));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_rc_xor_with_rc_predicate() {
        let pred1 = RcPredicate::new(|x: &i32| *x > 0);
        let pred2 = RcPredicate::new(|x: &i32| x % 2 == 0);
        let xor = pred1.xor(pred2.clone());

        assert!(xor.test(&3));
        assert!(!xor.test(&4));
        assert!(pred1.test(&5));
        assert!(pred2.test(&6));
    }

    // ============================================================================
    // RcPredicate::nor parameter type tests
    // ============================================================================

    #[test]
    fn test_rc_nor_with_closure() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let nor = pred.nor(|x: &i32| x % 2 == 0);

        assert!(nor.test(&-3));
        assert!(!nor.test(&4));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_rc_nor_with_function() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let nor = pred.nor(is_even);

        assert!(nor.test(&-3));
        assert!(!nor.test(&4));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_rc_nor_with_rc_predicate() {
        let pred1 = RcPredicate::new(|x: &i32| *x > 0);
        let pred2 = RcPredicate::new(|x: &i32| x % 2 == 0);
        let nor = pred1.nor(pred2.clone());

        assert!(nor.test(&-3));
        assert!(!nor.test(&4));
        assert!(pred1.test(&5));
        assert!(pred2.test(&6));
    }

    // ============================================================================
    // ArcPredicate::and parameter type tests
    // ============================================================================

    #[test]
    fn test_arc_and_with_closure() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let combined = pred.and(|x: &i32| x % 2 == 0);

        assert!(combined.test(&4));
        assert!(!combined.test(&3));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_arc_and_with_function() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let combined = pred.and(is_even);

        assert!(combined.test(&4));
        assert!(!combined.test(&3));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_arc_and_with_arc_predicate() {
        let pred1 = ArcPredicate::new(|x: &i32| *x > 0);
        let pred2 = ArcPredicate::new(|x: &i32| x % 2 == 0);
        let combined = pred1.and(pred2.clone());

        assert!(combined.test(&4));
        assert!(!combined.test(&3));
        assert!(pred1.test(&5));
        assert!(pred2.test(&6));
    }

    // ============================================================================
    // ArcPredicate::or parameter type tests
    // ============================================================================

    #[test]
    fn test_arc_or_with_closure() {
        let pred = ArcPredicate::new(|x: &i32| *x < 0);
        let combined = pred.or(|x: &i32| *x > 100);

        assert!(combined.test(&-5));
        assert!(combined.test(&150));
        assert!(!combined.test(&50));
        assert!(pred.test(&-10));
    }
}
