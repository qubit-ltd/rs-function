/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

// qubit-style: allow explicit-imports
use qubit_function::predicates::{
    ArcStatefulBiPredicate,
    BoxStatefulBiPredicate,
    FnStatefulBiPredicateOps,
    RcStatefulBiPredicate,
    StatefulBiPredicate,
};
use std::cell::{
    Cell,
    RefCell,
};
use std::rc::Rc;
use std::sync::{
    Arc,
    Mutex,
};
use std::thread;

fn box_stateful_bi_predicate_returning(value: bool) -> BoxStatefulBiPredicate<i32, i32> {
    BoxStatefulBiPredicate::new(move |_: &i32, _: &i32| value)
}

#[test]
fn test_stateful_bi_predicate_default_conversions_allow_relaxed_generic_types() {
    #[derive(Debug)]
    struct Borrowed<'a> {
        value: &'a str,
    }

    #[derive(Debug)]
    struct BorrowedStatefulBiPredicate {
        count: Cell<usize>,
    }

    impl Clone for BorrowedStatefulBiPredicate {
        fn clone(&self) -> Self {
            Self {
                count: Cell::new(self.count.get()),
            }
        }
    }

    impl<'a> StatefulBiPredicate<Borrowed<'a>, Borrowed<'a>> for BorrowedStatefulBiPredicate {
        fn test(&mut self, first: &Borrowed<'a>, second: &Borrowed<'a>) -> bool {
            self.count.set(self.count.get() + 1);
            first.value == "left" && second.value == "right"
        }
    }

    let left = String::from("left");
    let right = String::from("right");
    let first = Borrowed { value: left.as_str() };
    let second = Borrowed { value: right.as_str() };
    let predicate = BorrowedStatefulBiPredicate { count: Cell::new(0) };

    assert!(predicate.clone().into_box().test(&first, &second));
    assert!(predicate.clone().into_rc().test(&first, &second));
    assert!(predicate.clone().into_arc().test(&first, &second));

    let mut into_fn = predicate.clone().into_fn();
    assert!(into_fn(&first, &second));

    assert!(predicate.to_box().test(&first, &second));
    assert!(predicate.to_rc().test(&first, &second));
    assert!(predicate.to_arc().test(&first, &second));

    let mut to_fn = predicate.to_fn();
    assert!(to_fn(&first, &second));
}

#[test]
fn test_box_stateful_bi_predicate_tracks_state_and_short_circuits() {
    let mut calls = 0;
    let mut predicate =
        BoxStatefulBiPredicate::new_with_name("every_second_positive_sum", move |first: &i32, second: &i32| {
            calls += 1;
            calls % 2 == 0 && first + second > 0
        });

    assert_eq!(predicate.name(), Some("every_second_positive_sum"));
    assert!(!predicate.test(&5, &3));
    assert!(predicate.test(&5, &3));

    let rhs_calls = Rc::new(Cell::new(0));
    let rhs_seen = rhs_calls.clone();
    let mut and_predicate = BoxStatefulBiPredicate::new(|_: &i32, _: &i32| false).and(BoxStatefulBiPredicate::new(
        move |_: &i32, _: &i32| {
            rhs_seen.set(rhs_seen.get() + 1);
            true
        },
    ));

    assert!(!and_predicate.test(&1, &2));
    assert_eq!(rhs_calls.get(), 0);

    let rhs_calls = Rc::new(Cell::new(0));
    let rhs_seen = rhs_calls.clone();
    let mut or_predicate = BoxStatefulBiPredicate::new(|_: &i32, _: &i32| true).or(BoxStatefulBiPredicate::new(
        move |_: &i32, _: &i32| {
            rhs_seen.set(rhs_seen.get() + 1);
            false
        },
    ));

    assert!(or_predicate.test(&1, &2));
    assert_eq!(rhs_calls.get(), 0);
}

#[test]
fn test_rc_stateful_bi_predicate_clones_share_state_and_can_be_negated() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let observed = log.clone();
    let mut predicate = RcStatefulBiPredicate::new(move |first: &i32, second: &i32| {
        observed.borrow_mut().push(first + second);
        first > second
    });
    let mut clone = predicate.clone();

    assert!(predicate.test(&5, &3));
    assert!(!clone.test(&2, &7));
    assert_eq!(*log.borrow(), vec![8, 9]);

    let mut negated = !&predicate;
    assert!(!negated.test(&9, &1));
}

