/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::testers::stateful_tester::{
    StatefulTester,
    arc_stateful_tester::ArcStatefulTester,
    box_stateful_tester::BoxStatefulTester,
    fn_stateful_tester_ops::FnStatefulTesterOps,
    rc_stateful_tester::RcStatefulTester,
};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        Arc,
        atomic::{
            AtomicUsize,
            Ordering,
        },
    },
    thread,
};

#[derive(Clone)]
struct ThresholdTester {
    count: usize,
    threshold: usize,
}

impl StatefulTester for ThresholdTester {
    fn test(&mut self) -> bool {
        self.count += 1;
        self.count >= self.threshold
    }
}

#[test]
fn test_stateful_tester_closure_mutates_state() {
    let mut count = 0;
    let mut tester = move || {
        count += 1;
        count >= 2
    };

    assert!(!StatefulTester::test(&mut tester));
    assert!(StatefulTester::test(&mut tester));
}

#[test]
fn test_stateful_tester_into_fn_returns_fn_mut() {
    let tester = ThresholdTester {
        count: 0,
        threshold: 3,
    };
    let mut function = tester.into_fn();

    assert!(!function());
    assert!(!function());
    assert!(function());
}

#[test]
fn test_stateful_tester_into_mut_fn_returns_fn_mut() {
    let tester = ThresholdTester {
        count: 0,
        threshold: 2,
    };
    let mut function = tester.into_mut_fn();

    assert!(!function());
    assert!(function());
}

#[test]
fn test_stateful_tester_to_fn_uses_cloned_state() {
    let tester = ThresholdTester {
        count: 0,
        threshold: 2,
    };
    let mut function = tester.to_fn();

    assert!(!function());
    assert!(function());

    let mut original = tester.clone();
    assert!(!original.test());
}

#[test]
fn test_stateful_tester_to_mut_fn_uses_cloned_state() {
    let tester = ThresholdTester {
        count: 0,
        threshold: 2,
    };
    let mut function = tester.to_mut_fn();

    assert!(!function());
    assert!(function());

    let mut original = tester.clone();
    assert!(!original.test());
}

#[test]
fn test_stateful_tester_default_wrapper_conversions() {
    let tester = ThresholdTester {
        count: 0,
        threshold: 2,
    };

    let mut boxed = tester.clone().into_box();
    assert!(!boxed.test());
    assert!(boxed.test());

    let mut rc = tester.clone().into_rc();
    assert!(!rc.test());
    assert!(rc.test());

    let mut arc = tester.clone().into_arc();
    assert!(!arc.test());
    assert!(arc.test());

    let mut boxed_from_ref = tester.to_box();
    assert!(!boxed_from_ref.test());
    assert!(boxed_from_ref.test());

    let mut rc_from_ref = tester.to_rc();
    assert!(!rc_from_ref.test());
    assert!(rc_from_ref.test());

    let mut arc_from_ref = tester.to_arc();
    assert!(!arc_from_ref.test());
    assert!(arc_from_ref.test());
}

#[test]
fn test_box_stateful_tester_conversions_and_negation() {
    let mut tester = BoxStatefulTester::new({
        let mut count = 0;
        move || {
            count += 1;
            count >= 2
        }
    });
    assert!(!tester.test());
    assert!(tester.test());

    let mut boxed = BoxStatefulTester::new(|| true).into_box();
    assert!(boxed.test());

    let mut rc = BoxStatefulTester::new(|| true).into_rc();
    assert!(rc.test());

    let mut function = BoxStatefulTester::new(|| true).into_fn();
    assert!(function());

    let mut negated = !BoxStatefulTester::new(|| true);
    assert!(!negated.test());
}

