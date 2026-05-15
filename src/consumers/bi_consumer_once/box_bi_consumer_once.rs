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
//! Defines the `BoxBiConsumerOnce` public type.

use super::{
    BiConsumerOnce,
    BiConsumerOnceFn,
    BiPredicate,
    BoxConditionalBiConsumerOnce,
    impl_box_consumer_methods,
    impl_box_once_conversions,
    impl_closure_once_trait,
    impl_consumer_common_methods,
    impl_consumer_debug_display,
};

// =======================================================================
// 2. BoxBiConsumerOnce - Single Ownership Implementation
// =======================================================================

/// BoxBiConsumerOnce struct
///
/// A one-time bi-consumer implementation based on
/// `Box<dyn FnOnce(&T, &U)>` for single ownership scenarios. This is the
/// simplest one-time bi-consumer type for truly one-time use.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, ownership moves on use
/// - **Zero Overhead**: No reference counting or locking
/// - **One-Time Use**: Consumes self on first call
/// - **Builder Pattern**: Method chaining consumes `self` naturally
///
/// # Use Cases
///
/// Choose `BoxBiConsumerOnce` when:
/// - The bi-consumer is truly used only once
/// - Building pipelines where ownership naturally flows
/// - The consumer captures values that should be consumed
/// - Performance is critical and sharing overhead is unacceptable
///
/// # Performance
///
/// `BoxBiConsumerOnce` has the best performance:
/// - No reference counting overhead
/// - No lock acquisition or runtime borrow checking
/// - Direct function call through vtable
/// - Minimal memory footprint (single pointer)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumerOnce, BoxBiConsumerOnce};
///
/// let consumer = BoxBiConsumerOnce::new(|x: &i32, y: &i32| {
///     println!("Sum: {}", x + y);
/// });
/// consumer.accept(&5, &3);
/// ```
///
pub struct BoxBiConsumerOnce<T, U> {
    pub(super) function: Box<BiConsumerOnceFn<T, U>>,
    pub(super) name: Option<String>,
}

// All methods require T: 'static and U: 'static because
// Box<dyn FnOnce(&T, &U)> requires it
impl<T, U> BoxBiConsumerOnce<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(
        BoxBiConsumerOnce<T, U>,
        (FnOnce(&T, &U) + 'static),
        |f| Box::new(f)
    );

    // Generates: when() and and_then() methods that consume self
    impl_box_consumer_methods!(
        BoxBiConsumerOnce<T, U>,
        BoxConditionalBiConsumerOnce,
        BiConsumerOnce
    );
}

impl<T, U> BiConsumerOnce<T, U> for BoxBiConsumerOnce<T, U> {
    fn accept(self, first: &T, second: &U) {
        (self.function)(first, second)
    }

    impl_box_once_conversions!(
        BoxBiConsumerOnce<T, U>,
        BiConsumerOnce,
        FnOnce(&T, &U)
    );
}

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(BoxBiConsumerOnce<T, U>);

// =======================================================================
// 3. Implement BiConsumerOnce trait for closures
// =======================================================================

// Implement BiConsumerOnce for all FnOnce(&T, &U) using macro
impl_closure_once_trait!(
    BiConsumerOnce<T, U>,
    accept,
    BoxBiConsumerOnce,
    FnOnce(first: &T, second: &U)
);
