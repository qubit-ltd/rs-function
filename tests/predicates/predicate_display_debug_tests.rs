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
mod display_debug_tests {
    use super::ArcPredicate;
    use super::BoxPredicate;
    use super::RcPredicate;

    #[test]
    fn test_box_display_unnamed() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let display_str = format!("{}", pred);
        assert_eq!(display_str, "BoxPredicate(unnamed)");
    }

    #[test]
    fn test_box_display_named() {
        let pred = BoxPredicate::new_with_name("is_positive", |x: &i32| *x > 0);
        let display_str = format!("{}", pred);
        assert_eq!(display_str, "BoxPredicate(is_positive)");
    }

    #[test]
    fn test_box_debug() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let debug_str = format!("{:?}", pred);
        assert!(debug_str.contains("BoxPredicate"));
        assert!(debug_str.contains("name"));
    }

    #[test]
    fn test_arc_display_unnamed() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let display_str = format!("{}", pred);
        assert_eq!(display_str, "ArcPredicate(unnamed)");
    }

    #[test]
    fn test_arc_display_named() {
        let pred = ArcPredicate::new_with_name("is_positive", |x: &i32| *x > 0);
        let display_str = format!("{}", pred);
        assert_eq!(display_str, "ArcPredicate(is_positive)");
    }

    #[test]
    fn test_arc_debug() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let debug_str = format!("{:?}", pred);
        assert!(debug_str.contains("ArcPredicate"));
        assert!(debug_str.contains("name"));
    }

    #[test]
    fn test_rc_display_unnamed() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let display_str = format!("{}", pred);
        assert_eq!(display_str, "RcPredicate(unnamed)");
    }

    #[test]
    fn test_rc_display_named() {
        let pred = RcPredicate::new_with_name("is_positive", |x: &i32| *x > 0);
        let display_str = format!("{}", pred);
        assert_eq!(display_str, "RcPredicate(is_positive)");
    }

    #[test]
    fn test_rc_debug() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let debug_str = format!("{:?}", pred);
        assert!(debug_str.contains("RcPredicate"));
        assert!(debug_str.contains("name"));
    }
}
// ============================================================================
// Name Preservation Tests for into_xxx and to_xxx Methods
// ============================================================================
