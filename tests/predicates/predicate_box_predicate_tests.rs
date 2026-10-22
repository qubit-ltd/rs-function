// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for the predicate module.

#[cfg(test)]
mod box_predicate_tests {
    use qubit_function::predicates::BoxPredicate;
    use qubit_function::predicates::Predicate;

    #[test]
    fn test_new() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        assert!(pred.test(&5));
        assert!(!pred.test(&-3));
    }

    #[test]
    fn test_new_with_name() {
        let pred = BoxPredicate::new_with_name("is_positive", |x: &i32| *x > 0);
        assert_eq!(pred.name(), Some("is_positive"));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_set_name() {
        let mut pred = BoxPredicate::new(|x: &i32| *x > 0);
        assert_eq!(pred.name(), None);
        pred.set_name("is_positive");
        assert_eq!(pred.name(), Some("is_positive"));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_name_none() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        assert_eq!(pred.name(), None);
    }

    #[test]
    fn test_and_composition() {
        let pred1 = BoxPredicate::new(|x: &i32| *x > 0);
        let pred2 = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let combined = pred1.and(pred2);
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
        assert!(!combined.test(&-2));
    }

    #[test]
    fn test_and_with_names() {
        let pred1 = BoxPredicate::new_with_name("positive", |x: &i32| *x > 0);
        let pred2 = BoxPredicate::new_with_name("even", |x: &i32| x % 2 == 0);

        let combined = pred1.and(pred2);
        // Combined predicates do not inherit or generate names
        assert_eq!(combined.name(), None);
        assert!(combined.test(&4));
    }

    #[test]
    fn test_or_composition() {
        let pred1 = BoxPredicate::new(|x: &i32| *x < 0);
        let pred2 = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let combined = pred1.or(pred2);
        assert!(combined.test(&-5));
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_or_with_names() {
        let pred1 = BoxPredicate::new_with_name("negative", |x: &i32| *x < 0);
        let pred2 = BoxPredicate::new_with_name("even", |x: &i32| x % 2 == 0);

        let combined = pred1.or(pred2);
        // Combined predicates do not inherit or generate names
        assert_eq!(combined.name(), None);
        assert!(combined.test(&-5));
    }

    #[test]
    fn test_not_composition() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let negated = !pred;

        assert!(!negated.test(&5));
        assert!(negated.test(&-3));
        assert!(negated.test(&0));
    }

    #[test]
    fn test_not_with_name() {
        let pred = BoxPredicate::new_with_name("positive", |x: &i32| *x > 0);
        let negated = !pred;

        // Negation preserves the identity of its single source predicate.
        assert_eq!(negated.name(), Some("positive"));
        assert!(!negated.test(&5));
    }

    #[test]
    fn test_complex_composition() {
        let positive = BoxPredicate::new(|x: &i32| *x > 0);
        let even = BoxPredicate::new(|x: &i32| x % 2 == 0);
        let less_than_ten = BoxPredicate::new(|x: &i32| *x < 10);

        let combined = positive.and(even).and(less_than_ten);
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
        assert!(!combined.test(&12));
        assert!(!combined.test(&-2));
    }
}
