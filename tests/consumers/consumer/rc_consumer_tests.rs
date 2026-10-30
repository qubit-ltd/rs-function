// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::Cell;
use std::rc::Rc;

use qubit_function::Consumer;
use qubit_function::RcConsumer;

#[test]
fn test_rc_consumer_accepts_input() {
    let consumed = Rc::new(Cell::new(0));
    let captured = Rc::clone(&consumed);
    let consumer = RcConsumer::new(move |value: &i32| captured.set(*value));
    consumer.accept(&7);
    assert_eq!(consumed.get(), 7);
}
