// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_function::{
    ArcBiTransformer,
    BiTransformer,
    BoxBiTransformer,
    RcBiTransformer,
};
use std::thread;

// ============================================================================
// BoxBiTransformer Tests - Immutable, single ownership
// ============================================================================

#[cfg(test)]
mod rc_bi_transformer_once_tests {
    use super::{
        BiTransformer,
        RcBiTransformer,
    };

    #[test]
    fn test_apply() {
        let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
        assert_eq!(add.apply(20, 22), 42);
    }

    #[test]
    fn test_multiply_once() {
        let multiply = RcBiTransformer::new(|x: i32, y: i32| x * y);
        assert_eq!(multiply.apply(6, 7), 42);
    }

    #[test]
    fn test_string_concatenation_once() {
        let concat =
            RcBiTransformer::new(|x: String, y: String| format!("{} {}", x, y));
        let result = concat.apply("Hello".to_string(), "World".to_string());
        assert_eq!(result, "Hello World");
    }
}

// ============================================================================
// ArcBiTransformer BiTransformerOnce Tests
// ============================================================================
