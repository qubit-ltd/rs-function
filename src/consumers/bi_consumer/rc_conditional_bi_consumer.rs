/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `RcConditionalBiConsumer` public type.

#![allow(unused_imports)]

use super::*;

// =======================================================================
// 9. RcConditionalBiConsumer - Rc-based Conditional BiConsumer
// =======================================================================

/// RcConditionalBiConsumer struct
///
/// A conditional bi-consumer that wraps an `RcBiConsumer` and only executes
/// when a predicate is satisfied. Based on `Rc` for single-threaded shared ownership.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable through `Rc`, allows multiple owners
/// - **Single-Threaded**: Not thread-safe, more efficient than Arc in single-threaded contexts
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **Implements BiConsumer**: Can be used anywhere a `BiConsumer` is expected
/// - **Non-mutating**: Neither modifies itself nor input values
///
pub struct RcConditionalBiConsumer<T, U> {
    pub(super) consumer: RcBiConsumer<T, U>,
    pub(super) predicate: RcBiPredicate<T, U>,
}

// Use macro to generate conditional bi-consumer implementations
impl_shared_conditional_consumer!(
    RcConditionalBiConsumer<T, U>,
    RcBiConsumer,
    BiConsumer,
    into_rc,
    'static
);

// Hand-written BiConsumer trait implementation
impl<T, U> BiConsumer<T, U> for RcConditionalBiConsumer<T, U> {
    fn accept(&self, first: &T, second: &U) {
        if self.predicate.test(first, second) {
            self.consumer.accept(first, second);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_consumer_conversions!(
        BoxBiConsumer<T, U>,
        RcBiConsumer,
        Fn
    );
}

// Use macro to generate Clone implementation
impl_conditional_consumer_clone!(RcConditionalBiConsumer<T, U>);

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(RcConditionalBiConsumer<T, U>);
