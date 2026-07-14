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

#[test]
fn test_new_accepts_custom_predicate() {
    let boxed = BoxPredicate::new(PositivePredicate);
    let shared = ArcPredicate::new(PositivePredicate);

    assert!(boxed.test(&1));
    assert!(!shared.test(&-1));
}

#[test]
fn test_predicate_not_operator() {
    let boxed = !BoxPredicate::new(|x: &i32| *x > 0);
    assert!(!boxed.test(&5));
    assert!(boxed.test(&-5));

    let rc = RcPredicate::new(|x: &i32| *x > 0);
    let negated_rc = !&rc;
    assert!(!negated_rc.test(&5));
    assert!(negated_rc.test(&-5));
    assert!(rc.test(&5));

    let arc = ArcPredicate::new(|x: &i32| *x > 0);
    let negated_arc = !&arc;
    assert!(!negated_arc.test(&5));
    assert!(negated_arc.test(&-5));
    assert!(arc.test(&5));
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

#[cfg(test)]
mod box_predicate_tests {
    use super::{
        BoxPredicate,
        Predicate,
    };

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

#[cfg(test)]
mod rc_predicate_tests {
    use super::{
        Predicate,
        RcPredicate,
    };

    #[test]
    fn test_new() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        assert!(pred.test(&5));
        assert!(!pred.test(&-3));
    }

    #[test]
    fn test_new_with_name() {
        let pred = RcPredicate::new_with_name("is_positive", |x: &i32| *x > 0);
        assert_eq!(pred.name(), Some("is_positive"));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_set_name() {
        let mut pred = RcPredicate::new(|x: &i32| *x > 0);
        assert_eq!(pred.name(), None);
        pred.set_name("is_positive");
        assert_eq!(pred.name(), Some("is_positive"));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_clone() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let pred_clone = pred.clone();

        assert!(pred.test(&5));
        assert!(pred_clone.test(&5));
        assert!(!pred_clone.test(&-3));
    }

    #[test]
    fn test_and_composition() {
        let pred1 = RcPredicate::new(|x: &i32| *x > 0);
        let pred2 = RcPredicate::new(|x: &i32| x % 2 == 0);

        let combined = pred1.and(pred2.clone());

        // Original predicates are still usable
        assert!(pred1.test(&5));
        assert!(pred2.test(&4));

        // Combined predicate works correctly
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_or_composition() {
        let pred1 = RcPredicate::new(|x: &i32| *x < 0);
        let pred2 = RcPredicate::new(|x: &i32| x % 2 == 0);

        let combined = pred1.or(pred2.clone());

        // Original predicates are still usable
        assert!(pred1.test(&-5));
        assert!(pred2.test(&4));

        // Combined predicate works correctly
        assert!(combined.test(&-5));
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_not_composition() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let negated = !&pred;

        // Original predicate is still usable
        assert!(pred.test(&5));

        // Negated predicate works correctly
        assert!(!negated.test(&5));
        assert!(negated.test(&-3));
    }

    #[test]
    fn test_complex_reuse() {
        let positive = RcPredicate::new(|x: &i32| *x > 0);
        let even = RcPredicate::new(|x: &i32| x % 2 == 0);

        let combined1 = positive.and(even.clone());
        let combined2 = positive.or(even.clone());

        // All predicates are still usable
        assert!(positive.test(&5));
        assert!(even.test(&4));
        assert!(combined1.test(&4));
        assert!(combined2.test(&5));
    }
}

#[cfg(test)]
mod arc_predicate_tests {
    use super::{
        ArcPredicate,
        Predicate,
    };

    #[test]
    fn test_new() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        assert!(pred.test(&5));
        assert!(!pred.test(&-3));
    }

    #[test]
    fn test_new_with_name() {
        let pred = ArcPredicate::new_with_name("is_positive", |x: &i32| *x > 0);
        assert_eq!(pred.name(), Some("is_positive"));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_set_name() {
        let mut pred = ArcPredicate::new(|x: &i32| *x > 0);
        assert_eq!(pred.name(), None);
        pred.set_name("is_positive");
        assert_eq!(pred.name(), Some("is_positive"));
        assert!(pred.test(&5));
    }

    #[test]
    fn test_clone() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let pred_clone = pred.clone();

        assert!(pred.test(&5));
        assert!(pred_clone.test(&5));
        assert!(!pred_clone.test(&-3));
    }

    #[test]
    fn test_send_sync() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);

        std::thread::spawn(move || {
            assert!(pred.test(&5));
            assert!(!pred.test(&-3));
        })
        .join()
        .expect("thread should not panic");
    }

    #[test]
    fn test_and_composition() {
        let pred1 = ArcPredicate::new(|x: &i32| *x > 0);
        let pred2 = ArcPredicate::new(|x: &i32| x % 2 == 0);

        let combined = pred1.and(pred2.clone());

        // Original predicates are still usable
        assert!(pred1.test(&5));
        assert!(pred2.test(&4));

        // Combined predicate works correctly
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_or_composition() {
        let pred1 = ArcPredicate::new(|x: &i32| *x < 0);
        let pred2 = ArcPredicate::new(|x: &i32| x % 2 == 0);

        let combined = pred1.or(pred2.clone());

        // Original predicates are still usable
        assert!(pred1.test(&-5));
        assert!(pred2.test(&4));

        // Combined predicate works correctly
        assert!(combined.test(&-5));
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_not_composition() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let negated = !&pred;

        // Original predicate is still usable
        assert!(pred.test(&5));

        // Negated predicate works correctly
        assert!(!negated.test(&5));
        assert!(negated.test(&-3));
    }

    #[test]
    fn test_thread_safe_composition() {
        let pred1 = ArcPredicate::new(|x: &i32| *x > 0);
        let pred2 = ArcPredicate::new(|x: &i32| x % 2 == 0);

        let combined = pred1.and(pred2.clone());
        let combined_clone = combined.clone();

        let handle = std::thread::spawn(move || {
            assert!(combined_clone.test(&4));
            assert!(!combined_clone.test(&3));
        });

        assert!(combined.test(&4));
        handle.join().expect("thread should not panic");
    }
}

#[cfg(test)]
mod interior_mutability_tests {
    use super::{
        Arc,
        ArcPredicate,
        BoxPredicate,
        Mutex,
        Predicate,
        RcPredicate,
        RefCell,
    };

    #[test]
    fn test_box_predicate_with_refcell_counter() {
        let count = RefCell::new(0);
        let pred = BoxPredicate::new(move |x: &i32| {
            *count.borrow_mut() += 1;
            *x > 0
        });

        assert!(pred.test(&5));
        assert!(pred.test(&10));
        assert!(!pred.test(&-3));
    }

    #[test]
    fn test_arc_predicate_with_mutex_counter() {
        let count = Arc::new(Mutex::new(0));
        let count_clone = Arc::clone(&count);

        let pred = ArcPredicate::new(move |x: &i32| {
            let mut c =
                count_clone.lock().expect("mutex should not be poisoned");
            *c += 1;
            *x > 0
        });

        assert!(pred.test(&5));
        assert!(pred.test(&10));
        assert!(!pred.test(&-3));

        assert_eq!(*count.lock().expect("mutex should not be poisoned"), 3);
    }

    #[test]
    fn test_rc_predicate_with_refcell_cache() {
        use std::collections::HashMap;

        let cache = RefCell::new(HashMap::new());
        let pred = RcPredicate::new(move |x: &i32| {
            let mut c = cache.borrow_mut();
            *c.entry(*x).or_insert_with(|| *x > 0 && x % 2 == 0)
        });

        // First call computes and caches
        assert!(pred.test(&4));
        // Second call uses cache
        assert!(pred.test(&4));
        assert!(!pred.test(&3));
    }

    #[test]
    fn test_arc_predicate_thread_safe_counter() {
        let count = Arc::new(Mutex::new(0));
        let pred = ArcPredicate::new({
            let count = Arc::clone(&count);
            move |x: &i32| {
                let mut c = count.lock().expect("mutex should not be poisoned");
                *c += 1;
                *x > 0
            }
        });

        let pred_clone = pred.clone();
        let count_clone = Arc::clone(&count);

        let handle = std::thread::spawn(move || {
            assert!(pred_clone.test(&5));
            assert!(pred_clone.test(&10));
        });

        assert!(pred.test(&3));
        handle.join().expect("thread should not panic");

        assert_eq!(
            *count_clone.lock().expect("mutex should not be poisoned"),
            3
        );
    }
}

#[cfg(test)]
mod different_types_tests {
    use super::{
        BoxPredicate,
        Predicate,
    };

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

#[cfg(test)]
mod generic_function_tests {
    use super::{
        ArcPredicate,
        BoxPredicate,
        Predicate,
        RcPredicate,
    };

    fn filter_by_predicate<T, P>(items: Vec<T>, pred: P) -> Vec<T>
    where
        P: Predicate<T>,
    {
        items.into_iter().filter(|item| pred.test(item)).collect()
    }

    #[test]
    fn test_with_box_predicate() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let result = filter_by_predicate(vec![-2, -1, 0, 1, 2], pred);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_with_rc_predicate() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let pred_clone = pred.clone();
        let result = filter_by_predicate(vec![-2, -1, 0, 1, 2], pred_clone);
        assert_eq!(result, vec![1, 2]);

        // pred is still usable
        assert!(pred.test(&5));
    }

    #[test]
    fn test_with_arc_predicate() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let pred_clone = pred.clone();
        let result = filter_by_predicate(vec![-2, -1, 0, 1, 2], pred_clone);
        assert_eq!(result, vec![1, 2]);

        // pred is still usable
        assert!(pred.test(&5));
    }

    #[test]
    fn test_with_closure() {
        let pred = |x: &i32| *x > 0;
        let result = filter_by_predicate(vec![-2, -1, 0, 1, 2], pred);
        assert_eq!(result, vec![1, 2]);
    }
}

