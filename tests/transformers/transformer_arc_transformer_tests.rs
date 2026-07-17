// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

// ============================================================================
// BoxTransformer Tests - Immutable, single ownership
// ============================================================================

#[cfg(test)]
mod arc_transformer_tests {
    use qubit_function::{
        ArcTransformer,
        Transformer,
    };
    use std::thread;

    #[test]
    fn test_new_and_apply() {
        let double = ArcTransformer::new(|x: i32| x * 2);
        assert_eq!(double.apply(21), 42);
    }

    #[test]
    fn test_clone() {
        let double = ArcTransformer::new(|x: i32| x * 2);
        let cloned = double.clone();

        assert_eq!(double.apply(21), 42);
        assert_eq!(cloned.apply(21), 42);
    }

    #[test]
    fn test_thread_safe() {
        let double = ArcTransformer::new(|x: i32| x * 2);
        let cloned = double.clone();

        let handle = thread::spawn(move || cloned.apply(21));

        assert_eq!(handle.join().expect("thread should not panic"), 42);
        assert_eq!(double.apply(21), 42);
    }

    #[test]
    fn test_identity() {
        let identity = ArcTransformer::<i32, i32>::identity();
        assert_eq!(identity.apply(42), 42);
    }

    #[test]
    fn test_constant() {
        let constant = ArcTransformer::constant("hello");
        assert_eq!(constant.apply(123), "hello");
    }

    #[test]
    fn test_multiple_threads() {
        let square = ArcTransformer::new(|x: i32| x * x);

        let handles: Vec<_> = (0..4)
            .map(|i| {
                let sq = square.clone();
                thread::spawn(move || sq.apply(i))
            })
            .collect();

        let results: Vec<_> = handles
            .into_iter()
            .map(|h| h.join().expect("thread should not panic"))
            .collect();

        assert_eq!(results, vec![0, 1, 4, 9]);
    }

    #[test]
    fn test_and_then() {
        let double = ArcTransformer::new(|x: i32| x * 2);
        let to_string = ArcTransformer::new(|x: i32| x.to_string());
        let composed = double.and_then(to_string);

        // Original double transformer still usable
        assert_eq!(double.apply(21), 42);
        assert_eq!(composed.apply(21), "42");
    }

    #[test]
    fn test_compose() {
        let double = ArcTransformer::new(|x: i32| x * 2);
        let add_one = ArcTransformer::new(|x: i32| x + 1);
        let composed = add_one.and_then(double);

        assert_eq!(composed.apply(5), 12); // (5 + 1) * 2
    }
}

// ============================================================================
// RcTransformer Tests - Immutable, single-threaded
// ============================================================================
