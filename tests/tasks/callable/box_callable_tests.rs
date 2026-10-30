// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxCallable;
use qubit_function::Callable;

#[test]
fn test_box_callable_returns_success_value() {
    let mut callable = BoxCallable::new(|| Ok::<_, std::io::Error>(42));

    assert_eq!(callable.call().expect("call should succeed"), 42);
}
