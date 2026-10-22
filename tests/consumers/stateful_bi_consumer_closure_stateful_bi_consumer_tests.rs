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
    use std::sync::Arc;
    use std::sync::Mutex;

    use qubit_function::BiConsumerOnce;

    // Test accept() on closure
    #[test]
    fn test_closure_accept() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let closure = move |x: &i32, y: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x + *y);
        };
        closure.accept(&5, &3);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    }
}

// ============================================================================
// Edge Cases and Boundary Conditions Tests
// ============================================================================
