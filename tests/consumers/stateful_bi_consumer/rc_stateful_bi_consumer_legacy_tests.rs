// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Tests for the public `RcStatefulBiConsumer` API.

use std::cell::RefCell;
use std::rc::Rc;

use qubit_function::RcStatefulBiConsumer;
use qubit_function::StatefulBiConsumer;

#[test]
fn test_new_invokes_callback_with_both_inputs() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    let mut consumer = RcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
        l.borrow_mut().push(*x + *y);
    });
    consumer.accept(&5, &3);
    assert_eq!(*log.borrow(), vec![8]);
}

#[test]
fn test_new_with_name_sets_name_and_invokes_callback() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    let mut consumer = RcStatefulBiConsumer::new_with_name("test_consumer", move |x: &i32, y: &i32| {
        l.borrow_mut().push(*x + *y);
    });
    assert_eq!(consumer.name(), Some("test_consumer"));
    consumer.accept(&5, &3);
    assert_eq!(*log.borrow(), vec![8]);
}

#[test]
fn test_name_returns_none_without_a_name() {
    let consumer = RcStatefulBiConsumer::new(|_x: &i32, _y: &i32| {});
    assert_eq!(consumer.name(), None);
}

#[test]
fn test_set_name_updates_the_consumer_name() {
    let mut consumer = RcStatefulBiConsumer::new(|_x: &i32, _y: &i32| {});
    assert_eq!(consumer.name(), None);
    consumer.set_name("my_consumer");
    assert_eq!(consumer.name(), Some("my_consumer"));
}

#[test]
fn test_accept_invokes_callback_with_both_inputs() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    let mut consumer = RcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
        l.borrow_mut().push(*x * *y);
    });
    consumer.accept(&5, &3);
    assert_eq!(*log.borrow(), vec![15]);
}

#[test]
fn test_clone_shares_callback_state() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    let consumer = RcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
        l.borrow_mut().push(*x + *y);
    });

    let mut clone1 = consumer.clone();
    let mut clone2 = consumer.clone();

    clone1.accept(&5, &3);
    assert_eq!(*log.borrow(), vec![8]);

    clone2.accept(&10, &2);
    assert_eq!(*log.borrow(), vec![8, 12]);
}

#[test]
fn test_and_then_invokes_callbacks_in_sequence() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let l1 = log.clone();
    let l2 = log.clone();
    let first = RcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
        l1.borrow_mut().push(*x + *y);
    });
    let second = RcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
        l2.borrow_mut().push(*x * *y);
    });

    let mut chained = first.and_then(second);
    chained.accept(&5, &3);
    assert_eq!(*log.borrow(), vec![8, 15]);
}

#[test]
fn test_when_invokes_callback_only_when_predicate_matches() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    let consumer = RcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
        l.borrow_mut().push(*x + *y);
    });
    let mut conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
    conditional.accept(&5, &3);
    assert_eq!(*log.borrow(), vec![8]);
    conditional.accept(&-5, &3);
    assert_eq!(*log.borrow(), vec![8]);
}

#[test]
fn test_stateful_bi_consumer_accept_invokes_callback() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    let mut consumer = RcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
        l.borrow_mut().push(*x + *y);
    });
    consumer.accept(&5, &3);
    assert_eq!(*log.borrow(), vec![8]);
}

#[test]
fn test_debug_includes_consumer_type_name() {
    let consumer = RcStatefulBiConsumer::new(|_x: &i32, _y: &i32| {});
    let debug_str = format!("{:?}", consumer);
    assert!(debug_str.contains("RcStatefulBiConsumer"));
}

#[test]
fn test_display_formats_unnamed_consumer() {
    let consumer = RcStatefulBiConsumer::new(|_x: &i32, _y: &i32| {});
    let display_str = format!("{}", consumer);
    assert_eq!(display_str, "RcStatefulBiConsumer");
}

#[test]
fn test_display_formats_named_consumer() {
    let mut consumer = RcStatefulBiConsumer::new(|_x: &i32, _y: &i32| {});
    consumer.set_name("my_consumer");
    let display_str = format!("{}", consumer);
    assert_eq!(display_str, "RcStatefulBiConsumer(my_consumer)");
}
