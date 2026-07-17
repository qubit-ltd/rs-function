// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
#![cfg(feature = "full")]
use qubit_function::comparator::{
    ArcComparator,
    BoxComparator,
    Comparator,
    RcComparator,
};
use std::cmp::Ordering;

#[cfg(test)]
mod conversion_tests {
    use super::{
        Comparator,
        Ordering,
    };

    #[test]
    fn test_box_to_rc() {
        let box_cmp = qubit_function::comparator::BoxComparator::new(
            |a: &i32, b: &i32| a.cmp(b),
        );
        let rc_cmp = qubit_function::comparator::RcComparator::new(box_cmp);
        assert_eq!(rc_cmp.compare(&5, &3), Ordering::Greater);
    }

    #[test]
    fn test_arc_to_box() {
        let arc_cmp = qubit_function::comparator::ArcComparator::new(
            |a: &i32, b: &i32| a.cmp(b),
        );
        let box_cmp = qubit_function::comparator::BoxComparator::new(arc_cmp);
        assert_eq!(box_cmp.compare(&5, &3), Ordering::Greater);
    }

    #[test]
    fn test_arc_to_rc() {
        let arc_cmp = qubit_function::comparator::ArcComparator::new(
            |a: &i32, b: &i32| a.cmp(b),
        );
        let rc_cmp = qubit_function::comparator::RcComparator::new(arc_cmp);
        assert_eq!(rc_cmp.compare(&5, &3), Ordering::Greater);
    }

    #[test]
    fn test_rc_to_box() {
        let rc_cmp = qubit_function::comparator::RcComparator::new(
            |a: &i32, b: &i32| a.cmp(b),
        );
        let box_cmp = qubit_function::comparator::BoxComparator::new(rc_cmp);
        assert_eq!(box_cmp.compare(&5, &3), Ordering::Greater);
    }
}
