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
    (@state_init Consumer, $state:literal) => { "" };
    (@state_init StatefulConsumer, $state:literal) => {
        concat!("let mut ", $state, " = 0;\n")
    };
    (@state_init BiConsumer, $state:literal) => { "" };
    (@state_init StatefulBiConsumer, $state:literal) => {
        concat!("let mut ", $state, " = 0;\n")
    };
    (@callback Consumer, $state:literal, $observed:literal, $args:literal) => {
        concat!("move |", $args, "| { ", $observed,
            ".fetch_add(1, Ordering::SeqCst); }")
    };
    (@callback StatefulConsumer, $state:literal, $observed:literal, $args:literal) => {
        concat!("move |", $args, "| { ", $state, " += 1; ", $observed,
            ".store(", $state, ", Ordering::SeqCst); }")
    };
    (@callback BiConsumer, $state:literal, $observed:literal, $args:literal) => {
        concat!("move |", $args, "| { ", $observed,
            ".fetch_add(1, Ordering::SeqCst); }")
    };
    (@callback StatefulBiConsumer, $state:literal, $observed:literal, $args:literal) => {
        concat!("move |", $args, "| { ", $state, " += 1; ", $observed,
            ".store(", $state, ", Ordering::SeqCst); }")
    };
    (@mut Consumer) => { "" };
    (@mut StatefulConsumer) => { "mut " };
    (@mut BiConsumer) => { "" };
    (@mut StatefulBiConsumer) => { "mut " };

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
        /// # Type Parameters
        ///
        /// * `P` - Predicate that decides whether the wrapped callback runs.
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
        #[doc = concat!(
            "```rust\n",
            "use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};\n",
            "use qubit_function::{", stringify!($struct_name), ", ", stringify!($consumer_trait), "};\n",
            impl_shared_consumer_methods!(@state_init $consumer_trait, "callback_calls"),
            "let calls = Arc::new(AtomicUsize::new(0));\n",
            "let observed = Arc::clone(&calls);\n",
            "let consumer = ", stringify!($struct_name), "::new(",
            impl_shared_consumer_methods!(@callback $consumer_trait, "callback_calls", "observed", "_: &i32"), ");\n",
            "let ", impl_shared_consumer_methods!(@mut $consumer_trait), "conditional = consumer.when(|x: &i32| *x > 0);\n",
            "conditional.accept(&-5);\n",
            "assert_eq!(calls.load(Ordering::SeqCst), 0);\n",
            "conditional.accept(&5);\n",
            "assert_eq!(calls.load(Ordering::SeqCst), 1);\n",
            "```"
        )]
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $return_type<$t>
        where
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
        /// # Type Parameters
        ///
        /// * `C` - Callback implementation chained after this one.
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
        #[doc = concat!(
            "```rust\n",
            "use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};\n",
            "use qubit_function::{", stringify!($struct_name), ", ", stringify!($consumer_trait), "};\n",
            impl_shared_consumer_methods!(@state_init $consumer_trait, "first_state"),
            impl_shared_consumer_methods!(@state_init $consumer_trait, "after_state"),
            "let first_calls = Arc::new(AtomicUsize::new(0));\n",
            "let first_observed = Arc::clone(&first_calls);\n",
            "let first = ", stringify!($struct_name), "::new(",
            impl_shared_consumer_methods!(@callback $consumer_trait, "first_state", "first_observed", "_: &i32"), ");\n",
            "let after_calls = Arc::new(AtomicUsize::new(0));\n",
            "let after_observed = Arc::clone(&after_calls);\n",
            "let after = ", stringify!($struct_name), "::new(",
            impl_shared_consumer_methods!(@callback $consumer_trait, "after_state", "after_observed", "_: &i32"), ");\n",
            "let ", impl_shared_consumer_methods!(@mut $consumer_trait), "chained = first.and_then(after);\n",
            "chained.accept(&5);\n",
            "assert_eq!(first_calls.load(Ordering::SeqCst), 1);\n",
            "assert_eq!(after_calls.load(Ordering::SeqCst), 1);\n",
            "```"
        )]
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
        /// # Type Parameters
        ///
        /// * `P` - Predicate that decides whether the wrapped callback runs.
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
        #[doc = concat!(
            "```rust\n",
            "use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};\n",
            "use qubit_function::{", stringify!($struct_name), ", ", stringify!($consumer_trait), "};\n",
            impl_shared_consumer_methods!(@state_init $consumer_trait, "callback_calls"),
            "let calls = Arc::new(AtomicUsize::new(0));\n",
            "let observed = Arc::clone(&calls);\n",
            "let consumer = ", stringify!($struct_name), "::new(",
            impl_shared_consumer_methods!(@callback $consumer_trait, "callback_calls", "observed", "_: &i32, _: &i32"), ");\n",
            "let ", impl_shared_consumer_methods!(@mut $consumer_trait), "conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);\n",
            "conditional.accept(&-5, &3);\n",
            "assert_eq!(calls.load(Ordering::SeqCst), 0);\n",
            "conditional.accept(&5, &3);\n",
            "assert_eq!(calls.load(Ordering::SeqCst), 1);\n",
            "```"
        )]
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $return_type<$t, $u>
        where
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
        /// # Type Parameters
        ///
        /// * `C` - Callback implementation chained after this one.
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
        #[doc = concat!(
            "```rust\n",
            "use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};\n",
            "use qubit_function::{", stringify!($struct_name), ", ", stringify!($consumer_trait), "};\n",
            impl_shared_consumer_methods!(@state_init $consumer_trait, "first_state"),
            impl_shared_consumer_methods!(@state_init $consumer_trait, "after_state"),
            "let first_calls = Arc::new(AtomicUsize::new(0));\n",
            "let first_observed = Arc::clone(&first_calls);\n",
            "let first = ", stringify!($struct_name), "::new(",
            impl_shared_consumer_methods!(@callback $consumer_trait, "first_state", "first_observed", "_: &i32, _: &i32"), ");\n",
            "let after_calls = Arc::new(AtomicUsize::new(0));\n",
            "let after_observed = Arc::clone(&after_calls);\n",
            "let after = ", stringify!($struct_name), "::new(",
            impl_shared_consumer_methods!(@callback $consumer_trait, "after_state", "after_observed", "_: &i32, _: &i32"), ");\n",
            "let ", impl_shared_consumer_methods!(@mut $consumer_trait), "chained = first.and_then(after);\n",
            "chained.accept(&5, &3);\n",
            "assert_eq!(first_calls.load(Ordering::SeqCst), 1);\n",
            "assert_eq!(after_calls.load(Ordering::SeqCst), 1);\n",
            "```"
        )]
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
