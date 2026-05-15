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
//! Defines the `RcStatefulBiConsumer` public type.

use super::{
    BiPredicate,
    BoxBiConsumerOnce,
    BoxStatefulBiConsumer,
    Rc,
    RcConditionalStatefulBiConsumer,
    RefCell,
    StatefulBiConsumer,
    impl_consumer_clone,
    impl_consumer_common_methods,
    impl_consumer_debug_display,
    impl_rc_conversions,
    impl_shared_consumer_methods,
};

type RcStatefulBiConsumerFn<T, U> = Rc<RefCell<dyn FnMut(&T, &U)>>;

// =======================================================================
// 3. RcStatefulBiConsumer - Single-Threaded Shared Ownership Implementation
// =======================================================================

/// RcStatefulBiConsumer struct
///
/// A bi-consumer implementation based on `Rc<RefCell<dyn FnMut(&T, &U)>>`
/// for single-threaded shared ownership scenarios. This consumer provides
/// the benefits of shared ownership without the overhead of thread
/// safety.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot send across threads
/// - **Interior Mutability**: Uses `RefCell` for runtime borrow checking
/// - **No Lock Overhead**: More efficient than `ArcStatefulBiConsumer` for
///   single-threaded use
/// - **Non-Consuming API**: `and_then` borrows `&self`, original remains
///   usable
///
/// # Use Cases
///
/// Choose `RcStatefulBiConsumer` when:
/// - Need to share bi-consumer within a single thread
/// - Thread safety is not needed
/// - Performance matters (avoiding lock overhead)
/// - Single-threaded UI framework event handling
/// - Building complex single-threaded state machines
///
/// # Performance Considerations
///
/// `RcStatefulBiConsumer` performs better than `ArcStatefulBiConsumer` in single-threaded
/// scenarios:
/// - **Non-Atomic Counting**: clone/drop cheaper than `Arc`
/// - **No Lock Overhead**: `RefCell` uses runtime checking, no locks
/// - **Better Cache Locality**: No atomic operations means better CPU
///   cache behavior
///
/// But still has slight overhead compared to `BoxStatefulBiConsumer`:
/// - **Reference Counting**: Though non-atomic, still exists
/// - **Runtime Borrow Checking**: `RefCell` checks at runtime
///
/// # Safety
///
/// `RcStatefulBiConsumer` is not thread-safe and does not implement `Send` or
/// `Sync`. Attempting to send it to another thread will result in a
/// compile error. For thread-safe sharing, use `ArcStatefulBiConsumer` instead.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumer, RcStatefulBiConsumer, StatefulBiConsumer};
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// let log = Rc::new(RefCell::new(Vec::new()));
/// let l = log.clone();
/// let mut consumer = RcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
///     l.borrow_mut().push(*x + *y);
/// });
/// let mut clone = consumer.clone();
///
/// consumer.accept(&5, &3);
/// assert_eq!(*log.borrow(), vec![8]);
/// ```
///
pub struct RcStatefulBiConsumer<T, U> {
    pub(super) function: RcStatefulBiConsumerFn<T, U>,
    pub(super) name: Option<String>,
}

impl<T, U> RcStatefulBiConsumer<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(
        RcStatefulBiConsumer<T, U>,
        (FnMut(&T, &U) + 'static),
        |f| Rc::new(RefCell::new(f))
    );

    // Generates: when() and and_then() methods that borrow &self (Rc can clone)
    impl_shared_consumer_methods!(
        RcStatefulBiConsumer<T, U>,
        RcConditionalStatefulBiConsumer,
        into_rc,
        StatefulBiConsumer,
        'static
    );
}

impl<T, U> StatefulBiConsumer<T, U> for RcStatefulBiConsumer<T, U> {
    fn accept(&mut self, first: &T, second: &U) {
        (self.function.borrow_mut())(first, second)
    }

    // Use macro to implement conversion methods
    impl_rc_conversions!(
        RcStatefulBiConsumer<T, U>,
        BoxStatefulBiConsumer,
        BoxBiConsumerOnce,
        FnMut(t: &T, u: &U)
    );
}

// Use macro to generate Clone implementation
impl_consumer_clone!(RcStatefulBiConsumer<T, U>);

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(RcStatefulBiConsumer<T, U>);
