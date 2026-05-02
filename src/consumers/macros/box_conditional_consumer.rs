/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Box Conditional Consumer Macro
//!
//! Generates Box-based Conditional Consumer implementations
//!
//! For Box-based conditional consumers, generates `and_then` and `or_else` methods,
//! as well as complete Consumer/BiConsumer trait implementations.
//!
//! Box type characteristics:
//! - `and_then` and `or_else` consume self (because Box cannot Clone)
//! - Does not implement `into_arc()` (because Box types are not Send + Sync)
//! - Does not implement `to_xxx()` methods (because Box types cannot Clone)
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$consumer_type` - Consumer wrapper type name
//! * `$consumer_trait` - Consumer trait name
//!
//! # Usage Examples
//!
//! ```rust
//! macro_rules! impl_box_conditional_consumer {
//!     ($struct_name:ty, $consumer_type:ty, $consumer_trait:path) => {
//!         let _ = std::mem::size_of::<$struct_name>();
//!         let _ = std::mem::size_of::<$consumer_type>();
//!         let _ = std::stringify!($consumer_trait);
//!     };
//! }
//!
//! // Single-parameter Consumer
//! impl_box_conditional_consumer!(i32, i32, i32);
//!
//! // Two-parameter BiConsumer
//! impl_box_conditional_consumer!(i32, i32, i32);
//! ```
//!

