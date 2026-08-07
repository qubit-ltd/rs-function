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
mod conversion_tests {
    use super::ArcComparator;
    use super::BoxComparator;
    use super::Comparator;
    use super::Ordering;
    use super::RcComparator;

    #[test]
    fn test_box_to_rc() {
        let box_cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
        let rc_cmp = RcComparator::new(box_cmp);
        assert_eq!(rc_cmp.compare(&5, &3), Ordering::Greater);
    }

    #[test]
    fn test_arc_to_box() {
        let arc_cmp = ArcComparator::new(|a: &i32, b: &i32| a.cmp(b));
        let box_cmp = BoxComparator::new(arc_cmp);
        assert_eq!(box_cmp.compare(&5, &3), Ordering::Greater);
    }

    #[test]
    fn test_arc_to_rc() {
        let arc_cmp = ArcComparator::new(|a: &i32, b: &i32| a.cmp(b));
        let rc_cmp = RcComparator::new(arc_cmp);
        assert_eq!(rc_cmp.compare(&5, &3), Ordering::Greater);
    }

    #[test]
    fn test_rc_to_box() {
        let rc_cmp = RcComparator::new(|a: &i32, b: &i32| a.cmp(b));
        let box_cmp = BoxComparator::new(rc_cmp);
        assert_eq!(box_cmp.compare(&5, &3), Ordering::Greater);
    }
}
