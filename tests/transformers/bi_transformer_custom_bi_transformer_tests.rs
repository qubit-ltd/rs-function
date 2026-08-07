// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use std::thread;

use qubit_function::ArcBiTransformer;
use qubit_function::BiTransformer;
use qubit_function::BoxBiTransformer;
use qubit_function::RcBiTransformer;

// ============================================================================
// BoxBiTransformer Tests - Immutable, single ownership
// ============================================================================

#[cfg(test)]
mod custom_bi_transformer_tests {
    use super::BiTransformer;

    /// Custom BiTransformer implementation for testing default into_xxx()
    /// methods
    struct CustomBiTransformer {
        multiplier: i32,
    }

    impl CustomBiTransformer {
        fn new(multiplier: i32) -> Self {
            Self { multiplier }
        }
    }

    impl BiTransformer<i32, i32, i32> for CustomBiTransformer {
        fn apply(&self, first: i32, second: i32) -> i32 {
            (first + second) * self.multiplier
        }
    }

    #[test]
    fn test_custom_bi_transformer_apply() {
        let transformer = CustomBiTransformer::new(3);
        assert_eq!(transformer.apply(5, 10), 45); // (5 + 10) * 3 = 45
    }

    /// Custom Send + Sync BiTransformer implementation
    struct ThreadSafeBiTransformer {
        multiplier: i32,
    }

    impl ThreadSafeBiTransformer {
        fn new(multiplier: i32) -> Self {
            Self { multiplier }
        }
    }

    impl BiTransformer<i32, i32, i32> for ThreadSafeBiTransformer {
        fn apply(&self, first: i32, second: i32) -> i32 {
            (first + second) * self.multiplier
        }
    }

    // Manual implementation of Send and Sync
    unsafe impl Send for ThreadSafeBiTransformer {}
    unsafe impl Sync for ThreadSafeBiTransformer {}

    /// Test custom BiTransformer with different types combination
    struct StringCombiner {
        separator: String,
    }

    impl StringCombiner {
        fn new(separator: &str) -> Self {
            Self {
                separator: separator.to_string(),
            }
        }
    }

    impl BiTransformer<String, String, String> for StringCombiner {
        fn apply(&self, first: String, second: String) -> String {
            format!("{}{}{}", first, self.separator, second)
        }
    }

    /// Test custom BiTransformer's default to_xxx() implementations
    /// These are default implementations provided by the BiTransformer trait,
    /// requiring the type to implement Clone
    #[derive(Clone)]
    struct CloneableCustomBiTransformer {
        multiplier: i32,
    }

    impl CloneableCustomBiTransformer {
        fn new(multiplier: i32) -> Self {
            Self { multiplier }
        }
    }

    impl BiTransformer<i32, i32, i32> for CloneableCustomBiTransformer {
        fn apply(&self, first: i32, second: i32) -> i32 {
            (first + second) * self.multiplier
        }
    }

    /// Test custom Send + Sync BiTransformer's default to_arc() implementation
    #[derive(Clone)]
    struct ThreadSafeCloneableBiTransformer {
        multiplier: i32,
    }

    impl ThreadSafeCloneableBiTransformer {
        fn new(multiplier: i32) -> Self {
            Self { multiplier }
        }
    }

    impl BiTransformer<i32, i32, i32> for ThreadSafeCloneableBiTransformer {
        fn apply(&self, first: i32, second: i32) -> i32 {
            (first + second) * self.multiplier
        }
    }

    // Manual implementation of Send and Sync
    unsafe impl Send for ThreadSafeCloneableBiTransformer {}
    unsafe impl Sync for ThreadSafeCloneableBiTransformer {}

    /// Test custom string type's default to_xxx() methods
    #[derive(Clone)]
    struct CloneableStringCombiner {
        separator: String,
    }

    impl CloneableStringCombiner {
        fn new(separator: &str) -> Self {
            Self {
                separator: separator.to_string(),
            }
        }
    }

    impl BiTransformer<String, String, String> for CloneableStringCombiner {
        fn apply(&self, first: String, second: String) -> String {
            format!("{}{}{}", first, self.separator, second)
        }
    }
}

// ============================================================================
// BiTransformer Default Methods - to_xxx() Non-consuming Conversions
// ============================================================================

// ============================================================================
// Closure BiTransformer to_xxx() Methods Tests
// ============================================================================

// ============================================================================
// Complete to_xxx() Test Coverage for All Types
// ============================================================================

// ============================================================================
// Consuming into_xxx() and Non-consuming to_xxx() Comparison Tests
// ============================================================================

// ============================================================================
// BiTransformer Default Trait Methods - into_xxx() with Various Inputs
// ============================================================================

// ============================================================================
// Type Conversion Chain Tests
// ============================================================================

// ============================================================================
// String and Complex Types Conversion Tests
// ============================================================================

// ============================================================================
// Send+Sync Verification Tests for Arc Conversions
// ============================================================================

// ============================================================================
// BoxBiTransformer BiTransformerOnce Tests
// ============================================================================
