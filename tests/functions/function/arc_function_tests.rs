// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcFunction;
use qubit_function::Function;

#[test]
fn test_arc_function_new_allows_non_static_t() {
    fn run<'a>(value: &'a str) -> usize {
        let func: ArcFunction<&'a str, usize> = ArcFunction::new(|x: &&'a str| x.len() + 1);
        func.apply(&value)
    }

    let text = String::from("hello");
    assert_eq!(run(text.as_str()), 6);
}

#[test]
fn test_arc_function_applies_callback_after_clone() {
    let double = ArcFunction::new(|value: &i32| value * 2);
    let cloned = double.clone();

    assert_eq!(cloned.apply(&21), 42);
}
