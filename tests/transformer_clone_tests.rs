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
    BoxTransformer,
    Transformer,
};

#[test]
fn test_transformer_clone_observable_behavior() {
    let transformer = BoxTransformer::new(|value: i32| value + 1);
    assert_eq!(transformer.apply(41), 42);
}
