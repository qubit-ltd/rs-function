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
    let first = Borrowed {
        value: left.as_str(),
    };
    let second = Borrowed {
        value: right.as_str(),
    };
    let predicate = BorrowedStatefulBiPredicate {
        count: Cell::new(0),
    };

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
    let mut predicate = BoxStatefulBiPredicate::new_with_name(
        "every_second_positive_sum",
        move |first: &i32, second: &i32| {
            calls += 1;
            calls % 2 == 0 && first + second > 0
        },
    );

    assert_eq!(predicate.name(), Some("every_second_positive_sum"));
    assert!(!predicate.test(&5, &3));
    assert!(predicate.test(&5, &3));

    let rhs_calls = Rc::new(Cell::new(0));
    let rhs_seen = rhs_calls.clone();
    let mut and_predicate = BoxStatefulBiPredicate::new(|_: &i32, _: &i32| false).and(
        BoxStatefulBiPredicate::new(move |_: &i32, _: &i32| {
            rhs_seen.set(rhs_seen.get() + 1);
            true
        }),
    );

    assert!(!and_predicate.test(&1, &2));
    assert_eq!(rhs_calls.get(), 0);

    let rhs_calls = Rc::new(Cell::new(0));
    let rhs_seen = rhs_calls.clone();
    let mut or_predicate = BoxStatefulBiPredicate::new(|_: &i32, _: &i32| true).or(
        BoxStatefulBiPredicate::new(move |_: &i32, _: &i32| {
            rhs_seen.set(rhs_seen.get() + 1);
            false
        }),
    );

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
        observed
            .lock()
            .expect("mutex should not be poisoned")
            .push(sum);
        sum > 0
    });

    let mut thread_predicate = predicate.clone();
    let handle = thread::spawn(move || thread_predicate.test(&5, &3));
    assert!(handle.join().expect("thread should not panic"));

    let mut local_predicate = predicate.clone();
    assert!(!local_predicate.test(&-10, &2));
    assert_eq!(
        *log.lock().expect("mutex should not be poisoned"),
        vec![8, -8]
    );
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