#[cfg(test)]
mod logical_operations_tests {
    use super::{
        ArcPredicate,
        BoxPredicate,
        Predicate,
        RcPredicate,
    };

    // BoxPredicate NAND tests
    #[test]
    fn test_box_nand_basic() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let is_even = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let nand = is_positive.nand(is_even);

        // NAND: true unless both are true
        assert!(nand.test(&3)); // positive but odd: true && false = false, !false = true
        assert!(nand.test(&-2)); // negative but even: false && true = false, !false = true
        assert!(nand.test(&-1)); // negative and odd: false && false = false, !false = true
        assert!(!nand.test(&4)); // positive and even: true && true = true, !true = false
    }

    // BoxPredicate XOR tests
    #[test]
    fn test_box_xor_basic() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let is_even = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let xor = is_positive.xor(is_even);

        // XOR: true if exactly one is true
        assert!(xor.test(&3)); // positive but odd: true ^ false = true
        assert!(xor.test(&-2)); // negative but even: false ^ true = true
        assert!(!xor.test(&-1)); // negative and odd: false ^ false = false
        assert!(!xor.test(&4)); // positive and even: true ^ true = false
    }

    // BoxPredicate NOR tests
    #[test]
    fn test_box_nor_basic() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let is_even = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let nor = is_positive.nor(is_even);

        // NOR: true only when both are false
        assert!(nor.test(&-3)); // negative and odd: !(false || false) = true
        assert!(!nor.test(&3)); // positive but odd: !(true || false) = false
        assert!(!nor.test(&-2)); // negative but even: !(false || true) = false
        assert!(!nor.test(&4)); // positive and even: !(true || true) = false
    }

    // RcPredicate NAND tests
    #[test]
    fn test_rc_nand_basic() {
        let is_positive = RcPredicate::new(|x: &i32| *x > 0);
        let is_even = RcPredicate::new(|x: &i32| x % 2 == 0);

        let nand = is_positive.nand(is_even.clone());

        assert!(nand.test(&3)); // positive but odd
        assert!(nand.test(&-2)); // negative but even
        assert!(nand.test(&-1)); // negative and odd
        assert!(!nand.test(&4)); // positive and even

        // Original predicates still usable
        assert!(is_positive.test(&5));
        assert!(is_even.test(&6));
    }

    // RcPredicate XOR tests
    #[test]
    fn test_rc_xor_basic() {
        let is_positive = RcPredicate::new(|x: &i32| *x > 0);
        let is_even = RcPredicate::new(|x: &i32| x % 2 == 0);

        let xor = is_positive.xor(is_even.clone());

        assert!(xor.test(&3)); // positive but odd
        assert!(xor.test(&-2)); // negative but even
        assert!(!xor.test(&-1)); // negative and odd
        assert!(!xor.test(&4)); // positive and even

        // Original predicates still usable
        assert!(is_positive.test(&5));
        assert!(is_even.test(&6));
    }

    // RcPredicate NOR tests
    #[test]
    fn test_rc_nor_basic() {
        let is_positive = RcPredicate::new(|x: &i32| *x > 0);
        let is_even = RcPredicate::new(|x: &i32| x % 2 == 0);

        let nor = is_positive.nor(is_even.clone());

        // NOR: true only when both are false
        assert!(nor.test(&-3)); // negative and odd: !(false || false) = true
        assert!(!nor.test(&3)); // positive but odd: !(true || false) = false
        assert!(!nor.test(&-2)); // negative but even: !(false || true) = false
        assert!(!nor.test(&4)); // positive and even: !(true || true) = false

        // Original predicates still usable
        assert!(is_positive.test(&5));
        assert!(is_even.test(&6));
    }

    // ArcPredicate NAND tests
    #[test]
    fn test_arc_nand_basic() {
        let is_positive = ArcPredicate::new(|x: &i32| *x > 0);
        let is_even = ArcPredicate::new(|x: &i32| x % 2 == 0);

        let nand = is_positive.nand(is_even.clone());

        assert!(nand.test(&3)); // positive but odd
        assert!(nand.test(&-2)); // negative but even
        assert!(nand.test(&-1)); // negative and odd
        assert!(!nand.test(&4)); // positive and even

        // Original predicates still usable
        assert!(is_positive.test(&5));
        assert!(is_even.test(&6));
    }

    // ArcPredicate XOR tests
    #[test]
    fn test_arc_xor_basic() {
        let is_positive = ArcPredicate::new(|x: &i32| *x > 0);
        let is_even = ArcPredicate::new(|x: &i32| x % 2 == 0);

        let xor = is_positive.xor(is_even.clone());

        assert!(xor.test(&3)); // positive but odd
        assert!(xor.test(&-2)); // negative but even
        assert!(!xor.test(&-1)); // negative and odd
        assert!(!xor.test(&4)); // positive and even

        // Original predicates still usable
        assert!(is_positive.test(&5));
        assert!(is_even.test(&6));
    }

    // ArcPredicate NOR tests
    #[test]
    fn test_arc_nor_basic() {
        let is_positive = ArcPredicate::new(|x: &i32| *x > 0);
        let is_even = ArcPredicate::new(|x: &i32| x % 2 == 0);

        let nor = is_positive.nor(is_even.clone());

        // NOR: true only when both are false
        assert!(nor.test(&-3)); // negative and odd: !(false || false) = true
        assert!(!nor.test(&3)); // positive but odd: !(true || false) = false
        assert!(!nor.test(&-2)); // negative but even: !(false || true) = false
        assert!(!nor.test(&4)); // positive and even: !(true || true) = false

        // Original predicates still usable
        assert!(is_positive.test(&5));
        assert!(is_even.test(&6));
    }

    // Box wrapper NAND tests

    // Box wrapper XOR tests

    // Box wrapper NOR tests

    // Complex composition with NAND

    // Complex composition with XOR

    // NAND with string predicates
    #[test]
    fn test_nand_with_strings() {
        let is_long = BoxPredicate::new(|s: &String| s.len() > 5);
        let has_uppercase =
            BoxPredicate::new(|s: &String| s.chars().any(|c| c.is_uppercase()));

        let nand = is_long.nand(has_uppercase);

        assert!(nand.test(&"hello".to_string())); // short, no uppercase: !(false && false) = true
        assert!(nand.test(&"Hello".to_string())); // short, has uppercase: !(false && true) = true
        assert!(nand.test(&"goodbye".to_string())); // long, no uppercase: !(true && false) = true
        assert!(!nand.test(&"HelloWorld".to_string())); // long, has uppercase: !(true && true) = false
    }

    // XOR with string predicates
    #[test]
    fn test_xor_with_strings() {
        let is_long = BoxPredicate::new(|s: &String| s.len() > 5);
        let has_uppercase =
            BoxPredicate::new(|s: &String| s.chars().any(|c| c.is_uppercase()));

        let xor = is_long.xor(has_uppercase);

        assert!(!xor.test(&"hello".to_string())); // short, no uppercase: false ^ false = false
        assert!(xor.test(&"Hello".to_string())); // short, has uppercase: false ^ true = true
        assert!(xor.test(&"goodbye".to_string())); // long, no uppercase: true ^ false = true
        assert!(!xor.test(&"HelloWorld".to_string())); // long, has uppercase: true ^ true = false
    }

    // NOR with string predicates
    #[test]
    fn test_nor_with_strings() {
        let is_long = BoxPredicate::new(|s: &String| s.len() > 5);
        let has_uppercase =
            BoxPredicate::new(|s: &String| s.chars().any(|c| c.is_uppercase()));

        let nor = is_long.nor(has_uppercase);

        assert!(nor.test(&"hello".to_string())); // short, no uppercase: !(false || false) = true
        assert!(!nor.test(&"Hello".to_string())); // short, has uppercase: !(false || true) = false
        assert!(!nor.test(&"goodbye".to_string())); // long, no uppercase: !(true || false) = false
        assert!(!nor.test(&"HelloWorld".to_string())); // long, has uppercase: !(true || true) = false
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

#[cfg(test)]
mod always_predicates_tests {
    use super::{
        ArcPredicate,
        BoxPredicate,
        Predicate,
        RcPredicate,
    };

    #[test]
    fn test_box_always_true() {
        let pred = BoxPredicate::<i32>::always_true();
        assert!(pred.test(&5));
        assert!(pred.test(&-5));
        assert!(pred.test(&0));
    }

    #[test]
    fn test_box_always_false() {
        let pred = BoxPredicate::<i32>::always_false();
        assert!(!pred.test(&5));
        assert!(!pred.test(&-5));
        assert!(!pred.test(&0));
    }

    #[test]
    fn test_rc_always_true() {
        let pred = RcPredicate::<i32>::always_true();
        assert!(pred.test(&5));
        assert!(pred.test(&-5));
        assert!(pred.test(&0));
    }

    #[test]
    fn test_rc_always_false() {
        let pred = RcPredicate::<i32>::always_false();
        assert!(!pred.test(&5));
        assert!(!pred.test(&-5));
        assert!(!pred.test(&0));
    }

    #[test]
    fn test_arc_always_true() {
        let pred = ArcPredicate::<i32>::always_true();
        assert!(pred.test(&5));
        assert!(pred.test(&-5));
        assert!(pred.test(&0));
    }

    #[test]
    fn test_arc_always_false() {
        let pred = ArcPredicate::<i32>::always_false();
        assert!(!pred.test(&5));
        assert!(!pred.test(&-5));
        assert!(!pred.test(&0));
    }

    #[test]
    fn test_always_true_with_composition() {
        let always = BoxPredicate::<i32>::always_true();
        let is_positive = |x: &i32| *x > 0;

        let and_result = always.and(is_positive);
        assert!(and_result.test(&5));
        assert!(!and_result.test(&-5));
    }

    #[test]
    fn test_always_false_with_composition() {
        let never = BoxPredicate::<i32>::always_false();
        let is_positive = |x: &i32| *x > 0;

        let or_result = never.or(is_positive);
        assert!(or_result.test(&5));
        assert!(!or_result.test(&-5));
    }

    #[test]
    fn test_new_with_name() {
        let mut pred =
            BoxPredicate::new_with_name("positive", |x: &i32| *x > 0);
        assert_eq!(pred.name(), Some("positive"));
        assert!(pred.test(&5));

        pred.set_name("updated");
        assert_eq!(pred.name(), Some("updated"));
    }

    #[test]
    fn test_rc_new_with_name() {
        let mut pred = RcPredicate::new_with_name("positive", |x: &i32| *x > 0);
        assert_eq!(pred.name(), Some("positive"));
        assert!(pred.test(&5));

        pred.set_name("updated");
        assert_eq!(pred.name(), Some("updated"));
    }

    #[test]
    fn test_arc_new_with_name() {
        let mut pred =
            ArcPredicate::new_with_name("positive", |x: &i32| *x > 0);
        assert_eq!(pred.name(), Some("positive"));
        assert!(pred.test(&5));

        pred.set_name("updated");
        assert_eq!(pred.name(), Some("updated"));
    }
}

#[cfg(test)]
mod not_composition_tests {
    use super::{
        ArcPredicate,
        BoxPredicate,
        Predicate,
        RcPredicate,
    };

    #[test]
    fn test_box_not_and_composition() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let is_even = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !is_positive;
        let combined = not_positive.and(is_even);

        assert!(combined.test(&-2));
        assert!(!combined.test(&-3));
        assert!(!combined.test(&4));
    }

    #[test]
    fn test_box_not_or_composition() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let is_even = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !is_positive;
        let combined = not_positive.or(is_even);

        assert!(combined.test(&-3));
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_rc_not_and_composition() {
        let is_positive = RcPredicate::new(|x: &i32| *x > 0);
        let is_even = RcPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !&is_positive;
        let combined = not_positive.and(is_even);

        assert!(combined.test(&-2));
        assert!(!combined.test(&-3));
        assert!(!combined.test(&4));
    }

    #[test]
    fn test_rc_not_or_composition() {
        let is_positive = RcPredicate::new(|x: &i32| *x > 0);
        let is_even = RcPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !&is_positive;
        let combined = not_positive.or(is_even);

        assert!(combined.test(&-3));
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_arc_not_and_composition() {
        let is_positive = ArcPredicate::new(|x: &i32| *x > 0);
        let is_even = ArcPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !&is_positive;
        let combined = not_positive.and(is_even);

        assert!(combined.test(&-2));
        assert!(!combined.test(&-3));
        assert!(!combined.test(&4));
    }

    #[test]
    fn test_arc_not_or_composition() {
        let is_positive = ArcPredicate::new(|x: &i32| *x > 0);
        let is_even = ArcPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !&is_positive;
        let combined = not_positive.or(is_even);

        assert!(combined.test(&-3));
        assert!(combined.test(&4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_double_not() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let not_positive = !is_positive;
        let double_not = !not_positive;

        assert!(double_not.test(&5));
        assert!(!double_not.test(&-5));
    }

    #[test]
    fn test_not_with_nand() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let is_even = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !is_positive;
        let combined = not_positive.nand(is_even);

        assert!(combined.test(&-3));
        assert!(combined.test(&3));
        assert!(!combined.test(&-2));
    }

    #[test]
    fn test_not_with_xor() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let is_even = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !is_positive;
        let combined = not_positive.xor(is_even);

        assert!(combined.test(&-3));
        assert!(combined.test(&4));
        assert!(!combined.test(&-2));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_not_with_nor() {
        let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
        let is_even = BoxPredicate::new(|x: &i32| x % 2 == 0);

        let not_positive = !is_positive;
        let combined = not_positive.nor(is_even);

        assert!(combined.test(&3));
        assert!(!combined.test(&-3));
        assert!(!combined.test(&4));
        assert!(!combined.test(&-2));
    }
}

// ============================================================================
// Additional Type Conversion Tests
// ============================================================================

// ============================================================================
// Custom Predicate Type Tests (Default Implementation)
// ============================================================================

#[cfg(test)]
mod custom_predicate_tests {
    use super::Predicate;

    // Custom predicate struct that only implements the test method,
    // relying on default implementations for into_xxx methods.
    struct ThresholdPredicate {
        threshold: i32,
    }

    impl Predicate<i32> for ThresholdPredicate {
        fn test(&self, value: &i32) -> bool {
            *value > self.threshold
        }
        // All into_xxx methods use default implementations
    }

    #[test]
    fn test_custom_predicate_test() {
        let pred = ThresholdPredicate { threshold: 10 };

        assert!(pred.test(&15));
        assert!(pred.test(&100));
        assert!(!pred.test(&10));
        assert!(!pred.test(&5));
        assert!(!pred.test(&-5));
    }

    // Custom predicate with generic type parameter
    struct LengthPredicate {
        min_length: usize,
    }

    impl Predicate<String> for LengthPredicate {
        fn test(&self, value: &String) -> bool {
            value.len() >= self.min_length
        }
    }

    #[test]
    fn test_generic_custom_predicate() {
        let pred = LengthPredicate { min_length: 5 };

        assert!(pred.test(&"hello".to_string()));
        assert!(pred.test(&"world!".to_string()));
        assert!(!pred.test(&"hi".to_string()));
        assert!(!pred.test(&"".to_string()));
    }
}

// ============================================================================
// Display and Debug Tests
// ============================================================================

#[cfg(test)]
mod display_debug_tests {
    use super::{
        ArcPredicate,
        BoxPredicate,
        RcPredicate,
    };

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
