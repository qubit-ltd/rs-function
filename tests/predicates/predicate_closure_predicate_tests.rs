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
mod closure_predicate_tests {
    use super::Predicate;

    #[test]
    fn test_closure_implements_predicate() {
        let is_positive = |x: &i32| *x > 0;
        assert!(is_positive.test(&5));
        assert!(!is_positive.test(&-3));
        assert!(!is_positive.test(&0));
    }
}
