// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxCallableOnce;
use qubit_function::CallableOnce;

#[test]
fn test_box_callable_once_moves_non_clone_capture() {
    struct Payload(&'static str);

    let payload = Payload("owned");
    let callable = BoxCallableOnce::new(move || Ok::<_, std::io::Error>(payload));

    assert_eq!(callable.call_once().expect("call should succeed").0, "owned");
}