#[test]
fn test_box_stateful_tester_logical_operations_cover_branches() {
    let skipped = Rc::new(RefCell::new(false));
    let skipped_clone = Rc::clone(&skipped);
    let mut and_short = BoxStatefulTester::new(|| false).and(BoxStatefulTester::new(move || {
        *skipped_clone.borrow_mut() = true;
        true
    }));
    assert!(!and_short.test());
    assert!(!*skipped.borrow());

    let mut and_true = BoxStatefulTester::new(|| true).and(BoxStatefulTester::new(|| true));
    assert!(and_true.test());

    let skipped = Rc::new(RefCell::new(false));
    let skipped_clone = Rc::clone(&skipped);
    let mut or_short = BoxStatefulTester::new(|| true).or(BoxStatefulTester::new(move || {
        *skipped_clone.borrow_mut() = true;
        false
    }));
    assert!(or_short.test());
    assert!(!*skipped.borrow());

    let mut or_false = BoxStatefulTester::new(|| false).or(BoxStatefulTester::new(|| false));
    assert!(!or_false.test());

    let mut nand_false = BoxStatefulTester::new(|| true).nand(BoxStatefulTester::new(|| true));
    assert!(!nand_false.test());

    let mut nand_true = BoxStatefulTester::new(|| false).nand(BoxStatefulTester::new(|| true));
    assert!(nand_true.test());

    let mut xor_true = BoxStatefulTester::new(|| true).xor(BoxStatefulTester::new(|| false));
    assert!(xor_true.test());

    let mut xor_false = BoxStatefulTester::new(|| true).xor(BoxStatefulTester::new(|| true));
    assert!(!xor_false.test());

    let mut nor_true = BoxStatefulTester::new(|| false).nor(BoxStatefulTester::new(|| false));
    assert!(nor_true.test());

    let mut nor_false = BoxStatefulTester::new(|| true).nor(BoxStatefulTester::new(|| false));
    assert!(!nor_false.test());
}

#[test]
fn test_rc_stateful_tester_clone_shares_state() {
    let calls = Rc::new(RefCell::new(0));
    let calls_clone = Rc::clone(&calls);
    let mut tester = RcStatefulTester::new(move || {
        let mut calls = calls_clone.borrow_mut();
        *calls += 1;
        *calls >= 2
    });
    let mut clone = tester.clone();

    assert!(!tester.test());
    assert!(clone.test());
    assert_eq!(*calls.borrow(), 2);
}

#[test]
fn test_rc_stateful_tester_conversions_and_negation() {
    let tester = RcStatefulTester::new(|| true);

    let mut boxed = tester.to_box();
    assert!(boxed.test());

    let mut rc_clone = tester.to_rc();
    assert!(rc_clone.test());

    {
        let mut function = tester.to_fn();
        assert!(function());
    }

    let mut boxed_from_owned = tester.clone().into_box();
    assert!(boxed_from_owned.test());

    let mut rc_from_owned = tester.clone().into_rc();
    assert!(rc_from_owned.test());

    let mut function_from_owned = tester.clone().into_fn();
    assert!(function_from_owned());

    let mut borrowed_negated = !&tester;
    assert!(!borrowed_negated.test());

    let mut owned_negated = !tester;
    assert!(!owned_negated.test());
}

#[test]
fn test_rc_stateful_tester_logical_operations_cover_branches() {
    let first = RcStatefulTester::new(|| false);
    let mut and_false = first.and(RcStatefulTester::new(|| true));
    assert!(!and_false.test());

    let first = RcStatefulTester::new(|| true);
    let mut and_true = first.and(RcStatefulTester::new(|| true));
    assert!(and_true.test());

    let first = RcStatefulTester::new(|| true);
    let mut or_true = first.or(RcStatefulTester::new(|| false));
    assert!(or_true.test());

    let first = RcStatefulTester::new(|| false);
    let mut or_false = first.or(RcStatefulTester::new(|| false));
    assert!(!or_false.test());

    let first = RcStatefulTester::new(|| true);
    let mut nand_false = first.nand(RcStatefulTester::new(|| true));
    assert!(!nand_false.test());

    let first = RcStatefulTester::new(|| false);
    let mut nand_true = first.nand(RcStatefulTester::new(|| true));
    assert!(nand_true.test());

    let first = RcStatefulTester::new(|| true);
    let mut xor_true = first.xor(RcStatefulTester::new(|| false));
    assert!(xor_true.test());

    let first = RcStatefulTester::new(|| true);
    let mut xor_false = first.xor(RcStatefulTester::new(|| true));
    assert!(!xor_false.test());

    let first = RcStatefulTester::new(|| false);
    let mut nor_true = first.nor(RcStatefulTester::new(|| false));
    assert!(nor_true.test());

    let first = RcStatefulTester::new(|| true);
    let mut nor_false = first.nor(RcStatefulTester::new(|| false));
    assert!(!nor_false.test());
}

