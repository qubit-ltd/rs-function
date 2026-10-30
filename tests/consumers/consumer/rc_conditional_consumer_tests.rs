// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::RefCell;
use std::rc::Rc;

use qubit_function::Consumer;
use qubit_function::RcConditionalConsumer;
use qubit_function::RcConsumer;

#[test]
fn test_rc_conditional_consumer_selects_branch() {
    let output = Rc::new(RefCell::new(Vec::new()));
    let then_output = Rc::clone(&output);
    let else_output = Rc::clone(&output);
    let consumer = RcConsumer::new(move |value: &i32| then_output.borrow_mut().push(*value));
    let conditional: RcConditionalConsumer<i32> = consumer.when(|value: &i32| *value > 0);
    let conditional = conditional.or_else(move |value: &i32| {
        else_output.borrow_mut().push(-value);
    });
    conditional.accept(&5);
    conditional.accept(&-5);
    assert_eq!(*output.borrow(), vec![5, 5]);
}