#[test]
fn test_arc_stateful_bi_predicate_can_be_shared_across_threads() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let observed = log.clone();
    let predicate = ArcStatefulBiPredicate::new(move |first: &i32, second: &i32| {
        let sum = first + second;
        observed.lock().expect("mutex should not be poisoned").push(sum);
        sum > 0
    });

    let mut thread_predicate = predicate.clone();
    let handle = thread::spawn(move || thread_predicate.test(&5, &3));
    assert!(handle.join().expect("thread should not panic"));

    let mut local_predicate = predicate.clone();
    assert!(!local_predicate.test(&-10, &2));
    assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8, -8]);
}

#[test]
fn test_fn_stateful_bi_predicate_ops_compose_mutable_closures() {
    let left_calls = Rc::new(Cell::new(0));
    let right_calls = Rc::new(Cell::new(0));
    let left_seen = left_calls.clone();
    let right_seen = right_calls.clone();

    let left = move |first: &i32, second: &i32| {
        left_seen.set(left_seen.get() + 1);
        first + second > 0
    };
    let right = move |first: &i32, second: &i32| {
        right_seen.set(right_seen.get() + 1);
        first > second
    };

    let mut combined = left.and(right);
    assert!(combined.test(&5, &3));
    assert!(!combined.test(&-5, &3));
    assert_eq!(left_calls.get(), 2);
    assert_eq!(right_calls.get(), 1);

    let calls = Rc::new(Cell::new(0));
    let seen = calls.clone();
    let mut negated = (move |first: &i32, second: &i32| {
        seen.set(seen.get() + 1);
        first == second
    })
    .not();
    assert!(negated.test(&1, &2));
    assert!(!negated.test(&2, &2));
    assert_eq!(calls.get(), 2);
}

#[test]
fn test_box_stateful_bi_predicate_common_methods_and_conversions() {
    let mut unnamed = BoxStatefulBiPredicate::new(|first: &i32, second: &i32| first < second);
    assert!(unnamed.test(&1, &2));
    assert_eq!(unnamed.name(), None);
    assert_eq!(format!("{unnamed}"), "BoxStatefulBiPredicate(unnamed)");
    assert!(format!("{unnamed:?}").contains("BoxStatefulBiPredicate"));

    let mut named = BoxStatefulBiPredicate::new_with_name("less_than", |first: &i32, second: &i32| first < second);
    assert_eq!(named.name(), Some("less_than"));
    named.set_name("ordered");
    assert_eq!(named.name(), Some("ordered"));
    assert_eq!(format!("{named}"), "BoxStatefulBiPredicate(ordered)");
    assert!(named.test(&1, &2));

    let mut always_true = BoxStatefulBiPredicate::<i32, i32>::always_true();
    assert!(always_true.test(&10, &-10));
    assert_eq!(always_true.name(), Some("always_true"));

    let mut always_false = BoxStatefulBiPredicate::<i32, i32>::always_false();
    assert!(!always_false.test(&10, &-10));
    assert_eq!(always_false.name(), Some("always_false"));

    let mut boxed = BoxStatefulBiPredicate::new(|first: &i32, second: &i32| first == second).into_box();
    assert!(boxed.test(&3, &3));

    let mut rc = BoxStatefulBiPredicate::new(|first: &i32, second: &i32| first > second).into_rc();
    assert!(rc.test(&3, &1));

    let mut function = BoxStatefulBiPredicate::new(|first: &i32, second: &i32| first + second == 5).into_fn();
    assert!(function(&2, &3));
}

#[test]
fn test_box_stateful_bi_predicate_logical_methods() {
    let mut nand =
        BoxStatefulBiPredicate::new(|_: &i32, _: &i32| true).nand(BoxStatefulBiPredicate::new(|_: &i32, _: &i32| true));
    assert!(!nand.test(&1, &2));

    let mut xor =
        BoxStatefulBiPredicate::new(|_: &i32, _: &i32| true).xor(BoxStatefulBiPredicate::new(|_: &i32, _: &i32| false));
    assert!(xor.test(&1, &2));

    let mut nor = BoxStatefulBiPredicate::new(|_: &i32, _: &i32| false)
        .nor(BoxStatefulBiPredicate::new(|_: &i32, _: &i32| false));
    assert!(nor.test(&1, &2));

    let mut negated = !BoxStatefulBiPredicate::new(|_: &i32, _: &i32| true);
    assert!(!negated.test(&1, &2));
}

