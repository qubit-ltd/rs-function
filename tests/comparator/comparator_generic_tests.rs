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
mod generic_tests {
    use super::Comparator;

    fn sort_with_comparator<C: Comparator<i32>>(
        cmp: &C,
        mut vec: Vec<i32>,
    ) -> Vec<i32> {
        vec.sort_by(|a, b| cmp.compare(a, b));
        vec
    }

    #[test]
    fn test_with_box_comparator() {
        let cmp = qubit_function::comparator::BoxComparator::new(
            |a: &i32, b: &i32| a.cmp(b),
        );
        let sorted = sort_with_comparator(&cmp, vec![3, 1, 4, 1, 5]);
        assert_eq!(sorted, vec![1, 1, 3, 4, 5]);
    }

    #[test]
    fn test_with_arc_comparator() {
        let cmp = qubit_function::comparator::ArcComparator::new(
            |a: &i32, b: &i32| a.cmp(b),
        );
        let sorted = sort_with_comparator(&cmp, vec![3, 1, 4, 1, 5]);
        assert_eq!(sorted, vec![1, 1, 3, 4, 5]);
    }

    #[test]
    fn test_with_rc_comparator() {
        let cmp = qubit_function::comparator::RcComparator::new(
            |a: &i32, b: &i32| a.cmp(b),
        );
        let sorted = sort_with_comparator(&cmp, vec![3, 1, 4, 1, 5]);
        assert_eq!(sorted, vec![1, 1, 3, 4, 5]);
    }

    #[test]
    fn test_with_closure() {
        let cmp = |a: &i32, b: &i32| a.cmp(b);
        let sorted = sort_with_comparator(&cmp, vec![3, 1, 4, 1, 5]);
        assert_eq!(sorted, vec![1, 1, 3, 4, 5]);
    }
}
