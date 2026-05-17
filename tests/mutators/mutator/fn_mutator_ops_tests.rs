/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnMutatorOps;

#[test]
fn test_fn_mutator_ops_observable_behavior() {
    fn assert_ops<F: FnMutatorOps<i32>>(_: &F) {}

    let mutator = |value: &mut i32| *value += 1;
    assert_ops(&mutator);
    let mut value = 41;
    mutator(&mut value);
    assert_eq!(value, 42);
}
