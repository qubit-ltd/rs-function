// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Transformer Demo
//!
//! Demonstrates consuming a `TransformerOnce` callback and sharing an
//! `ArcTransformer` between the current thread and a worker thread.

use std::thread;

use qubit_function::ArcTransformer;
use qubit_function::BoxTransformerOnce;
use qubit_function::Transformer;
use qubit_function::TransformerOnce;

fn main() {
    let suffix = String::from("!");
    let once = BoxTransformerOnce::new(move |mut input: String| {
        input.push_str(&suffix);
        (input, suffix)
    });
    let result = TransformerOnce::apply(once, String::from("ready"));
    assert_eq!(result, (String::from("ready!"), String::from("!")));
    println!("Once callback transferred its captured String: {result:?}");

    let double = ArcTransformer::new(|value: i32| value * 2);
    let shared = double.clone();
    let worker = thread::spawn(move || Transformer::apply(&shared, 21));
    assert_eq!(Transformer::apply(&double, 10), 20);
    assert_eq!(worker.join().expect("worker should not panic"), 42);
}
