// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for StatefulMutator types

use qubit_function::MutatorOnce;

#[test]
fn test_closure_accepts_mutator_once() {
    let closure = |x: &mut i32| *x *= 2;
    let mut value = 5;
    closure.apply(&mut value);
    assert_eq!(value, 10);
}
