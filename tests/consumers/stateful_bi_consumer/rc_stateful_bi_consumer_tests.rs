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
fn test_rc_stateful_bi_consumer_invokes_callback_with_both_inputs() {
    let observed = Rc::new(RefCell::new(None));
    let captured_observed = Rc::clone(&observed);
    let mut consumer = RcStatefulBiConsumer::new(move |first: &i32, second: &i32| {
        *captured_observed.borrow_mut() = Some((*first, *second));
    });

    consumer.accept(&3, &7);

    assert_eq!(*observed.borrow(), Some((3, 7)));
}
