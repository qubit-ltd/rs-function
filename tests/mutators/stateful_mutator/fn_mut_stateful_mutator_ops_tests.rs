/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnMutStatefulMutatorOps;

#[test]
fn test_fn_mut_stateful_mutator_ops_observable_behavior() {
    fn assert_ops<F: FnMutStatefulMutatorOps<i32>>(_: &F) {}

    let mut delta = 0;
    let mut mutator = |value: &mut i32| {
        delta += 1;
        *value += delta;
    };
    assert_ops(&mutator);
    let mut value = 41;
    mutator(&mut value);
    assert_eq!(value, 42);
}
