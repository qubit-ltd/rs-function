// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for the predicate module.

#[cfg(test)]
mod different_types_tests {
    use qubit_function::predicates::BoxPredicate;
    use qubit_function::predicates::Predicate;

    #[test]
    fn test_string_predicate() {
        let pred = BoxPredicate::new(|s: &String| s.len() > 3);
        assert!(pred.test(&"hello".to_string()));
        assert!(!pred.test(&"hi".to_string()));
    }

    #[test]
    fn test_str_predicate() {
        let pred = BoxPredicate::new(|s: &&str| s.len() > 3);
        assert!(pred.test(&"hello"));
        assert!(!pred.test(&"hi"));
    }

    #[test]
    fn test_vec_predicate() {
        let pred = BoxPredicate::new(|v: &Vec<i32>| v.len() > 2);
        assert!(pred.test(&vec![1, 2, 3]));
        assert!(!pred.test(&vec![1]));
    }

    #[test]
    fn test_option_predicate() {
        let pred = BoxPredicate::new(|opt: &Option<i32>| opt.is_some());
        assert!(pred.test(&Some(5)));
        assert!(!pred.test(&None));
    }

    #[test]
    fn test_tuple_predicate() {
        let pred = BoxPredicate::new(|(a, b): &(i32, i32)| a + b > 10);
        assert!(pred.test(&(6, 5)));
        assert!(!pred.test(&(2, 3)));
    }
}
