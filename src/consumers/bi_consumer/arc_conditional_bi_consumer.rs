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
//! Defines the `ArcConditionalBiConsumer` public type.

use super::{
    ArcBiConsumer,
    ArcBiPredicate,
    BiConsumer,
    BiPredicate,
    BoxBiConsumer,
    RcBiConsumer,
    impl_conditional_consumer_clone,
    impl_conditional_consumer_conversions,
    impl_conditional_consumer_debug_display,
    impl_shared_conditional_consumer,
};

// =======================================================================
// 8. ArcConditionalBiConsumer - Arc-based Conditional BiConsumer
// =======================================================================

/// ArcConditionalBiConsumer struct
///
/// A conditional bi-consumer that wraps an `ArcBiConsumer` and only executes
/// when a predicate is satisfied. Based on `Arc` for thread-safe shared ownership.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable through `Arc`, allows multiple owners
/// - **Thread Safe**: Implements `Send + Sync`, can be safely used concurrently
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **Implements BiConsumer**: Can be used anywhere a `BiConsumer` is expected
/// - **Non-mutating**: Neither modifies itself nor input values
///
pub struct ArcConditionalBiConsumer<T, U> {
    pub(super) consumer: ArcBiConsumer<T, U>,
    pub(super) predicate: ArcBiPredicate<T, U>,
}

// Use macro to generate conditional bi-consumer implementations
impl_shared_conditional_consumer!(
    ArcConditionalBiConsumer<T, U>,
    ArcBiConsumer,
    BiConsumer,
    into_arc,
    Send + Sync + 'static
);

// Hand-written BiConsumer trait implementation
impl<T, U> BiConsumer<T, U> for ArcConditionalBiConsumer<T, U> {
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
impl_conditional_consumer_clone!(ArcConditionalBiConsumer<T, U>);

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(ArcConditionalBiConsumer<T, U>);
