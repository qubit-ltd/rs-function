// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for BiTransformerOnce trait and implementations

use qubit_function::BiTransformerOnce;
use qubit_function::BoxBiTransformerOnce;

// ============================================================================
// Tests for BiTransformerOnce trait
// ============================================================================

#[cfg(test)]
mod trait_tests {
    use super::BiTransformerOnce;

    #[test]
    fn test_blanket_impl_with_closure() {
        let add = |x: i32, y: i32| x + y;
        assert_eq!(add.apply(20, 22), 42);
    }

    #[test]
    fn test_blanket_impl_with_function() {
        fn multiply(x: i32, y: i32) -> i32 {
            x * y
        }
        assert_eq!(multiply.apply(6, 7), 42);
    }

    #[test]
    fn test_blanket_impl_with_consuming_closure() {
        let owned_x = String::from("hello");
        let owned_y = String::from("world");
        let concat = |x: String, y: String| format!("{} {}", x, y);
        assert_eq!(concat.apply(owned_x, owned_y), "hello world");
    }
}
// ============================================================================
// Tests for BoxBiTransformerOnce
// ============================================================================
