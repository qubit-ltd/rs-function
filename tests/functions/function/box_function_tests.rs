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
fn test_box_function_new_allows_non_static_t() {
    fn run<'a>(value: &'a str) -> usize {
        let func: BoxFunction<&'a str, usize> = BoxFunction::new(|x: &&'a str| x.len());
        func.apply(&value)
    }

    let text = String::from("hello");
    assert_eq!(run(text.as_str()), 5);
}

#[test]
fn test_box_function_new_allows_non_static_r() {
    fn run<'a>(value: &'a str) -> &'a str {
        let func: BoxFunction<&'a str, &'a str> = BoxFunction::new(|x: &&'a str| *x);
        func.apply(&value)
    }

    let text = String::from("qubit");
    assert_eq!(run(text.as_str()), "qubit");
}

#[test]
fn test_box_function_applies_callback_repeatedly() {
    let add_one = BoxFunction::new(|value: &i32| value + 1);

    assert_eq!(add_one.apply(&1), 2);
    assert_eq!(add_one.apply(&2), 3);
}
