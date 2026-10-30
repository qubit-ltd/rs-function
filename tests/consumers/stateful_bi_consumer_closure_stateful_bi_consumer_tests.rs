// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Tests for closures implementing `StatefulBiConsumer`.

#[cfg(test)]
mod closure_stateful_bi_consumer_tests {
    use qubit_function::StatefulBiConsumer;

    // Test accept() on closure
    #[test]
    fn test_closure_accept_updates_captured_state() {
        let mut sums = Vec::new();
        let mut consumer = |left: &i32, right: &i32| sums.push(left + right);
        consumer.accept(&5, &3);
        consumer.accept(&2, &4);
        assert_eq!(sums, vec![8, 6]);
    }
}
