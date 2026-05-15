/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Shared Conditional Consumer Macro
//!
//! Generates Arc/Rc-based Conditional Consumer implementations
//!
//! For Arc/Rc-based conditional consumers, generates `and_then` and `or_else` methods,
//! as well as complete Consumer/BiConsumer trait implementations.
//!
//! Arc/Rc type characteristics:
//! - `and_then` and `or_else` borrow &self (because Arc/Rc can Clone)
//! - Uses trait default implementations for `into_arc()` and `to_arc()`
//! - Arc types will work with `into_arc()` and `to_arc()` (satisfy Send + Sync constraints)
//! - Rc types will get compile errors if trying to use `into_arc()` or `to_arc()` (don't satisfy Send + Sync)
//! - Implement complete `to_xxx()` methods (because they can Clone)
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$consumer_type` - Consumer wrapper type name
//! * `$consumer_trait` - Consumer trait name
//! * `$predicate_conversion` - Predicate conversion method (into_arc or into_rc)
//! * `$extra_bounds` - Extra trait bounds
//!
//! # Usage Examples
//!
//! ```ignore
//! // Arc single-parameter Consumer
//! impl_shared_conditional_consumer!(
//!     ArcConditionalConsumer<T>,
//!     ArcConsumer,
//!     Consumer,
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc single-parameter Consumer
//! impl_shared_conditional_consumer!(
//!     RcConditionalConsumer<T>,
//!     RcConsumer,
//!     Consumer,
//!     into_rc,
//!     'static
//! );
//!
//! // Arc two-parameter BiConsumer
//! impl_shared_conditional_consumer!(
//!     ArcConditionalBiConsumer<T, U>,
//!     ArcBiConsumer,
//!     BiConsumer,
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc two-parameter BiConsumer
//! impl_shared_conditional_consumer!(
//!     RcConditionalBiConsumer<T, U>,
//!     RcBiConsumer,
//!     BiConsumer,
//!     into_rc,
//!     'static
//! );
//! ```
//!

