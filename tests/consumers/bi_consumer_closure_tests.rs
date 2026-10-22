// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
#[cfg(test)]
mod closure_tests {
    use std::sync::Arc;

    use qubit_function::BiConsumer;
    use qubit_function::BoxBiConsumer;

    #[test]
    fn test_closure_accept() {
        let closure = |x: &i32, y: &i32| {
            std::hint::black_box(x + y);
        };
        closure.accept(&5, &3);
    }

    #[test]
    fn test_closure_and_then() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let chained = BoxBiConsumer::new(move |_x: &i32, _y: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32, _y: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        });

        chained.accept(&5, &3);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 2);
    }
}

// ============================================================================
// Edge Cases Tests
// ============================================================================
