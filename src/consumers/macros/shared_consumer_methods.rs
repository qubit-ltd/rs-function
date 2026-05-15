/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Shared Consumer Methods Macro
//!
//! Generates when and and_then method implementations for Arc/Rc-based Consumer
//!
//! Generates conditional execution when method and chaining and_then method
//! for Arc/Rc-based consumers that borrow &self (because Arc/Rc can be cloned).
//!
//! This macro supports both single-parameter and two-parameter consumers through
//! pattern matching on the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `ArcConsumer<T>`
//!   - Two parameters: `ArcBiConsumer<T, U>`
//! * `$return_type` - The return type for when (e.g., ArcConditionalConsumer)
//! * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
//! * `$consumer_trait` - Consumer trait name (e.g., Consumer, BiConsumer)
//! * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
//!
//! # All Macro Invocations
//!
//! | Consumer Type | Struct Signature | `$return_type` | `$predicate_conversion` | `$consumer_trait` | `$extra_bounds` |
//! |---------------|-----------------|----------------|------------------------|------------------|----------------|
//! | **ArcConsumer** | `ArcConsumer<T>` | ArcConditionalConsumer | into_arc | Consumer | Send + Sync + 'static |
//! | **RcConsumer** | `RcConsumer<T>` | RcConditionalConsumer | into_rc | Consumer | 'static |
//! | **ArcStatefulConsumer** | `ArcStatefulConsumer<T>` | ArcConditionalStatefulConsumer | into_arc | StatefulConsumer | Send + Sync + 'static |
//! | **RcStatefulConsumer** | `RcStatefulConsumer<T>` | RcConditionalStatefulConsumer | into_rc | StatefulConsumer | 'static |
//! | **ArcBiConsumer** | `ArcBiConsumer<T, U>` | ArcConditionalBiConsumer | into_arc | BiConsumer | Send + Sync + 'static |
//! | **RcBiConsumer** | `RcBiConsumer<T, U>` | RcConditionalBiConsumer | into_rc | BiConsumer | 'static |
//! | **ArcStatefulBiConsumer** | `ArcStatefulBiConsumer<T, U>` | ArcConditionalStatefulBiConsumer | into_arc | StatefulBiConsumer | Send + Sync + 'static |
//! | **RcStatefulBiConsumer** | `RcStatefulBiConsumer<T, U>` | RcConditionalStatefulBiConsumer | into_rc | StatefulBiConsumer | 'static |
//!
//! # Examples
//!
//! ```ignore
//! // Single-parameter with Arc
//! impl_shared_consumer_methods!(
//!     ArcConsumer<T>,
//!     ArcConditionalConsumer,
//!     into_arc,
//!     Consumer,
//!     Send + Sync + 'static
//! );
//!
//! // Two-parameter with Rc
//! impl_shared_consumer_methods!(
//!     RcBiConsumer<T, U>,
//!     RcConditionalBiConsumer,
//!     into_rc,
//!     BiConsumer,
//!     'static
//! );
//! ```
//!

/// Generates when and and_then method implementations for Arc/Rc-based Consumer
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates conditional execution when method and chaining
/// and_then method for Arc/Rc-based consumers that borrow &self (because Arc/Rc
/// can be cloned).
///
/// This macro supports both single-parameter and two-parameter consumers through
/// pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `ArcConsumer<T>`
///   - Two parameters: `ArcBiConsumer<T, U>`
/// * `$return_type` - The return type for when (e.g., ArcConditionalConsumer)
/// * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
/// * `$consumer_trait` - Consumer trait name (e.g., Consumer, BiConsumer)
/// * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
///
/// # All Macro Invocations
///
/// | Consumer Type | Struct Signature | `$return_type` | `$predicate_conversion` | `$consumer_trait` | `$extra_bounds` |
/// |---------------|-----------------|----------------|------------------------|------------------|----------------|
/// | **ArcConsumer** | `ArcConsumer<T>` | ArcConditionalConsumer | into_arc | Consumer | Send + Sync + 'static |
/// | **RcConsumer** | `RcConsumer<T>` | RcConditionalConsumer | into_rc | Consumer | 'static |
/// | **ArcStatefulConsumer** | `ArcStatefulConsumer<T>` | ArcConditionalStatefulConsumer | into_arc | StatefulConsumer | Send + Sync + 'static |
/// | **RcStatefulConsumer** | `RcStatefulConsumer<T>` | RcConditionalStatefulConsumer | into_rc | StatefulConsumer | 'static |
/// | **ArcBiConsumer** | `ArcBiConsumer<T, U>` | ArcConditionalBiConsumer | into_arc | BiConsumer | Send + Sync + 'static |
/// | **RcBiConsumer** | `RcBiConsumer<T, U>` | RcConditionalBiConsumer | into_rc | BiConsumer | 'static |
/// | **ArcStatefulBiConsumer** | `ArcStatefulBiConsumer<T, U>` | ArcConditionalStatefulBiConsumer | into_arc | StatefulBiConsumer | Send + Sync + 'static |
/// | **RcStatefulBiConsumer** | `RcStatefulBiConsumer<T, U>` | RcConditionalStatefulBiConsumer | into_rc | StatefulBiConsumer | 'static |
///
/// # Examples
///
/// ```rust
/// // Single-parameter with Arc
/// use qubit_function::{ArcConsumer, RcConsumer};
///
/// let _arc_conditional = ArcConsumer::new(|x: &i32| {
///     let _ = x;
/// }).when(|x: &i32| *x > 0);
/// let _rc_conditional = RcConsumer::new(|x: &i32| {
///     let _ = x;
/// }).when(|x: &i32| *x > 0);
///
/// // Two-parameter with Rc
/// use qubit_function::{RcBiConsumer};
/// let _rc_bi_conditional = RcBiConsumer::new(|x: &i32, y: &i32| {
///     let _ = (*x, *y);
/// }).when(|x: &i32, y: &i32| *x > 0 && *y > 0);
/// ```
///
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
        $predicate_conversion:ident,
        $consumer_trait:ident,
        $($extra_bounds:tt)+
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
            P: Predicate<$t> + $($extra_bounds)+,
        {
            $return_type {
                consumer: self.clone(),
                predicate: predicate.$predicate_conversion(),
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
            C: $consumer_trait<$t> + $($extra_bounds)+,
        {
            impl_shared_consumer_methods!(@and_then $consumer_trait, $struct_name, self.clone(), after, $t)
        }
    };

    // Two generic parameters - BiConsumer types
    (
        $struct_name:ident < $t:ident, $u:ident >,
        $return_type:ident,
        $predicate_conversion:ident,
        $consumer_trait:ident,
        $($extra_bounds:tt)+
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
            P: BiPredicate<$t, $u> + $($extra_bounds)+,
        {
            $return_type {
                consumer: self.clone(),
                predicate: predicate.$predicate_conversion(),
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
            C: $consumer_trait<$t, $u> + $($extra_bounds)+,
        {
            impl_shared_consumer_methods!(@and_then_bi $consumer_trait, $struct_name, self.clone(), after, $t, $u)
        }
    };
}

pub(crate) use impl_shared_consumer_methods;
