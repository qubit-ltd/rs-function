// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxFunction;
use qubit_function::Function;

#[test]
fn test_box_function_constructor_name_variants() {
    let unnamed = BoxFunction::new(|value: &i32| value + 1);
    assert_eq!(unnamed.name(), None);
    assert_eq!(unnamed.apply(&41), 42);
    let named = BoxFunction::new_with_name("named", |value: &i32| value + 1);
    assert_eq!(named.name(), Some("named"));
    assert_eq!(named.apply(&41), 42);
    let optional_none = BoxFunction::new_with_optional_name(|value: &i32| value + 1, None);
    assert_eq!(optional_none.name(), None);
    assert_eq!(optional_none.apply(&41), 42);
    let optional_some = BoxFunction::new_with_optional_name(|value: &i32| value + 1, Some(String::from("optional")));
    assert_eq!(optional_some.name(), Some("optional"));
    assert_eq!(optional_some.apply(&41), 42);
}
