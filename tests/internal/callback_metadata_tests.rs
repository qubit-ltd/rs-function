// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxFunction;

#[test]
fn test_callback_metadata_is_observable_through_wrapper_names() {
    let mut function = BoxFunction::<i32, i32>::new(|value: &i32| *value);
    assert_eq!(function.name(), None);
    function.set_name("identity");
    assert_eq!(function.name(), Some("identity"));
    function.clear_name();
    assert_eq!(function.name(), None);
}
