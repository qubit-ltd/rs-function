// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::RefCell;
use std::rc::Rc;

use qubit_function::RcStatefulBiConsumer;
use qubit_function::StatefulBiConsumer;

#[test]
fn test_rc_conditional_stateful_bi_consumer_only_accepts_matching_inputs() {
    let results = Rc::new(RefCell::new(Vec::new()));
    let captured_results = Rc::clone(&results);
    let consumer = RcStatefulBiConsumer::new(move |first: &i32, second: &i32| {
        captured_results.borrow_mut().push(first + second);
    });
    let mut conditional = consumer.when(|first: &i32, _second: &i32| *first > 0);

    conditional.accept(&2, &3);
    conditional.accept(&-1, &3);

    assert_eq!(*results.borrow(), vec![5]);
}