/// Generates Box-based Conditional Consumer implementations
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// For Box-based conditional consumers, generates `and_then` and `or_else` methods,
/// as well as complete Consumer/BiConsumer trait implementations.
///
/// Box type characteristics:
/// - `and_then` and `or_else` consume self (because Box cannot Clone)
/// - Does not implement `into_arc()` (because Box types are not Send + Sync)
/// - Does not implement `to_xxx()` methods (because Box types cannot Clone)
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$consumer_type` - Consumer wrapper type name
/// * `$consumer_trait` - Consumer trait name
///
/// # Usage Examples
///
/// ```rust
/// macro_rules! impl_box_conditional_consumer {
///     ($struct_name:ty, $consumer_type:ty, $consumer_trait:path) => {
///         let _ = std::mem::size_of::<$struct_name>();
///         let _ = std::mem::size_of::<$consumer_type>();
///         let _ = std::stringify!($consumer_trait);
///     };
/// }
/// // Single-parameter Consumer
/// impl_box_conditional_consumer!(i32, i32, i32);
///
/// // Two-parameter BiConsumer
/// impl_box_conditional_consumer!(i32, i32, i32);
/// ```
///
macro_rules! impl_box_conditional_consumer {
    // Single generic parameter - Consumer
    (
        $struct_name:ident<$t:ident>,
        $consumer_type:ident,
        $consumer_trait:ident
    ) => {
        impl<$t> $struct_name<$t> {
            /// Chains another consumer in sequence
            ///
            /// Combines the current conditional consumer with another consumer
            /// into a new consumer that implements the following semantics:
            ///
            /// When the returned consumer is called with an argument:
            /// 1. First, it checks the predicate of this conditional consumer
            /// 2. If the predicate is satisfied, it executes the internal
            ///    consumer of this conditional consumer
            /// 3. Then, **regardless of whether the predicate was satisfied**,
            ///    it unconditionally executes the `next` consumer
            ///
            /// In other words, this creates a consumer that conditionally
            /// executes the first action (based on the predicate), and then
            /// always executes the second action.
            ///
            /// # Parameters
            ///
            /// * `next` - The next consumer to execute (always executed)
            ///
            /// # Returns
            ///
            /// Returns a new combined consumer
            ///
            /// # Examples
            ///
            /// ```rust
            /// use std::sync::atomic::{AtomicI32, Ordering};
            /// use std::sync::Arc;
            /// use qubit_function::BoxConsumer;
            /// use qubit_function::Consumer;
            ///
            /// let result = Arc::new(AtomicI32::new(0));
            /// let result1 = result.clone();
            /// let result2 = result.clone();
            ///
            /// let consumer1 = BoxConsumer::new(move |x: &i32| {
            ///     result1.fetch_add(*x, Ordering::SeqCst);
            /// });
            ///
            /// let consumer2 = BoxConsumer::new(move |x: &i32| {
            ///     result2.fetch_add(2 * (*x), Ordering::SeqCst);
            /// });
            /// let result3 = result.clone();
            ///
            /// let conditional = consumer1.when(|x: &i32| *x > 0);
            /// let chained = conditional.and_then(consumer2);
            ///
            /// chained.accept(&5);  // result = 5 + (2*5) = 15
            /// result3.store(0, Ordering::SeqCst);  // reset
            /// chained.accept(&-5); // result = 0 + (2*-5) = -10 (not -15!)
            /// ```
            #[allow(unused_mut)]
            pub fn and_then<C>(self, mut next: C) -> $consumer_type<$t>
            where
                $t: 'static,
                C: $consumer_trait<$t> + 'static,
            {
                let predicate = self.predicate;
                let mut consumer = self.consumer;
                $consumer_type::new(move |t| {
                    if predicate.test(t) {
                        consumer.accept(t);
                    }
                    next.accept(t);
                })
            }

            /// Adds an else branch
            ///
            /// Executes the original consumer when the condition is satisfied, otherwise
            /// executes else_consumer.
            ///
            /// # Parameters
            ///
            /// * `else_consumer` - The consumer for the else branch
            ///
            /// # Returns
            ///
            /// Returns a new consumer with if-then-else logic
            #[allow(unused_mut)]
            pub fn or_else<C>(self, mut else_consumer: C) -> $consumer_type<$t>
            where
                $t: 'static,
                C: $consumer_trait<$t> + 'static,
            {
                let predicate = self.predicate;
                let mut then_consumer = self.consumer;
                $consumer_type::new(move |t| {
                    if predicate.test(t) {
                        then_consumer.accept(t);
                    } else {
                        else_consumer.accept(t);
                    }
                })
            }
        }
    };

    // Two generic parameters - BiConsumer
    (
        $struct_name:ident<$t:ident, $u:ident>,
        $consumer_type:ident,
        $consumer_trait:ident
    ) => {
        impl<$t, $u> $struct_name<$t, $u> {
            /// Chains another bi-consumer in sequence
            ///
            /// Combines the current conditional bi-consumer with another
            /// bi-consumer into a new bi-consumer that implements the
            /// following semantics:
            ///
            /// When the returned bi-consumer is called with two arguments:
            /// 1. First, it checks the predicate of this conditional
            ///    bi-consumer
            /// 2. If the predicate is satisfied, it executes the internal
            ///    bi-consumer of this conditional bi-consumer
            /// 3. Then, **regardless of whether the predicate was
            ///    satisfied**, it unconditionally executes the `next`
            ///    bi-consumer
            ///
            /// In other words, this creates a bi-consumer that conditionally
            /// executes the first action (based on the predicate), and then
            /// always executes the second action.
            ///
            /// # Parameters
            ///
            /// * `next` - The next bi-consumer to execute (always executed)
            ///
            /// # Returns
            ///
            /// Returns a new combined bi-consumer
            ///
            /// # Examples
            ///
            /// ```rust
            /// use std::sync::atomic::{AtomicI32, Ordering};
            /// use qubit_function::BoxBiConsumer;
            /// use qubit_function::BiConsumer;
            /// use std::sync::Arc;
            ///
            /// let result = Arc::new(AtomicI32::new(0));
            /// let result1 = result.clone();
            /// let result2 = result.clone();
            ///
            /// let consumer1 = BoxBiConsumer::new(move |x: &i32, y: &i32| {
            ///     result1.fetch_add(x + y, Ordering::SeqCst);
            /// });
            ///
            /// let consumer2 = BoxBiConsumer::new(move |x: &i32, y: &i32| {
            ///     result2.fetch_add(2 * (x + y), Ordering::SeqCst);
            /// });
            ///
            /// let conditional = consumer1.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
            /// let chained = conditional.and_then(consumer2);
            ///
            /// chained.accept(&5, &3);  // result = (5+3) + 2*(5+3) = 24
            /// let result3 = result.clone();
            /// result3.store(0, Ordering::SeqCst);  // reset
            /// chained.accept(&-5, &3); // result = 0 + 2*(-5+3) = -4 (not -8!)
            /// ```
            #[allow(unused_mut)]
            pub fn and_then<C>(self, mut next: C) -> $consumer_type<$t, $u>
            where
                $t: 'static,
                $u: 'static,
                C: $consumer_trait<$t, $u> + 'static,
            {
                let predicate = self.predicate;
                let mut consumer = self.consumer;
                $consumer_type::new(move |t, u| {
                    if predicate.test(t, u) {
                        consumer.accept(t, u);
                    }
                    next.accept(t, u);
                })
            }

            /// Adds an else branch
            ///
            /// Executes the original bi-consumer when the condition is satisfied, otherwise
            /// executes else_consumer.
            ///
            /// # Parameters
            ///
            /// * `else_consumer` - The bi-consumer for the else branch
            ///
            /// # Returns
            ///
            /// Returns a new bi-consumer with if-then-else logic
            #[allow(unused_mut)]
            pub fn or_else<C>(self, mut else_consumer: C) -> $consumer_type<$t, $u>
            where
                $t: 'static,
                $u: 'static,
                C: $consumer_trait<$t, $u> + 'static,
            {
                let predicate = self.predicate;
                let mut then_consumer = self.consumer;
                $consumer_type::new(move |t, u| {
                    if predicate.test(t, u) {
                        then_consumer.accept(t, u);
                    } else {
                        else_consumer.accept(t, u);
                    }
                })
            }
        }
    };
}

pub(crate) use impl_box_conditional_consumer;
