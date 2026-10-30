// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::Cell;
use std::rc::Rc;

use qubit_function::BiConsumer;
use qubit_function::RcBiConsumer;

#[test]
fn test_rc_bi_consumer_accepts_inputs() {
    let sum = Rc::new(Cell::new(0));
    let captured = Rc::clone(&sum);
    let consumer = RcBiConsumer::new(move |left: &i32, right: &i32| captured.set(left + right));
    consumer.accept(&5, &3);
    assert_eq!(sum.get(), 8);
}
