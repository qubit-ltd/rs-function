// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::Cell;
use std::rc::Rc;

use qubit_function::RcStatefulConsumer;
use qubit_function::StatefulConsumer;

#[test]
fn test_rc_stateful_consumer_retains_callback_state() {
    let calls = Rc::new(Cell::new(0));
    let captured = Rc::clone(&calls);
    let mut consumer = RcStatefulConsumer::new(move |_value: &i32| captured.set(captured.get() + 1));
    consumer.accept(&5);
    consumer.accept(&3);
    assert_eq!(calls.get(), 2);
}
