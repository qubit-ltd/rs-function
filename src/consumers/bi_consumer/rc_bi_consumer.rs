// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcBiConsumer` public type.

use std::rc::Rc;

use super::BiConsumerFn;
use crate::BiConsumer;
use crate::BiPredicate;
use crate::RcConditionalBiConsumer;
use crate::consumers::macros::impl_consumer_clone;
use crate::consumers::macros::impl_consumer_common_methods;
use crate::consumers::macros::impl_consumer_debug_display;
use crate::consumers::macros::impl_shared_consumer_methods;

// =======================================================================
// 3. RcBiConsumer - Single-Threaded Shared Ownership
// =======================================================================

/// RcBiConsumer struct
///
/// A non-mutating bi-consumer implementation based on `Rc<dyn Fn(&T, &U)>`
/// for single-threaded shared ownership scenarios. The wrapper does not need
/// `RefCell` because it only invokes a shared `Fn`.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot send across threads
/// - **No Wrapper Interior Mutability Overhead**: No RefCell needed by the
///   wrapper
/// - **Non-Consuming API**: `and_then` borrows `&self`, original remains usable
///
/// # Use Cases
///
/// Choose `RcBiConsumer` when:
/// - Need to share non-mutating bi-consumer within a single thread
/// - Observation operations requiring single-threaded shared ownership
/// - Single-threaded UI framework event handling
///
/// # Ownership Cost
///
/// `Rc` uses non-atomic reference counting and is single-threaded; `Arc` uses
/// atomic reference counting and supports thread-safe sharing. Neither wrapper
/// adds a mutex around callback invocation. Choose based on thread-safety
/// requirements and benchmark the real workload when performance matters.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumer, RcBiConsumer};
///
/// let consumer: RcBiConsumer<i32, i32> = RcBiConsumer::new(|x: &i32, y: &i32| {
///     println!("Sum: {}", x + y);
/// });
/// let clone = consumer.clone();
///
/// consumer.accept(&5, &3);
/// clone.accept(&10, &20);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcBiConsumer<T, U> {
    /// The wrapped callback implementation.
    pub(super) function: Rc<BiConsumerFn<T, U>>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, U> RcBiConsumer<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(
        RcBiConsumer<T, U>,
        (Fn(&T, &U) + 'static),
        |f| Rc::new(f)
    );

    // Generates: when() and and_then() methods that borrow &self (Rc can clone)
    impl_shared_consumer_methods!(
        RcBiConsumer<T, U>,
        RcConditionalBiConsumer,
        RcBiPredicate,
        BiConsumer,
        predicate_bounds = ('static),
        chained_bounds = ('static)
    );
}

impl<T, U> BiConsumer<T, U> for RcBiConsumer<T, U> {
    #[inline(always)]
    fn accept(&self, first: &T, second: &U) {
        (self.function)(first, second)
    }
}

// Use macro to generate Clone implementation
impl_consumer_clone!(RcBiConsumer<T, U>);

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(RcBiConsumer<T, U>);
