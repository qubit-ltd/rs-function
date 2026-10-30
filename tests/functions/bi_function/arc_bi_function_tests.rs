// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcBiFunction;
use qubit_function::BiFunction;

#[test]
fn test_arc_bi_function_new_allows_non_static_t() {
    fn run<'a>(value: &'a str) -> usize {
        let func: ArcBiFunction<&'a str, i32, usize> =
            ArcBiFunction::new(|x: &&'a str, y: &i32| x.len() + (*y as usize));
        func.apply(&value, &3)
    }

    let text = String::from("hello");
    assert_eq!(run(text.as_str()), 8);
}

#[test]
fn test_arc_bi_function_new_allows_non_static_u() {
    fn run<'a>(value: &'a str) -> usize {
        let func: ArcBiFunction<i32, &'a str, usize> =
            ArcBiFunction::new(|x: &i32, y: &&'a str| (*x as usize) + y.len());
        func.apply(&3, &value)
    }

    let text = String::from("world");
    assert_eq!(run(text.as_str()), 8);
}

#[test]
fn test_arc_bi_function_new_allows_non_static_r() {
    fn run<'a>(value: &'a str) -> &'a str {
        let func: ArcBiFunction<&'a str, i32, &'a str> = ArcBiFunction::new(|x: &&'a str, _y: &i32| *x);
        func.apply(&value, &0)
    }

    let text = String::from("qubit");
    assert_eq!(run(text.as_str()), "qubit");
}

#[test]
fn test_arc_bi_function_applies_shared_callback() {
    let function = ArcBiFunction::new(|left: &i32, right: &i32| left - right);
    let cloned = function.clone();
    assert_eq!(function.apply(&9, &4), 5);
    assert_eq!(cloned.apply(&4, &9), -5);
}
