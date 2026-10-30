// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for StatefulMutatingFunction types (stateful FnMut(&mut T) ->
//! R)

use qubit_function::ArcStatefulMutatingFunction;
use qubit_function::StatefulMutatingFunction;

// ============================================================================
// ArcStatefulMutatingFunction Tests
// ============================================================================

#[cfg(test)]
mod test_arc_stateful_mutating_function {
    use std::sync::Arc;
    use std::sync::Barrier;
    use std::thread;

    use super::ArcStatefulMutatingFunction;
    use super::StatefulMutatingFunction;

    #[test]
    fn test_new() {
        let mut counter = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x += 1;
                count
            })
        };
        let mut value = 5;
        assert_eq!(counter.apply(&mut value), 1);
        assert_eq!(value, 6);
        assert_eq!(counter.apply(&mut value), 2);
        assert_eq!(value, 7);
    }

    #[test]
    fn test_clone() {
        let mut counter = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut cloned = counter.clone();
        let mut first = 5;
        let mut second = 3;
        assert_eq!(counter.apply(&mut first), 1);
        assert_eq!(cloned.apply(&mut second), 2);
        assert_eq!(counter.apply(&mut first), 3);
        assert_eq!((first, second), (20, 6));
    }

    #[test]
    fn test_thread_safe() {
        let mut counter = {
            let mut count = 0usize;
            ArcStatefulMutatingFunction::new(move |value: &mut i32| {
                count += 1;
                *value += 1;
                count
            })
        };
        let barrier = Arc::new(Barrier::new(5));
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let mut shared = counter.clone();
                let ready = Arc::clone(&barrier);
                thread::spawn(move || {
                    ready.wait();
                    (0..16)
                        .map(|_| {
                            let mut input = 0;
                            let sequence = shared.apply(&mut input);
                            assert_eq!(input, 1);
                            sequence
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        barrier.wait();
        let mut observed = Vec::new();
        for handle in handles {
            observed.extend(handle.join().expect("worker should not panic"));
        }
        observed.sort_unstable();
        assert_eq!(observed, (1..=64).collect::<Vec<_>>());
        let mut input = 0;
        assert_eq!(counter.apply(&mut input), 65);
        assert_eq!(input, 1);
    }

    #[test]
    fn test_identity() {
        let mut identity = ArcStatefulMutatingFunction::<i32, i32>::identity();
        let mut value = 42;
        assert_eq!(identity.apply(&mut value), 42);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_map() {
        let func = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut mapped = func.and_then::<String, _>(|count: &i32| format!("Call #{}", *count));

        let mut value = 5;
        assert_eq!(mapped.apply(&mut value), "Call #1");
        assert_eq!(value, 10);
    }
}
