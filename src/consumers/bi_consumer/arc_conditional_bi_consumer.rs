// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcConditionalBiConsumer` public type.

use crate::ArcBiConsumer;
use crate::ArcBiPredicate;
use crate::BiConsumer;
use crate::BiPredicate;
use crate::consumers::macros::impl_conditional_consumer_clone;
use crate::consumers::macros::impl_conditional_consumer_debug_display;
use crate::consumers::macros::impl_shared_conditional_consumer;

// =======================================================================
// 8. ArcConditionalBiConsumer - Arc-based Conditional BiConsumer
// =======================================================================

/// A conditional bi-consumer that shares its callback and predicate through
/// `Arc`, invoking the callback only when the predicate accepts both inputs.
/// The callback and predicate must be thread-safe and `'static`.
///
/// # Type Parameters
///
/// * `T` - The type of the first borrowed input.
/// * `U` - The type of the second borrowed input.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable through `Arc`, allows multiple owners
/// - **Thread Safe**: Implements `Send + Sync`, can be safely used concurrently
/// - **Conditional Execution**: Invokes the callback only when the predicate
///   returns `true`
/// - **Implements BiConsumer**: Can be used anywhere a `BiConsumer` is expected
/// - **Shared access**: Receives shared references; interior mutability and
///   external side effects remain possible
///
/// # Examples
///
/// ```rust
/// use std::sync::Arc;
/// use std::sync::atomic::AtomicI32;
/// use std::sync::atomic::Ordering;
///
/// use qubit_function::ArcBiConsumer;
/// use qubit_function::ArcConditionalBiConsumer;
/// use qubit_function::BiConsumer;
///
/// let total = Arc::new(AtomicI32::new(0));
/// let callback_total = Arc::clone(&total);
/// let consumer = ArcBiConsumer::new(move |first: &i32, second: &i32| {
///     callback_total.fetch_add(first + second, Ordering::SeqCst);
/// })
/// .when(|first: &i32, second: &i32| *first > 0 && *second > 0);
/// let conditional: ArcConditionalBiConsumer<i32, i32> = consumer;
///
/// conditional.accept(&2, &3);
/// conditional.accept(&-1, &5);
/// assert_eq!(total.load(Ordering::SeqCst), 5);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcConditionalBiConsumer<T, U> {
    /// The wrapped consumer callback.
    pub(super) consumer: ArcBiConsumer<T, U>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: ArcBiPredicate<T, U>,
}

// Use macro to generate conditional bi-consumer implementations
impl_shared_conditional_consumer!(
    ArcConditionalBiConsumer<T, U>,
    ArcBiConsumer,
    BiConsumer,
    callback_bounds = (Send + Sync + 'static)
);

// Hand-written BiConsumer trait implementation
impl<T, U> BiConsumer<T, U> for ArcConditionalBiConsumer<T, U> {
    /// Invokes the wrapped consumer when the predicate accepts both inputs.
    fn accept(&self, first: &T, second: &U) {
        if self.predicate.test(first, second) {
            self.consumer.accept(first, second);
        }
    }
}

// Use macro to generate Clone implementation
impl_conditional_consumer_clone!(ArcConditionalBiConsumer<T, U>);

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(ArcConditionalBiConsumer<T, U>);
