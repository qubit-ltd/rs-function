// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
#![cfg(feature = "full")]
use std::cmp::Ordering;

use qubit_function::comparator::ArcComparator;
use qubit_function::comparator::BoxComparator;
use qubit_function::comparator::Comparator;
use qubit_function::comparator::RcComparator;

#[cfg(test)]
mod closure_tests {
    use super::ArcComparator;
    use super::BoxComparator;
    use super::Comparator;
    use super::Ordering;
    use super::RcComparator;

    #[test]
    fn test_closure_as_comparator() {
        let cmp = |a: &i32, b: &i32| a.cmp(b);
        assert_eq!(cmp.compare(&5, &3), Ordering::Greater);
        assert_eq!(cmp.compare(&3, &5), Ordering::Less);
        assert_eq!(cmp.compare(&5, &5), Ordering::Equal);
    }

    #[test]
    fn test_closure_into_box() {
        let cmp = |a: &i32, b: &i32| a.cmp(b);
        let boxed = BoxComparator::new(cmp);
        assert_eq!(boxed.compare(&5, &3), Ordering::Greater);
    }

    #[test]
    fn test_closure_into_rc() {
        let cmp = |a: &i32, b: &i32| a.cmp(b);
        let rc = RcComparator::new(cmp);
        assert_eq!(rc.compare(&5, &3), Ordering::Greater);
    }

    #[test]
    fn test_closure_into_arc() {
        let cmp = |a: &i32, b: &i32| a.cmp(b);
        let arc = ArcComparator::new(cmp);
        assert_eq!(arc.compare(&5, &3), Ordering::Greater);
    }

    #[test]
    fn test_closure_into_fn() {
        let cmp = |a: &i32, b: &i32| a.cmp(b);
        let func = move |first: &i32, second: &i32| cmp.compare(first, second);
        assert_eq!(func(&5, &3), Ordering::Greater);
    }
}
