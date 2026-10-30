// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

use qubit_function::ArcFunction;
use qubit_function::Function;

#[test]
fn test_shared_conditional_function_clone_preserves_predicate() {
    let conditional = ArcFunction::new({
        let calls = AtomicI32::new(0);
        move |_: &i32| calls.fetch_add(1, Ordering::SeqCst) + 1
    })
    .when(|value: &i32| *value > 0)
    .or_else(|value: &i32| value + 100);
    let cloned = conditional.clone();

    assert_eq!(conditional.apply(&2), 1);
    assert_eq!(cloned.apply(&3), 2);
    assert_eq!(cloned.apply(&-1), 99);
}
