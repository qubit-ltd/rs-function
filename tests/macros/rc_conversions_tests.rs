// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Confirms `RcFunction` clones share the Rc-backed callback and retain naming.

use std::cell::Cell;
use std::rc::Rc;

use qubit_function::Function;
use qubit_function::RcFunction;

#[test]
fn test_rc_function_clone_shares_callback_and_preserves_name() {
    let calls = Rc::new(Cell::new(0));
    let captured_calls = Rc::clone(&calls);
    let mut function = RcFunction::new_with_name("shared", move |value: &i32| {
        captured_calls.set(captured_calls.get() + 1);
        value + 1
    });
    let cloned = function.clone();

    assert_eq!(function.name(), Some("shared"));
    function.set_name("renamed");
    assert_eq!(function.apply(&41), 42);
    assert_eq!(cloned.apply(&1), 2);
    assert_eq!(calls.get(), 2);
    assert_eq!(format!("{}", function), "RcFunction(renamed)");
}
