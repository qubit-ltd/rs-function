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
//! Defines the `RcBiConsumer` public type.

#![allow(unused_imports)]

use super::*;

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
/// - **Non-Consuming API**: `and_then` borrows `&self`, original remains
///   usable
///
/// # Use Cases
///
/// Choose `RcBiConsumer` when:
/// - Need to share non-mutating bi-consumer within a single thread
/// - Pure observation operations, performance critical
/// - Single-threaded UI framework event handling
///
/// # Performance Advantages
///
/// `RcBiConsumer` has neither Arc's atomic operation overhead nor
/// RefCell's runtime borrow checking overhead, making it the best
/// performing among the three non-mutating bi-consumer types.
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
///
pub struct RcBiConsumer<T, U> {
    pub(super) function: Rc<BiConsumerFn<T, U>>,
    pub(super) name: Option<String>,
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
        into_rc,
        BiConsumer,
        'static
    );
}

impl<T, U> BiConsumer<T, U> for RcBiConsumer<T, U> {
    fn accept(&self, first: &T, second: &U) {
        (self.function)(first, second)
    }

    // Use macro to implement conversion methods
    impl_rc_conversions!(
        RcBiConsumer<T, U>,
        BoxBiConsumer,
        BoxBiConsumerOnce,
        Fn(t: &T, u: &U)
    );
}

// Use macro to generate Clone implementation
impl_consumer_clone!(RcBiConsumer<T, U>);

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(RcBiConsumer<T, U>);