/// Generates Arc/Rc-based Conditional Consumer implementations
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// For Arc/Rc-based conditional consumers, generates `and_then` and `or_else` methods,
/// as well as complete Consumer/BiConsumer trait implementations.
///
/// Arc/Rc type characteristics:
/// - `and_then` and `or_else` borrow &self (because Arc/Rc can Clone)
/// - Uses trait default implementations for `into_arc()` and `to_arc()`
/// - Arc types will work with `into_arc()` and `to_arc()` (satisfy Send + Sync constraints)
/// - Rc types will get compile errors if trying to use `into_arc()` or `to_arc()` (don't satisfy Send + Sync)
/// - Implement complete `to_xxx()` methods (because they can Clone)
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$consumer_type` - Consumer wrapper type name
/// * `$consumer_trait` - Consumer trait name
/// * `$predicate_conversion` - Predicate conversion method (into_arc or into_rc)
/// * `$extra_bounds` - Extra trait bounds
///
/// # Usage Examples
///
/// ```rust
/// // Arc single-parameter Consumer
/// use std::sync::atomic::{AtomicI32, Ordering};
/// use std::sync::Arc;
/// use qubit_function::{Consumer, ArcConsumer};
///
/// let result = Arc::new(AtomicI32::new(0));
/// let result1 = std::sync::Arc::clone(&result);
/// let consumer1 = ArcConsumer::new(move |x: &i32| {
///     result1.fetch_add(*x, Ordering::SeqCst);
/// });
///
/// let consumer2 = consumer1.when(|x: &i32| *x > 0);
/// let result2 = std::sync::Arc::clone(&result);
/// let chained = consumer2.and_then(ArcConsumer::new(move |x: &i32| {
///     result2.fetch_add(*x * 2, Ordering::SeqCst);
/// }));
///
/// chained.accept(&5);
/// assert_eq!(result.load(Ordering::SeqCst), 15);
/// chained.accept(&-5);
/// assert_eq!(result.load(Ordering::SeqCst), 5);
///
/// // Rc single-parameter Consumer
/// use qubit_function::{RcConsumer};
///
/// let base = RcConsumer::new(|x: &i32| {
///     let _ = x;
/// });
/// let _ = base.when(|x: &i32| *x > 0);
///
/// // Arc two-parameter BiConsumer
/// use qubit_function::{BiConsumer, ArcBiConsumer};
/// let bi_base = ArcBiConsumer::new(|x: &i32, y: &i32| {
///     let _ = (*x, *y);
/// });
/// let bi_conditional = bi_base.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
/// let _ = bi_conditional.and_then(ArcBiConsumer::new(|x: &i32, y: &i32| {
///     let _ = (*x, *y);
/// }));
///
/// // Rc two-parameter BiConsumer
/// use qubit_function::RcBiConsumer;
/// let bi_base_rc = RcBiConsumer::new(|x: &i32, y: &i32| {
///     let _ = (*x, *y);
/// });
/// let bi_conditional_rc = bi_base_rc.when(|x: &i32, y: &i32| *x > 0 || *y > 0);
/// let _ = bi_conditional_rc.and_then(RcBiConsumer::new(|x: &i32, y: &i32| {
///     let _ = (*x, *y);
/// }));
/// ```
///
macro_rules! impl_shared_conditional_consumer {
    (@let_consumer Consumer, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_consumer StatefulConsumer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_consumer BiConsumer, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_consumer StatefulBiConsumer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    // Single generic parameter - Consumer types
    (
        $struct_name:ident < $t:ident >,
        $consumer_type:ident,
        $consumer_trait:ident,
        $predicate_conversion:ident,
        $($extra_bounds:tt)+
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
            /// use qubit_function::{Consumer, ArcConsumer};
            ///
/// let result = std::sync::Arc::new(AtomicI32::new(0));
/// let result1 = std::sync::Arc::clone(&result);
/// let consumer = ArcConsumer::new(move |x: &i32| {
///     result1.fetch_add(*x, Ordering::SeqCst);
/// });
            ///
/// let consumer2 = consumer.when(|x: &i32| *x > 0);
            ///
/// let result2 = std::sync::Arc::clone(&result);
/// let chained = consumer2.and_then(ArcConsumer::new(move |x: &i32| {
///     result2.fetch_add(2 * (*x), Ordering::SeqCst);
/// }));
            ///
            /// chained.accept(&5);  // result = 5 + (2*5) = 15
            /// result.store(0, Ordering::SeqCst);  // reset
            /// chained.accept(&-5); // result = 0 + (2*-5) = -10 (not -15!)
            /// assert_eq!(result.load(Ordering::SeqCst), -10);
            /// ```
            pub fn and_then<C>(&self, next: C) -> $consumer_type<$t>
            where
                $t: 'static,
                C: $consumer_trait<$t> + $($extra_bounds)+,
            {
                let first_predicate = self.predicate.clone();
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, first_consumer, self.consumer.clone());
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, next, next);
                $consumer_type::new(move |t| {
                    if first_predicate.test(t) {
                        first_consumer.accept(t);
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
            pub fn or_else<C>(&self, else_consumer: C) -> $consumer_type<$t>
            where
                $t: 'static,
                C: $consumer_trait<$t> + $($extra_bounds)+,
            {
                let predicate = self.predicate.clone();
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, then_consumer, self.consumer.clone());
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, else_consumer, else_consumer);
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

    // Two generic parameters - BiConsumer types
    (
        $struct_name:ident < $t:ident, $u:ident >,
        $consumer_type:ident,
        $consumer_trait:ident,
        $predicate_conversion:ident,
        $($extra_bounds:tt)+
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
/// use std::sync::Arc;
/// use qubit_function::{BiConsumer, ArcBiConsumer};
///
/// let result = Arc::new(AtomicI32::new(0));
/// let result1 = std::sync::Arc::clone(&result);
/// let consumer = ArcBiConsumer::new(move |x: &i32, y: &i32| {
///     result1.fetch_add(*x + *y, Ordering::SeqCst);
/// });
            ///
/// let consumer2 = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
/// let result2 = std::sync::Arc::clone(&result);
/// let chained = consumer2.and_then(ArcBiConsumer::new(move |x: &i32, y: &i32| {
///     result2.fetch_add(2 * (*x + *y), Ordering::SeqCst);
/// }));
            ///
            /// chained.accept(&5, &3);  // result = (5+3) + 2*(5+3) = 24
            /// result.store(0, Ordering::SeqCst);  // reset
            /// chained.accept(&-5, &3); // result = 0 + 2*(-5+3) = -4 (not -8!)
            /// assert_eq!(result.load(Ordering::SeqCst), -4);
            /// ```
            pub fn and_then<C>(&self, next: C) -> $consumer_type<$t, $u>
            where
                $t: 'static,
                $u: 'static,
                C: $consumer_trait<$t, $u> + $($extra_bounds)+,
            {
                let first_predicate = self.predicate.clone();
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, first_consumer, self.consumer.clone());
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, next, next);
                $consumer_type::new(move |t, u| {
                    if first_predicate.test(t, u) {
                        first_consumer.accept(t, u);
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
            pub fn or_else<C>(&self, else_consumer: C) -> $consumer_type<$t, $u>
            where
                $t: 'static,
                $u: 'static,
                C: $consumer_trait<$t, $u> + $($extra_bounds)+,
            {
                let predicate = self.predicate.clone();
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, then_consumer, self.consumer.clone());
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, else_consumer, else_consumer);
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

pub(crate) use impl_shared_conditional_consumer;
