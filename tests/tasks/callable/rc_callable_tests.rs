// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::Callable;
use qubit_function::RcCallable;

#[test]
fn test_rc_callable_invokes_callback() {
    let mut callable = RcCallable::new(|| Ok::<_, std::io::Error>(42));

    assert_eq!(callable.call().expect("call should succeed"), 42);
}
