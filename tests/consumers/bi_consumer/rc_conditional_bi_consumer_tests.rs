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
fn test_rc_conditional_bi_consumer_observable_behavior() {
    let total = Rc::new(Cell::new(0));
    let callback_total = Rc::clone(&total);
    let consumer = RcBiConsumer::new(move |first: &usize, second: &usize| {
        callback_total.set(callback_total.get() + first + second);
    });
    let conditional_consumer = consumer.when(|first: &usize, second: &usize| first + second > 10);

    conditional_consumer.accept(&2, &3);
    conditional_consumer.accept(&5, &7);

    assert_eq!(total.get(), 12, "only matching input should be consumed");
}
