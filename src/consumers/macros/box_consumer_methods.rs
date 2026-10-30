// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Macros used to implement methods for boxed consumer types.

/// Generates `when` and `and_then` method implementations for boxed consumers.
///
/// Invoke this macro inside the target type's `impl` block to generate its
/// `when` and `and_then` methods.
/// The generated methods consume `self`, which supports boxed consumers that
/// cannot be cloned.
///
/// This macro supports both single-parameter and two-parameter consumers
/// through pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `BoxConsumer<T>`
///   - Two parameters: `BoxBiConsumer<T, U>`
/// * `$conditional_type` - The conditional consumer type for when (e.g.,
///   BoxConditionalConsumer)
/// * `$consumer_trait` - Consumer trait name (e.g., Consumer, BiConsumer)
///
/// # Parameter Usage Comparison
///
/// | Consumer Type | Struct Signature | `$conditional_type` | `$consumer_trait` |
/// |---------------|-----------------|----------------|------------------|
/// | **Consumer** | `BoxConsumer<T>` | BoxConditionalConsumer | Consumer |
/// | **ConsumerOnce** | `BoxConsumerOnce<T>` | BoxConditionalConsumerOnce | ConsumerOnce |
/// | **StatefulConsumer** | `BoxStatefulConsumer<T>` | BoxConditionalStatefulConsumer | StatefulConsumer |
/// | **BiConsumer** | `BoxBiConsumer<T, U>` | BoxConditionalBiConsumer | BiConsumer |
/// | **BiConsumerOnce** | `BoxBiConsumerOnce<T, U>` | BoxConditionalBiConsumerOnce | BiConsumerOnce |
/// | **StatefulBiConsumer** | `BoxStatefulBiConsumer<T, U>` | BoxConditionalStatefulBiConsumer | StatefulBiConsumer |
///
/// # Examples
///
/// ```rust
/// use qubit_function::consumers::BoxBiConsumer;
/// use qubit_function::consumers::BoxConsumer;
/// macro_rules! impl_box_consumer_methods {
///     ($struct_name:ty, $conditional_type:ty, $consumer_trait:ident) => {
///         let _ = std::mem::size_of::<$struct_name>();
///         let _ = stringify!($conditional_type);
///         let _ = stringify!($consumer_trait);
///     };
/// }
/// impl_box_consumer_methods!(BoxConsumer<i32>, BoxConditionalConsumer<i32>, Consumer);
/// impl_box_consumer_methods!(BoxBiConsumer<i32, i32>, BoxConditionalBiConsumer<i32, i32>, BiConsumer);
/// ```
macro_rules! impl_box_consumer_methods {
    (@doc_state Consumer, $state:literal, $payload:literal) => { "" };
    (@doc_state BiConsumer, $state:literal, $payload:literal) => { "" };
    (@doc_state StatefulConsumer, $state:literal, $payload:literal) => {
        concat!("let mut ", $state, " = 0;\n")
    };
    (@doc_state StatefulBiConsumer, $state:literal, $payload:literal) => {
        concat!("let mut ", $state, " = 0;\n")
    };
    (@doc_state ConsumerOnce, $state:literal, $payload:literal) => {
        concat!("let ", $payload, " = String::from(\"owned\");\n")
    };
    (@doc_state BiConsumerOnce, $state:literal, $payload:literal) => {
        concat!("let ", $payload, " = String::from(\"owned\");\n")
    };
    (@doc_mut Consumer) => { "" };
    (@doc_mut BiConsumer) => { "" };
    (@doc_mut StatefulConsumer) => { "mut " };
    (@doc_mut StatefulBiConsumer) => { "mut " };
    (@doc_mut ConsumerOnce) => { "" };
    (@doc_mut BiConsumerOnce) => { "" };
    (@doc_apply Consumer) => { "&1" };
    (@doc_apply StatefulConsumer) => { "&1" };
    (@doc_apply ConsumerOnce) => { "&1" };
    (@doc_apply BiConsumer) => { "&1, &2" };
    (@doc_apply StatefulBiConsumer) => { "&1, &2" };
    (@doc_apply BiConsumerOnce) => { "&1, &2" };
    (@doc_when_calls Consumer, $wrapper:ident) => { "conditional.accept(&-1);\nassert_eq!(calls.load(Ordering::SeqCst), 0);\nconditional.accept(&1);\nassert_eq!(calls.load(Ordering::SeqCst), 1);\n" };
    (@doc_when_calls StatefulConsumer, $wrapper:ident) => { "conditional.accept(&-1);\nassert_eq!(calls.load(Ordering::SeqCst), 0);\nconditional.accept(&1);\nassert_eq!(calls.load(Ordering::SeqCst), 1);\n" };
    (@doc_when_calls BiConsumer, $wrapper:ident) => { "conditional.accept(&-1, &1);\nassert_eq!(calls.load(Ordering::SeqCst), 0);\nconditional.accept(&1, &1);\nassert_eq!(calls.load(Ordering::SeqCst), 1);\n" };
    (@doc_when_calls StatefulBiConsumer, $wrapper:ident) => { "conditional.accept(&-1, &1);\nassert_eq!(calls.load(Ordering::SeqCst), 0);\nconditional.accept(&1, &1);\nassert_eq!(calls.load(Ordering::SeqCst), 1);\n" };
    (@doc_when_calls BiConsumerOnce, $wrapper:ident) => {
        concat!("conditional.accept(&-1, &1);\nassert_eq!(calls.load(Ordering::SeqCst), 0);\n",
            "let hit_payload = String::from(\"owned\");\n",
            "let hit_calls = Arc::new(AtomicUsize::new(0));\n",
            "let hit_observed = Arc::clone(&hit_calls);\n",
            "let hit = ", stringify!($wrapper), "::new(move |_: &i32, _: &i32| { drop(hit_payload); hit_observed.fetch_add(1, Ordering::SeqCst); });\n",
            "hit.when(|x: &i32, y: &i32| *x > 0 && *y > 0).accept(&1, &1);\n",
            "assert_eq!(hit_calls.load(Ordering::SeqCst), 1);\n")
    };
    (@doc_when_calls ConsumerOnce, $wrapper:ident) => {
        concat!("conditional.accept(&-1);\nassert_eq!(calls.load(Ordering::SeqCst), 0);\n",
            "let hit_payload = String::from(\"owned\");\n",
            "let hit_calls = Arc::new(AtomicUsize::new(0));\n",
            "let hit_observed = Arc::clone(&hit_calls);\n",
            "let hit = ", stringify!($wrapper), "::new(move |_: &i32| { drop(hit_payload); hit_observed.fetch_add(1, Ordering::SeqCst); });\n",
            "hit.when(|x: &i32| *x > 0).accept(&1);\n",
            "assert_eq!(hit_calls.load(Ordering::SeqCst), 1);\n")
    };
    (@doc_callback Consumer, $args:literal, $state:literal, $payload:literal, $observed:literal) => {
        concat!("move |", $args, "| { ", $observed, ".fetch_add(1, Ordering::SeqCst); }")
    };
    (@doc_callback BiConsumer, $args:literal, $state:literal, $payload:literal, $observed:literal) => {
        concat!("move |", $args, "| { ", $observed, ".fetch_add(1, Ordering::SeqCst); }")
    };
    (@doc_callback StatefulConsumer, $args:literal, $state:literal, $payload:literal, $observed:literal) => {
        concat!("move |", $args, "| { ", $state, " += 1; ", $observed, ".store(", $state, ", Ordering::SeqCst); }")
    };
    (@doc_callback StatefulBiConsumer, $args:literal, $state:literal, $payload:literal, $observed:literal) => {
        concat!("move |", $args, "| { ", $state, " += 1; ", $observed, ".store(", $state, ", Ordering::SeqCst); }")
    };
    (@doc_callback ConsumerOnce, $args:literal, $state:literal, $payload:literal, $observed:literal) => {
        concat!("move |", $args, "| { drop(", $payload, "); ", $observed, ".fetch_add(1, Ordering::SeqCst); }")
    };
    (@doc_callback BiConsumerOnce, $args:literal, $state:literal, $payload:literal, $observed:literal) => {
        concat!("move |", $args, "| { drop(", $payload, "); ", $observed, ".fetch_add(1, Ordering::SeqCst); }")
    };

    (@and_then Consumer, $struct_name:ident, $first:expr, $after:expr, $t:ident) => {{
        let first = $first;
        let after = $after;
        $struct_name::new(move |t: &$t| {
            first.accept(t);
            after.accept(t);
        })
    }};

    (@and_then ConsumerOnce, $struct_name:ident, $first:expr, $after:expr, $t:ident) => {{
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

    (@and_then_bi BiConsumerOnce, $struct_name:ident, $first:expr, $after:expr, $t:ident, $u:ident) => {{
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

    // Single generic parameter - Consumer
    (
        $struct_name:ident < $t:ident >,
        $conditional_type:ident,
        $consumer_trait:ident
    ) => {
        /// Creates a conditional consumer that executes based on predicate
        /// result.
        ///
        /// # Type Parameters
        ///
        /// * `P` - The predicate type used to decide whether the consumer
        ///   runs.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to determine whether to execute
        ///   the consumption operation
        ///
        /// # Returns
        ///
        /// Returns a conditional consumer that only executes when the
        /// predicate returns `true`.
        ///
        /// # Examples
        ///
        #[doc = concat!(
            "```rust\n",
            "use std::sync::Arc;\n",
            "use std::sync::atomic::AtomicUsize;\n",
            "use std::sync::atomic::Ordering;\n",
            "use qubit_function::", stringify!($struct_name), ";\n",
            "use qubit_function::", stringify!($consumer_trait), ";\n",
            impl_box_consumer_methods!(@doc_state $consumer_trait, "callback_calls", "payload"),
            "let calls = Arc::new(AtomicUsize::new(0));\n",
            "let observed = Arc::clone(&calls);\n",
            "let consumer = ", stringify!($struct_name), "::new(",
            impl_box_consumer_methods!(@doc_callback $consumer_trait, "_: &i32", "callback_calls", "payload", "observed"), ");\n",
            "let ", impl_box_consumer_methods!(@doc_mut $consumer_trait), "conditional = consumer.when(|x: &i32| *x > 0);\n",
            impl_box_consumer_methods!(@doc_when_calls $consumer_trait, $struct_name),
            "```"
        )]
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t>
        where
            P: Predicate<$t> + 'static,
        {
            $conditional_type {
                consumer: self,
                predicate: $crate::BoxPredicate::new(predicate),
            }
        }

        /// Chains execution with another consumer, executing the current
        /// consumer first, then the subsequent consumer.
        ///
        /// # Type Parameters
        ///
        /// * `C` - The subsequent consumer type.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent consumer to execute after the current
        ///   consumer completes
        ///
        /// # Returns
        ///
        /// Returns a new consumer that executes the current consumer and
        /// the subsequent consumer in sequence.
        ///
        /// # Examples
        ///
        #[doc = concat!(
            "```rust\n",
            "use std::sync::Arc;\n",
            "use std::sync::atomic::AtomicUsize;\n",
            "use std::sync::atomic::Ordering;\n",
            "use qubit_function::", stringify!($struct_name), ";\n",
            "use qubit_function::", stringify!($consumer_trait), ";\n",
            impl_box_consumer_methods!(@doc_state $consumer_trait, "first_state", "first_payload"),
            impl_box_consumer_methods!(@doc_state $consumer_trait, "after_state", "after_payload"),
            "let first_calls = Arc::new(AtomicUsize::new(0));\n",
            "let first_observed = Arc::clone(&first_calls);\n",
            "let first = ", stringify!($struct_name), "::new(",
            impl_box_consumer_methods!(@doc_callback $consumer_trait, "_: &i32", "first_state", "first_payload", "first_observed"), ");\n",
            "let after_calls = Arc::new(AtomicUsize::new(0));\n",
            "let after_observed = Arc::clone(&after_calls);\n",
            "let after = ", stringify!($struct_name), "::new(",
            impl_box_consumer_methods!(@doc_callback $consumer_trait, "_: &i32", "after_state", "after_payload", "after_observed"), ");\n",
            "let ", impl_box_consumer_methods!(@doc_mut $consumer_trait), "chained = first.and_then(after);\n",
            "chained.accept(", impl_box_consumer_methods!(@doc_apply $consumer_trait), ");\n",
            "assert_eq!(first_calls.load(Ordering::SeqCst), 1);\n",
            "assert_eq!(after_calls.load(Ordering::SeqCst), 1);\n",
            "```"
        )]
        pub fn and_then<C>(self, after: C) -> $struct_name<$t>
        where
            Self: Sized + 'static,
            $t: 'static,
            C: $consumer_trait<$t> + 'static,
        {
            impl_box_consumer_methods!(@and_then $consumer_trait, $struct_name, self, after, $t)
        }
    };

    // Two generic parameters - BiConsumer
    (
        $struct_name:ident < $t:ident, $u:ident >,
        $conditional_type:ident,
        $consumer_trait:ident
    ) => {
        /// Creates a conditional two-parameter consumer that executes based
        /// on bi-predicate result.
        ///
        /// # Type Parameters
        ///
        /// * `P` - The bi-predicate type used to decide whether the consumer
        ///   runs.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The bi-predicate to determine whether to execute
        ///   the consumption operation
        ///
        /// # Returns
        ///
        /// Returns a conditional two-parameter consumer that only executes
        /// when the predicate returns `true`.
        ///
        /// # Examples
        ///
        #[doc = concat!(
            "```rust\n",
            "use std::sync::Arc;\n",
            "use std::sync::atomic::AtomicUsize;\n",
            "use std::sync::atomic::Ordering;\n",
            "use qubit_function::", stringify!($struct_name), ";\n",
            "use qubit_function::", stringify!($consumer_trait), ";\n",
            impl_box_consumer_methods!(@doc_state $consumer_trait, "callback_calls", "payload"),
            "let calls = Arc::new(AtomicUsize::new(0));\n",
            "let observed = Arc::clone(&calls);\n",
            "let consumer = ", stringify!($struct_name), "::new(",
            impl_box_consumer_methods!(@doc_callback $consumer_trait, "_: &i32, _: &i32", "callback_calls", "payload", "observed"), ");\n",
            "let ", impl_box_consumer_methods!(@doc_mut $consumer_trait), "conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);\n",
            impl_box_consumer_methods!(@doc_when_calls $consumer_trait, $struct_name),
            "```"
        )]
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t, $u>
        where
            P: BiPredicate<$t, $u> + 'static,
        {
            $conditional_type {
                consumer: self,
                predicate: $crate::BoxBiPredicate::new(predicate),
            }
        }

        /// Chains execution with another two-parameter consumer, executing
        /// the current consumer first, then the subsequent consumer.
        ///
        /// # Type Parameters
        ///
        /// * `C` - The subsequent two-parameter consumer type.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent two-parameter consumer to execute after
        ///   the current consumer completes
        ///
        /// # Returns
        ///
        /// Returns a new two-parameter consumer that executes the current
        /// consumer and the subsequent consumer in sequence.
        ///
        /// # Examples
        ///
        #[doc = concat!(
            "```rust\n",
            "use std::sync::Arc;\n",
            "use std::sync::atomic::AtomicUsize;\n",
            "use std::sync::atomic::Ordering;\n",
            "use qubit_function::", stringify!($struct_name), ";\n",
            "use qubit_function::", stringify!($consumer_trait), ";\n",
            impl_box_consumer_methods!(@doc_state $consumer_trait, "first_state", "first_payload"),
            impl_box_consumer_methods!(@doc_state $consumer_trait, "after_state", "after_payload"),
            "let first_calls = Arc::new(AtomicUsize::new(0));\n",
            "let first_observed = Arc::clone(&first_calls);\n",
            "let first = ", stringify!($struct_name), "::new(",
            impl_box_consumer_methods!(@doc_callback $consumer_trait, "_: &i32, _: &i32", "first_state", "first_payload", "first_observed"), ");\n",
            "let after_calls = Arc::new(AtomicUsize::new(0));\n",
            "let after_observed = Arc::clone(&after_calls);\n",
            "let after = ", stringify!($struct_name), "::new(",
            impl_box_consumer_methods!(@doc_callback $consumer_trait, "_: &i32, _: &i32", "after_state", "after_payload", "after_observed"), ");\n",
            "let ", impl_box_consumer_methods!(@doc_mut $consumer_trait), "chained = first.and_then(after);\n",
            "chained.accept(", impl_box_consumer_methods!(@doc_apply $consumer_trait), ");\n",
            "assert_eq!(first_calls.load(Ordering::SeqCst), 1);\n",
            "assert_eq!(after_calls.load(Ordering::SeqCst), 1);\n",
            "```"
        )]
        pub fn and_then<C>(self, after: C) -> $struct_name<$t, $u>
        where
            Self: Sized + 'static,
            $t: 'static,
            $u: 'static,
            C: $consumer_trait<$t, $u> + 'static,
        {
            impl_box_consumer_methods!(@and_then_bi $consumer_trait, $struct_name, self, after, $t, $u)
        }
    };
}

pub(crate) use impl_box_consumer_methods;