#[test]
fn test_box_stateful_bi_predicate_logical_truth_tables() {
    for (left, right, expected) in [
        (true, true, true),
        (true, false, false),
        (false, true, false),
        (false, false, false),
    ] {
        let mut predicate = box_stateful_bi_predicate_returning(left).and(box_stateful_bi_predicate_returning(right));
        assert_eq!(predicate.test(&1, &2), expected);
    }

    for (left, right, expected) in [
        (true, true, true),
        (true, false, true),
        (false, true, true),
        (false, false, false),
    ] {
        let mut predicate = box_stateful_bi_predicate_returning(left).or(box_stateful_bi_predicate_returning(right));
        assert_eq!(predicate.test(&1, &2), expected);
    }

    for (left, right, expected) in [
        (true, true, false),
        (true, false, true),
        (false, true, true),
        (false, false, true),
    ] {
        let mut predicate = box_stateful_bi_predicate_returning(left).nand(box_stateful_bi_predicate_returning(right));
        assert_eq!(predicate.test(&1, &2), expected);
    }

    for (left, right, expected) in [
        (true, true, false),
        (true, false, true),
        (false, true, true),
        (false, false, false),
    ] {
        let mut predicate = box_stateful_bi_predicate_returning(left).xor(box_stateful_bi_predicate_returning(right));
        assert_eq!(predicate.test(&1, &2), expected);
    }

    for (left, right, expected) in [
        (true, true, false),
        (true, false, false),
        (false, true, false),
        (false, false, true),
    ] {
        let mut predicate = box_stateful_bi_predicate_returning(left).nor(box_stateful_bi_predicate_returning(right));
        assert_eq!(predicate.test(&1, &2), expected);
    }

    let mut negated = !box_stateful_bi_predicate_returning(false);
    assert!(negated.test(&1, &2));
}

#[test]
fn test_rc_stateful_bi_predicate_common_methods_logical_methods_and_conversions() {
    let mut named = RcStatefulBiPredicate::new_with_name("less_than", |first: &i32, second: &i32| first < second);
    assert!(named.test(&1, &2));
    assert_eq!(named.name(), Some("less_than"));
    named.set_name("ordered");
    assert_eq!(named.name(), Some("ordered"));
    assert_eq!(format!("{named}"), "RcStatefulBiPredicate(ordered)");
    assert!(format!("{named:?}").contains("RcStatefulBiPredicate"));

    let mut always_true = RcStatefulBiPredicate::<i32, i32>::always_true();
    assert!(always_true.test(&1, &2));

    let mut always_false = RcStatefulBiPredicate::<i32, i32>::always_false();
    assert!(!always_false.test(&1, &2));

    let mut and_predicate = RcStatefulBiPredicate::new(|first: &i32, second: &i32| first < second).and(
        RcStatefulBiPredicate::new(|first: &i32, second: &i32| first + second > 0),
    );
    assert!(and_predicate.test(&1, &2));

    let mut or_predicate = RcStatefulBiPredicate::new(|_: &i32, _: &i32| false)
        .or(RcStatefulBiPredicate::new(|first: &i32, second: &i32| first < second));
    assert!(or_predicate.test(&1, &2));

    let mut nand_predicate =
        RcStatefulBiPredicate::new(|_: &i32, _: &i32| true).nand(RcStatefulBiPredicate::new(|_: &i32, _: &i32| true));
    assert!(!nand_predicate.test(&1, &2));

    let mut xor_predicate =
        RcStatefulBiPredicate::new(|_: &i32, _: &i32| true).xor(RcStatefulBiPredicate::new(|_: &i32, _: &i32| false));
    assert!(xor_predicate.test(&1, &2));

    let mut nor_predicate =
        RcStatefulBiPredicate::new(|_: &i32, _: &i32| false).nor(RcStatefulBiPredicate::new(|_: &i32, _: &i32| false));
    assert!(nor_predicate.test(&1, &2));

    let mut negated_value = !RcStatefulBiPredicate::new(|_: &i32, _: &i32| true);
    assert!(!negated_value.test(&1, &2));

    let source = RcStatefulBiPredicate::new_with_name("convertible", |first: &i32, second: &i32| first + second == 5);
    let mut boxed = source.to_box();
    assert!(boxed.test(&2, &3));
    assert_eq!(boxed.name(), Some("convertible"));

    let mut cloned_rc = source.to_rc();
    assert!(cloned_rc.test(&2, &3));

    {
        let mut cloned_fn = source.to_fn();
        assert!(cloned_fn(&2, &3));
    }

    let mut owned_box = source.clone().into_box();
    assert!(owned_box.test(&2, &3));

    let mut owned_rc = source.clone().into_rc();
    assert!(owned_rc.test(&2, &3));

    let mut owned_fn = source.into_fn();
    assert!(owned_fn(&2, &3));
}

