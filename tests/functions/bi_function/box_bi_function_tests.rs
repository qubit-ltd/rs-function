// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BiFunction;
use qubit_function::BoxBiFunction;

#[test]
fn test_box_bi_function_new_allows_non_static_t() {
    fn run<'a>(value: &'a str) -> usize {
        let func: BoxBiFunction<&'a str, i32, usize> =
            BoxBiFunction::new(|x: &&'a str, y: &i32| x.len() + (*y as usize));
        func.apply(&value, &3)
    }

    let text = String::from("hello");
    assert_eq!(run(text.as_str()), 8);
}

#[test]
fn test_box_bi_function_new_allows_non_static_u() {
    fn run<'a>(value: &'a str) -> usize {
        let func: BoxBiFunction<i32, &'a str, usize> =
            BoxBiFunction::new(|x: &i32, y: &&'a str| (*x as usize) + y.len());
        func.apply(&3, &value)
    }

    let text = String::from("world");
    assert_eq!(run(text.as_str()), 8);
}

#[test]
fn test_box_bi_function_new_allows_non_static_r() {
    fn run<'a>(value: &'a str) -> &'a str {
        let func: BoxBiFunction<&'a str, i32, &'a str> = BoxBiFunction::new(|x: &&'a str, _y: &i32| *x);
        func.apply(&value, &0)
    }

    let text = String::from("qubit");
    assert_eq!(run(text.as_str()), "qubit");
}

#[test]
fn test_box_bi_function_applies_callback_to_both_inputs() {
    let function = BoxBiFunction::new(|first: &i32, second: &i32| first + second);

    assert_eq!(function.apply(&2, &3), 5);
}
