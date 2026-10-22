// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for StatefulConsumer types

// ============================================================================
// BoxConsumer Tests
// ============================================================================

#[cfg(test)]
mod test_box_consumer_chaining {
    use std::sync::Arc;
    use std::sync::Mutex;

    use qubit_function::BoxConsumer;
    use qubit_function::Consumer;
    use qubit_function::StatefulConsumer;

    #[test]
    fn test_and_then_with_closure() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let chained = BoxConsumer::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x * 2);
        })
        .and_then(move |x: &i32| {
            l2.lock().expect("mutex should not be poisoned").push(*x + 10);
        });

        let value = 5;
        chained.accept(&value);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![10, 15]);
    }
}

// ============================================================================
// Name Tests
// ============================================================================