#[test]
fn test_arc_stateful_tester_clone_shares_state_across_threads() {
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_clone = Arc::clone(&calls);
    let mut tester = ArcStatefulTester::new(move || {
        calls_clone.fetch_add(1, Ordering::SeqCst);
        true
    });
    let clone = tester.clone();

    assert!(tester.test());
    let handle = thread::spawn(move || {
        let mut clone = clone;
        clone.test()
    });
    assert!(handle.join().expect("thread should not panic"));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
fn test_arc_stateful_tester_conversions_and_negation() {
    let tester = ArcStatefulTester::new(|| true);

    let mut boxed = tester.to_box();
    assert!(boxed.test());

    let mut rc = tester.to_rc();
    assert!(rc.test());

    let mut arc_clone = tester.to_arc();
    assert!(arc_clone.test());

    {
        let mut function = tester.to_fn();
        assert!(function());
    }

    let mut boxed_from_owned = tester.clone().into_box();
    assert!(boxed_from_owned.test());

    let mut rc_from_owned = tester.clone().into_rc();
    assert!(rc_from_owned.test());

    let mut arc_from_owned = tester.clone().into_arc();
    assert!(arc_from_owned.test());

    let mut function_from_owned = tester.clone().into_fn();
    assert!(function_from_owned());

    let mut borrowed_negated = !&tester;
    assert!(!borrowed_negated.test());

    let mut owned_negated = !tester;
    assert!(!owned_negated.test());
}

#[test]
fn test_arc_stateful_tester_logical_operations_cover_branches() {
    let first = ArcStatefulTester::new(|| false);
    let mut and_false = first.and(ArcStatefulTester::new(|| true));
    assert!(!and_false.test());

    let first = ArcStatefulTester::new(|| true);
    let mut and_true = first.and(ArcStatefulTester::new(|| true));
    assert!(and_true.test());

    let first = ArcStatefulTester::new(|| true);
    let mut or_true = first.or(ArcStatefulTester::new(|| false));
    assert!(or_true.test());

    let first = ArcStatefulTester::new(|| false);
    let mut or_false = first.or(ArcStatefulTester::new(|| false));
    assert!(!or_false.test());

    let first = ArcStatefulTester::new(|| true);
    let mut nand_false = first.nand(ArcStatefulTester::new(|| true));
    assert!(!nand_false.test());

    let first = ArcStatefulTester::new(|| false);
    let mut nand_true = first.nand(ArcStatefulTester::new(|| true));
    assert!(nand_true.test());

    let first = ArcStatefulTester::new(|| true);
    let mut xor_true = first.xor(ArcStatefulTester::new(|| false));
    assert!(xor_true.test());

    let first = ArcStatefulTester::new(|| true);
    let mut xor_false = first.xor(ArcStatefulTester::new(|| true));
    assert!(!xor_false.test());

    let first = ArcStatefulTester::new(|| false);
    let mut nor_true = first.nor(ArcStatefulTester::new(|| false));
    assert!(nor_true.test());

    let first = ArcStatefulTester::new(|| true);
    let mut nor_false = first.nor(ArcStatefulTester::new(|| false));
    assert!(!nor_false.test());
}

#[test]
fn test_fn_stateful_tester_ops_logical_operations_cover_branches() {
    let mut and_true = FnStatefulTesterOps::and(|| true, || true);
    assert!(and_true.test());

    let mut and_false = FnStatefulTesterOps::and(|| false, || true);
    assert!(!and_false.test());

    let mut or_true = FnStatefulTesterOps::or(|| true, || false);
    assert!(or_true.test());

    let mut or_false = FnStatefulTesterOps::or(|| false, || false);
    assert!(!or_false.test());

    let mut not_true = FnStatefulTesterOps::not(|| false);
    assert!(not_true.test());

    let mut nand_false = FnStatefulTesterOps::nand(|| true, || true);
    assert!(!nand_false.test());

    let mut nand_true = FnStatefulTesterOps::nand(|| false, || true);
    assert!(nand_true.test());

    let mut xor_true = FnStatefulTesterOps::xor(|| true, || false);
    assert!(xor_true.test());

    let mut xor_false = FnStatefulTesterOps::xor(|| true, || true);
    assert!(!xor_false.test());

    let mut nor_true = FnStatefulTesterOps::nor(|| false, || false);
    assert!(nor_true.test());

    let mut nor_false = FnStatefulTesterOps::nor(|| true, || false);
    assert!(!nor_false.test());
}
