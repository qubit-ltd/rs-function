/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::RcConditionalConsumer;

#[test]
fn test_rc_conditional_consumer_observable_behavior() {
    let type_name = std::any::type_name::<RcConditionalConsumer<i32>>();
    assert!(type_name.contains("RcConditionalConsumer"), "{type_name}");
}
