// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::io::Error;

use qubit_function::ArcCallable;
use qubit_function::Callable;

#[test]
fn test_arc_callable_invokes_mutable_callback() {
    let mut count = 0;
    let mut callable = ArcCallable::<i32, Error>::new(move || {
        count += 1;
        Ok(count)
    });

    assert_eq!(callable.call().expect("call should succeed"), 1);
    assert_eq!(callable.call().expect("call should succeed"), 2);
}
