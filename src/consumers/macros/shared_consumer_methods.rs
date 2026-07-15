// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Shared Consumer Methods Macro
//!
//! Generates `when` and `and_then` for shared Arc/Rc consumers.
//! The generated methods borrow `&self`, clone the shared wrapper, and keep
//! predicate-storage capabilities separate from chained-callback
//! capabilities.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Consumer or BiConsumer wrapper type.
//! * `$return_type` - Conditional wrapper returned by `when`, such as
//!   `ArcConditionalConsumer`.
//! * `$predicate_type` - Predicate wrapper used by the conditional result.
//! * `$consumer_trait` - Semantic trait implemented by the chained callback,
//!   such as `Consumer`.
//! * `$predicate_bounds` - Bounds required to store the predicate wrapper.
//! * `$chained_bounds` - Bounds required to store the callback produced by
//!   `and_then`.
//!
//! # Capability policy
//!
//! | Wrapper family | `predicate_bounds` | `chained_bounds` |
//! |----------------|--------------------|------------------|
//! | Arc stateless | `Send + Sync + 'static` | `Send + Sync + 'static` |
//! | Arc stateful | `Send + Sync + 'static` | `Send + 'static` |
//! | Rc stateless/stateful | `'static` | `'static` |

/// Generates `when` and `and_then` for shared Arc/Rc consumers.
///
/// Invoke this macro inside the target wrapper's `impl` block. Predicate
/// bounds describe the immutable predicate object stored by the conditional
/// wrapper. Chained bounds describe the callback stored by the returned
/// consumer; stateful Arc callbacks are serialized by their outer
/// mutex and therefore require `Send`, but not `Sync`.
///
/// # Capability policy
///
/// | Wrapper family | `predicate_bounds` | `chained_bounds` |
/// |----------------|--------------------|------------------|
/// | Arc stateless | `Send + Sync + 'static` | `Send + Sync + 'static` |
/// | Arc stateful | `Send + Sync + 'static` | `Send + 'static` |
/// | Rc stateless/stateful | `'static` | `'static` |
macro_rules! impl_shared_consumer_methods {
    (@and_then Consumer, $struct_name:ident, $first:expr, $after:expr, $t:ident) => {{
        let first = $first;
        let after = $after;
        $struct_name::new(move |t: &$t| {
            first.accept(t);
            after.accept(t);
        })
    }};

    (@and_then StatefulConsumer, $struct_name:ident, $first:expr, $after:expr, $t:ident) => {{
        let mut first = $first;
        let mut after = $after;
        $struct_name::new(move |t: &$t| {
            first.accept(t);
            after.accept(t);
        })
    }};

    (@and_then_bi BiConsumer, $struct_name:ident, $first:expr, $after:expr, $t:ident, $u:ident) => {{
        let first = $first;
        let after = $after;
        $struct_name::new(move |t: &$t, u: &$u| {
            first.accept(t, u);
            after.accept(t, u);
        })
    }};

    (@and_then_bi StatefulBiConsumer, $struct_name:ident, $first:expr, $after:expr, $t:ident, $u:ident) => {{
        let mut first = $first;
        let mut after = $after;
        $struct_name::new(move |t: &$t, u: &$u| {
            first.accept(t, u);
            after.accept(t, u);
        })
    }};

    // Single generic parameter - Consumer types
    (
        $struct_name:ident < $t:ident >,
        $return_type:ident,
        $predicate_type:ident,
        $consumer_trait:ident,
        predicate_bounds = ($($predicate_bounds:tt)+),
        chained_bounds = ($($chained_bounds:tt)+)
    ) => {
        /// Creates a conditional consumer
        ///
        /// Wraps this consumer with a predicate condition, creating a new
        /// conditional consumer that will only execute the original consumer
        /// when the predicate evaluates to true.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The condition that must be satisfied for the consumer
        ///   to execute
        ///
        /// # Returns
        ///
        /// Returns a conditional consumer that executes this consumer only when
        /// the predicate is satisfied
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{ArcConsumer, Consumer};
        /// let consumer = ArcConsumer::new(|x: &i32| println!("{}", x));
        /// let conditional = consumer.when(|x: &i32| *x > 0);
        ///
        /// conditional.accept(&5);  // prints: 5
        /// conditional.accept(&-5); // prints nothing
        /// ```
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $return_type<$t>
        where
            $t: 'static,
            P: Predicate<$t> + $($predicate_bounds)+,
        {
            $return_type {
                consumer: self.clone(),
                predicate: $crate::$predicate_type::new(predicate),
            }
        }

        /// Chains another consumer in sequence
        ///
        /// Combines this consumer with another consumer into a new consumer
        /// that executes both consumers in sequence. The returned consumer
        /// first executes this consumer, then unconditionally executes the
        /// `after` consumer.
        ///
        /// # Parameters
        ///
        /// * `after` - The consumer to execute after this one (always executed)
        ///
        /// # Returns
        ///
        /// Returns a new consumer that executes both consumers in sequence
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{ArcConsumer, Consumer};
        /// let consumer1 = ArcConsumer::new(|x: &i32| print!("first: {}", x));
        /// let consumer2 = ArcConsumer::new(|x: &i32| println!(" second: {}", x));
        ///
        /// let chained = consumer1.and_then(consumer2);
        ///
        /// chained.accept(&5);  // prints: first: 5 second: 5
        /// ```
        #[inline]
        pub fn and_then<C>(&self, after: C) -> $struct_name<$t>
        where
            $t: 'static,
            C: $consumer_trait<$t> + $($chained_bounds)+,
        {
            impl_shared_consumer_methods!(@and_then $consumer_trait, $struct_name, self.clone(), after, $t)
        }
    };

    // Two generic parameters - BiConsumer types
    (
        $struct_name:ident < $t:ident, $u:ident >,
        $return_type:ident,
        $predicate_type:ident,
        $consumer_trait:ident,
        predicate_bounds = ($($predicate_bounds:tt)+),
        chained_bounds = ($($chained_bounds:tt)+)
    ) => {
        /// Creates a conditional bi-consumer
        ///
        /// Wraps this bi-consumer with a bi-predicate condition, creating a new
        /// conditional bi-consumer that will only execute the original bi-consumer
        /// when the predicate evaluates to true.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The condition that must be satisfied for the bi-consumer
        ///   to execute
        ///
        /// # Returns
        ///
        /// Returns a conditional bi-consumer that executes this bi-consumer only
        /// when the predicate is satisfied
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{ArcBiConsumer, BiConsumer};
        /// let consumer = ArcBiConsumer::new(|x: &i32, y: &i32| println!("{}", x + y));
        /// let conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        ///
        /// conditional.accept(&5, &3);  // prints: 8
        /// conditional.accept(&-5, &3); // prints nothing
        /// ```
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $return_type<$t, $u>
        where
            $t: 'static,
            $u: 'static,
            P: BiPredicate<$t, $u> + $($predicate_bounds)+,
        {
            $return_type {
                consumer: self.clone(),
                predicate: $crate::$predicate_type::new(predicate),
            }
        }

        /// Chains another bi-consumer in sequence
        ///
        /// Combines this bi-consumer with another bi-consumer into a new
        /// bi-consumer that executes both bi-consumers in sequence. The returned
        /// bi-consumer first executes this bi-consumer, then unconditionally
        /// executes the `after` bi-consumer.
        ///
        /// # Parameters
        ///
        /// * `after` - The bi-consumer to execute after this one (always executed)
        ///
        /// # Returns
        ///
        /// Returns a new bi-consumer that executes both bi-consumers in sequence
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{ArcBiConsumer, BiConsumer};
        /// let consumer1 = ArcBiConsumer::new(|x: &i32, y: &i32| print!("first: {}", x + y));
        /// let consumer2 = ArcBiConsumer::new(|x: &i32, y: &i32| println!(" second: {}", x * y));
        ///
        /// let chained = consumer1.and_then(consumer2);
        ///
        /// chained.accept(&5, &3);  // prints: first: 8 second: 15
        /// ```
        #[inline]
        pub fn and_then<C>(&self, after: C) -> $struct_name<$t, $u>
        where
            $t: 'static,
            $u: 'static,
            C: $consumer_trait<$t, $u> + $($chained_bounds)+,
        {
            impl_shared_consumer_methods!(@and_then_bi $consumer_trait, $struct_name, self.clone(), after, $t, $u)
        }
    };
}

pub(crate) use impl_shared_consumer_methods;
