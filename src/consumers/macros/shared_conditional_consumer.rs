// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Shared Conditional Consumer Macro
//!
//! Generates `and_then` and `or_else` for shared Arc/Rc conditional consumers.
//! Generated methods borrow `&self` and return a wrapper from the same
//! ownership and statefulness family.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Conditional wrapper type.
//! * `$consumer_type` - Result wrapper type, such as `ArcConsumer`.
//! * `$consumer_trait` - Semantic trait accepted for the additional callback,
//!   such as `Consumer`.
//! * `$callback_bounds` - Storage capabilities required for the additional
//!   callback.
//!
//! # Capability policy
//!
//! | Wrapper family | `callback_bounds` |
//! |----------------|-------------------|
//! | Arc stateless | `Send + Sync + 'static` |
//! | Arc stateful | `Send + 'static` |
//! | Rc stateless/stateful | `'static` |

/// Generates `and_then` and `or_else` for shared Arc/Rc conditional consumers.
///
/// Invoke this macro at module scope. The selected callback bounds reflect
/// how the result wrapper stores its callback: stateless Arc wrappers call
/// through a shared reference and require `Sync`; stateful Arc wrappers call
/// under an outer mutex and require only `Send`.
///
/// # Parameters
///
/// * `$struct_name` - Conditional wrapper type receiving the generated methods.
/// * `$consumer_type` - Shared consumer type returned by the generated methods.
/// * `$consumer_trait` - Consumer trait implemented by the callback arguments.
/// * `callback_bounds` - Additional bounds required by the callback storage.
///
/// # Capability policy
///
/// | Wrapper family | `callback_bounds` |
/// |----------------|-------------------|
/// | Arc stateless | `Send + Sync + 'static` |
/// | Arc stateful | `Send + 'static` |
/// | Rc stateless/stateful | `'static` |
macro_rules! impl_shared_conditional_consumer {
    (@doc_state Consumer, $state:literal) => { "" };
    (@doc_state BiConsumer, $state:literal) => { "" };
    (@doc_state StatefulConsumer, $state:literal) => { concat!("let mut ", $state, " = 0;\n") };
    (@doc_state StatefulBiConsumer, $state:literal) => { concat!("let mut ", $state, " = 0;\n") };
    (@doc_mut Consumer) => { "" };
    (@doc_mut BiConsumer) => { "" };
    (@doc_mut StatefulConsumer) => { "mut " };
    (@doc_mut StatefulBiConsumer) => { "mut " };
    (@doc_callback Consumer, $args:literal, $state:literal, $observed:literal) => { concat!("move |", $args, "| { ", $observed, ".fetch_add(1, Ordering::SeqCst); }") };
    (@doc_callback BiConsumer, $args:literal, $state:literal, $observed:literal) => { concat!("move |", $args, "| { ", $observed, ".fetch_add(1, Ordering::SeqCst); }") };
    (@doc_callback StatefulConsumer, $args:literal, $state:literal, $observed:literal) => { concat!("move |", $args, "| { ", $state, " += 1; ", $observed, ".store(", $state, ", Ordering::SeqCst); }") };
    (@doc_callback StatefulBiConsumer, $args:literal, $state:literal, $observed:literal) => { concat!("move |", $args, "| { ", $state, " += 1; ", $observed, ".store(", $state, ", Ordering::SeqCst); }") };

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
        callback_bounds = ($($callback_bounds:tt)+)
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
            /// # Type Parameters
            ///
            /// * `C` - Type of the next consumer.
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
            #[doc = concat!(
                "```rust\n",
                "use std::sync::Arc;\n",
                "use std::sync::atomic::AtomicUsize;\n",
                "use std::sync::atomic::Ordering;\n",
                "use qubit_function::{", stringify!($consumer_type), ", ", stringify!($consumer_trait), "};\n",
                impl_shared_conditional_consumer!(@doc_state $consumer_trait, "first_state"),
                impl_shared_conditional_consumer!(@doc_state $consumer_trait, "branch_state"),
                "let first_calls = Arc::new(AtomicUsize::new(0));\n",
                "let first_observed = Arc::clone(&first_calls);\n",
                "let first = ", stringify!($consumer_type), "::new(", impl_shared_conditional_consumer!(@doc_callback $consumer_trait, "_: &i32", "first_state", "first_observed"), ");\n",
                "let conditional = first.when(|x: &i32| *x > 0);\n",
                "let branch_calls = Arc::new(AtomicUsize::new(0));\n",
                "let branch_observed = Arc::clone(&branch_calls);\n",
                "let branch = ", stringify!($consumer_type), "::new(", impl_shared_conditional_consumer!(@doc_callback $consumer_trait, "_: &i32", "branch_state", "branch_observed"), ");\n",
                "let ", impl_shared_conditional_consumer!(@doc_mut $consumer_trait), "combined = conditional.and_then(branch);\n",
                "combined.accept(&-1);\n",
                "assert_eq!(first_calls.load(Ordering::SeqCst), 0);\n",
                "assert_eq!(branch_calls.load(Ordering::SeqCst), 1);\n",
                "```"
            )]
            pub fn and_then<C>(&self, next: C) -> $consumer_type<$t>
            where
                $t: 'static,
                C: $consumer_trait<$t> + $($callback_bounds)+,
            {
                let first_predicate = self.predicate.clone();
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, first_consumer, self.consumer.clone());
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, next, next);
                $consumer_type::new(move |t: &$t| {
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
            /// # Type Parameters
            ///
            /// * `C` - Type of the else-branch consumer.
            ///
            /// # Parameters
            ///
            /// * `else_consumer` - The consumer for the else branch
            ///
            /// # Returns
            ///
            /// Returns a new consumer with if-then-else logic
            ///
            /// # Examples
            ///
            #[doc = concat!(
                "```rust\n",
                "use std::sync::Arc;\n",
                "use std::sync::atomic::AtomicUsize;\n",
                "use std::sync::atomic::Ordering;\n",
                "use qubit_function::{", stringify!($consumer_type), ", ", stringify!($consumer_trait), "};\n",
                impl_shared_conditional_consumer!(@doc_state $consumer_trait, "first_state"),
                impl_shared_conditional_consumer!(@doc_state $consumer_trait, "else_state"),
                "let first_calls = Arc::new(AtomicUsize::new(0));\n",
                "let first_observed = Arc::clone(&first_calls);\n",
                "let first = ", stringify!($consumer_type), "::new(", impl_shared_conditional_consumer!(@doc_callback $consumer_trait, "_: &i32", "first_state", "first_observed"), ");\n",
                "let conditional = first.when(|x: &i32| *x > 0);\n",
                "let else_calls = Arc::new(AtomicUsize::new(0));\n",
                "let else_observed = Arc::clone(&else_calls);\n",
                "let else_consumer = ", stringify!($consumer_type), "::new(", impl_shared_conditional_consumer!(@doc_callback $consumer_trait, "_: &i32", "else_state", "else_observed"), ");\n",
                "let ", impl_shared_conditional_consumer!(@doc_mut $consumer_trait), "combined = conditional.or_else(else_consumer);\n",
                "combined.accept(&1);\n",
                "assert_eq!(first_calls.load(Ordering::SeqCst), 1);\n",
                "assert_eq!(else_calls.load(Ordering::SeqCst), 0);\n",
                "```"
            )]
            pub fn or_else<C>(&self, else_consumer: C) -> $consumer_type<$t>
            where
                $t: 'static,
                C: $consumer_trait<$t> + $($callback_bounds)+,
            {
                let predicate = self.predicate.clone();
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, then_consumer, self.consumer.clone());
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, else_consumer, else_consumer);
                $consumer_type::new(move |t: &$t| {
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
        callback_bounds = ($($callback_bounds:tt)+)
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
            /// # Type Parameters
            ///
            /// * `C` - Type of the next bi-consumer.
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
            #[doc = concat!(
                "```rust\n",
                "use std::sync::Arc;\n",
                "use std::sync::atomic::AtomicUsize;\n",
                "use std::sync::atomic::Ordering;\n",
                "use qubit_function::{", stringify!($consumer_type), ", ", stringify!($consumer_trait), "};\n",
                impl_shared_conditional_consumer!(@doc_state $consumer_trait, "first_state"),
                impl_shared_conditional_consumer!(@doc_state $consumer_trait, "branch_state"),
                "let first_calls = Arc::new(AtomicUsize::new(0));\n",
                "let first_observed = Arc::clone(&first_calls);\n",
                "let first = ", stringify!($consumer_type), "::new(", impl_shared_conditional_consumer!(@doc_callback $consumer_trait, "_: &i32, _: &i32", "first_state", "first_observed"), ");\n",
                "let conditional = first.when(|x: &i32, y: &i32| *x > 0 && *y > 0);\n",
                "let branch_calls = Arc::new(AtomicUsize::new(0));\n",
                "let branch_observed = Arc::clone(&branch_calls);\n",
                "let branch = ", stringify!($consumer_type), "::new(", impl_shared_conditional_consumer!(@doc_callback $consumer_trait, "_: &i32, _: &i32", "branch_state", "branch_observed"), ");\n",
                "let ", impl_shared_conditional_consumer!(@doc_mut $consumer_trait), "combined = conditional.and_then(branch);\n",
                "combined.accept(&-1, &2);\n",
                "assert_eq!(first_calls.load(Ordering::SeqCst), 0);\n",
                "assert_eq!(branch_calls.load(Ordering::SeqCst), 1);\n",
                "```"
            )]
            pub fn and_then<C>(&self, next: C) -> $consumer_type<$t, $u>
            where
                $t: 'static,
                $u: 'static,
                C: $consumer_trait<$t, $u> + $($callback_bounds)+,
            {
                let first_predicate = self.predicate.clone();
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, first_consumer, self.consumer.clone());
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, next, next);
                $consumer_type::new(move |t: &$t, u: &$u| {
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
            /// # Type Parameters
            ///
            /// * `C` - Type of the else-branch bi-consumer.
            ///
            /// # Parameters
            ///
            /// * `else_consumer` - The bi-consumer for the else branch
            ///
            /// # Returns
            ///
            /// Returns a new bi-consumer with if-then-else logic
            ///
            /// # Examples
            ///
            #[doc = concat!(
                "```rust\n",
                "use std::sync::Arc;\n",
                "use std::sync::atomic::AtomicUsize;\n",
                "use std::sync::atomic::Ordering;\n",
                "use qubit_function::{", stringify!($consumer_type), ", ", stringify!($consumer_trait), "};\n",
                impl_shared_conditional_consumer!(@doc_state $consumer_trait, "first_state"),
                impl_shared_conditional_consumer!(@doc_state $consumer_trait, "else_state"),
                "let first_calls = Arc::new(AtomicUsize::new(0));\n",
                "let first_observed = Arc::clone(&first_calls);\n",
                "let first = ", stringify!($consumer_type), "::new(", impl_shared_conditional_consumer!(@doc_callback $consumer_trait, "_: &i32, _: &i32", "first_state", "first_observed"), ");\n",
                "let conditional = first.when(|x: &i32, y: &i32| *x > 0 && *y > 0);\n",
                "let else_calls = Arc::new(AtomicUsize::new(0));\n",
                "let else_observed = Arc::clone(&else_calls);\n",
                "let else_consumer = ", stringify!($consumer_type), "::new(", impl_shared_conditional_consumer!(@doc_callback $consumer_trait, "_: &i32, _: &i32", "else_state", "else_observed"), ");\n",
                "let ", impl_shared_conditional_consumer!(@doc_mut $consumer_trait), "combined = conditional.or_else(else_consumer);\n",
                "combined.accept(&1, &2);\n",
                "assert_eq!(first_calls.load(Ordering::SeqCst), 1);\n",
                "assert_eq!(else_calls.load(Ordering::SeqCst), 0);\n",
                "```"
            )]
            pub fn or_else<C>(&self, else_consumer: C) -> $consumer_type<$t, $u>
            where
                $t: 'static,
                $u: 'static,
                C: $consumer_trait<$t, $u> + $($callback_bounds)+,
            {
                let predicate = self.predicate.clone();
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, then_consumer, self.consumer.clone());
                impl_shared_conditional_consumer!(@let_consumer $consumer_trait, else_consumer, else_consumer);
                $consumer_type::new(move |t: &$t, u: &$u| {
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
