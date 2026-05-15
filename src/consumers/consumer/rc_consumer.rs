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
//! Defines the `RcConsumer` public type.

use super::{
    BoxConsumer,
    BoxConsumerOnce,
    Consumer,
    Predicate,
    Rc,
    RcConditionalConsumer,
    impl_consumer_clone,
    impl_consumer_common_methods,
    impl_consumer_debug_display,
    impl_rc_conversions,
    impl_shared_consumer_methods,
};

// ============================================================================
// 3. RcConsumer - Single-threaded Shared Ownership Implementation
// ============================================================================

/// RcConsumer struct
///
/// Non-mutating consumer implementation based on `Rc<dyn Fn(&T)>` for
/// single-threaded shared ownership scenarios. The wrapper does not need
/// `RefCell` because it only invokes a shared `Fn`.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable through `Rc`, allows multiple owners
/// - **Single-threaded**: Not thread-safe, cannot be sent across threads
/// - **No Wrapper Interior Mutability Overhead**: No RefCell needed by the
///   wrapper
/// - **Non-consuming API**: `and_then` borrows `&self`, original object remains
///   usable
///
/// # Use Cases
///
/// Choose `RcConsumer` when:
/// - Need to share non-mutating consumer within a single thread
/// - Pure observation operations, performance critical
/// - Event handling in single-threaded UI frameworks
///
/// # Performance Advantages
///
/// `RcConsumer` has neither Arc's atomic operation overhead nor
/// RefCell's runtime borrow checking overhead, making it the most performant of
/// the three non-mutating consumers.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Consumer, RcConsumer};
///
/// let consumer = RcConsumer::new(|x: &i32| {
///     println!("Observed: {}", x);
/// });
/// let clone = consumer.clone();
///
/// consumer.accept(&5);
/// clone.accept(&10);
/// ```
///
pub struct RcConsumer<T> {
    pub(super) function: Rc<dyn Fn(&T)>,
    pub(super) name: Option<String>,
}

impl<T> RcConsumer<T> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(RcConsumer<T>, (Fn(&T) + 'static), |f| Rc::new(f));

    // Generates: when() and and_then() methods that borrow &self (Rc can clone)
    impl_shared_consumer_methods!(
        RcConsumer<T>,
        RcConditionalConsumer,
        into_rc,
        Consumer,
        'static
    );
}

impl<T> Consumer<T> for RcConsumer<T> {
    fn accept(&self, value: &T) {
        (self.function)(value)
    }

    // Use macro to implement conversion methods
    impl_rc_conversions!(
        RcConsumer<T>,
        BoxConsumer,
        BoxConsumerOnce,
        Fn(t: &T)
    );
}

// Use macro to generate Clone implementation
impl_consumer_clone!(RcConsumer<T>);

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(RcConsumer<T>);
