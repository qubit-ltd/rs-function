/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::{
    BoxMutator,
    Mutator,
};

#[test]
fn test_box_mutator_methods_observable_behavior() {
    let mutator = BoxMutator::new(|value: &mut i32| *value += 1);
    let mut value = 41;
    mutator.apply(&mut value);
    assert_eq!(value, 42);
}