#[test]
fn test_arc_stateful_bi_predicate_common_methods_logical_methods_and_conversions() {
    let mut named =
        ArcStatefulBiPredicate::new_with_name("positive_sum", |first: &i32, second: &i32| first + second > 0);
    assert!(named.test(&1, &2));
    assert_eq!(named.name(), Some("positive_sum"));
    named.set_name("sum_positive");
    assert_eq!(named.name(), Some("sum_positive"));
    assert_eq!(format!("{named}"), "ArcStatefulBiPredicate(sum_positive)");
    assert!(format!("{named:?}").contains("ArcStatefulBiPredicate"));

    let mut always_true = ArcStatefulBiPredicate::<i32, i32>::always_true();
    assert!(always_true.test(&1, &2));

    let mut always_false = ArcStatefulBiPredicate::<i32, i32>::always_false();
    assert!(!always_false.test(&1, &2));

    let base = ArcStatefulBiPredicate::new(|first: &i32, second: &i32| first < second);
    let mut and_predicate = base.and(ArcStatefulBiPredicate::new(|first: &i32, second: &i32| {
        first + second > 0
    }));
    assert!(and_predicate.test(&1, &2));

    let mut or_predicate = base.or(ArcStatefulBiPredicate::new(|_: &i32, _: &i32| false));
    assert!(or_predicate.test(&1, &2));

    let mut nand_predicate = base.nand(ArcStatefulBiPredicate::new(|_: &i32, _: &i32| true));
    assert!(!nand_predicate.test(&1, &2));

    let mut xor_predicate = base.xor(ArcStatefulBiPredicate::new(|_: &i32, _: &i32| false));
    assert!(xor_predicate.test(&1, &2));

    let mut nor_predicate = ArcStatefulBiPredicate::new(|_: &i32, _: &i32| false)
        .nor(ArcStatefulBiPredicate::new(|_: &i32, _: &i32| false));
    assert!(nor_predicate.test(&1, &2));

    let mut negated_ref = !&base;
    assert!(!negated_ref.test(&1, &2));

    let mut negated_value = !ArcStatefulBiPredicate::new(|_: &i32, _: &i32| true);
    assert!(!negated_value.test(&1, &2));

    let source = ArcStatefulBiPredicate::new_with_name("convertible", |first: &i32, second: &i32| first + second == 5);
    let mut boxed = source.to_box();
    assert!(boxed.test(&2, &3));
    assert_eq!(boxed.name(), Some("convertible"));

    let mut rc = source.to_rc();
    assert!(rc.test(&2, &3));

    let mut arc = source.to_arc();
    assert!(arc.test(&2, &3));

    {
        let mut function = source.to_fn();
        assert!(function(&2, &3));
    }

    let mut owned_box = source.clone().into_box();
    assert!(owned_box.test(&2, &3));

    let mut owned_rc = source.clone().into_rc();
    assert!(owned_rc.test(&2, &3));

    let mut owned_arc = source.clone().into_arc();
    assert!(owned_arc.test(&2, &3));

    let mut owned_fn = source.into_fn();
    assert!(owned_fn(&2, &3));
}

#[test]
fn test_fn_stateful_bi_predicate_ops_cover_all_logical_methods() {
    let mut or_predicate =
        (|_: &i32, _: &i32| false).or(BoxStatefulBiPredicate::new(|first: &i32, second: &i32| first < second));
    assert!(or_predicate.test(&1, &2));

    let mut nand_predicate = (|_: &i32, _: &i32| true).nand(BoxStatefulBiPredicate::new(|_: &i32, _: &i32| true));
    assert!(!nand_predicate.test(&1, &2));

    let mut xor_predicate = (|_: &i32, _: &i32| true).xor(BoxStatefulBiPredicate::new(|_: &i32, _: &i32| false));
    assert!(xor_predicate.test(&1, &2));

    let mut nor_predicate = (|_: &i32, _: &i32| false).nor(BoxStatefulBiPredicate::new(|_: &i32, _: &i32| false));
    assert!(nor_predicate.test(&1, &2));
}
