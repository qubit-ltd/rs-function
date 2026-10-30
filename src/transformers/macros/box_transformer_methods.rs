// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Box Transformer Methods Macro
//!
//! Generates `when` and `and_then` methods for Box-based transformers.
//!
//! The generated methods support predicate-based conditional execution and
//! chaining for Box-based transformers that consume `self`.
//!
//! Separate patterns handle single-parameter and two-parameter transformers.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `BoxTransformer<T, U>`
//!   - Two parameters: `BoxBiTransformer<T, U, V>`
//! * `$conditional_type` - The conditional transformer type for when (e.g.,
//!   BoxConditionalTransformer)
//! * `$chained_transformer_trait` - The trait implemented by the next
//!   transformer, which receives the preceding transformer’s single return
//!   value. Its input arity is independent of the preceding transformer’s input
//!   arity.
//!
//! # Parameter Usage Comparison
//!
//! | Transformer Type | Struct Signature | `$conditional_type` | `$chained_transformer_trait` |
//! |------------------|-----------------|----------------|---------------------|
//! | **Transformer** | `BoxTransformer<T, U>` | BoxConditionalTransformer | Transformer |
//! | **TransformerOnce** | `BoxTransformerOnce<T, U>` | BoxConditionalTransformerOnce | TransformerOnce |
//! | **StatefulTransformer** | `BoxStatefulTransformer<T, U>` | BoxConditionalStatefulTransformer | StatefulTransformer |
//! | **BiTransformer** | `BoxBiTransformer<T, U, V>` | BoxConditionalBiTransformer | Transformer |
//! | **BiTransformerOnce** | `BoxBiTransformerOnce<T, U, V>` | BoxConditionalBiTransformerOnce | TransformerOnce |
//! | **StatefulBiTransformer** | `BoxStatefulBiTransformer<T, U, V>` | BoxConditionalStatefulBiTransformer | StatefulTransformer |
//!
//! # Examples
//!
//! ```text
//! // Single-parameter transformer
//! // impl_box_transformer_methods!(
//! //     BoxTransformer<T, U>,
//! //     BoxConditionalTransformer,
//! //     Transformer
//! // );
//! //
//! // Two-parameter transformer
//! // impl_box_transformer_methods!(
//! //     BoxBiTransformer<T, U, V>,
//! //     BoxConditionalBiTransformer,
//! //     Transformer
//! // );
//! ```

/// Generates `when` and `and_then` methods for Box-based transformers.
///
/// Invoke this macro inside the inherent `impl` block for the specified struct.
/// It emits methods for predicate-based conditional execution and transformer
/// chaining. These methods consume `self`, which supports Box-based
/// transformers that cannot be cloned.
///
/// This macro supports both single-parameter and two-parameter transformers
/// through pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `BoxTransformer<T, U>`
///   - Two parameters: `BoxBiTransformer<T, U, V>`
/// * `$conditional_type` - The conditional transformer type for when (e.g.,
///   BoxConditionalTransformer)
/// * `$chained_transformer_trait` - The trait implemented by the next
///   transformer, which receives the preceding transformer’s single return
///   value. Its input arity is independent of the preceding transformer’s input
///   arity.
///
/// # Parameter Usage Comparison
///
/// | Transformer Type | Struct Signature | `$conditional_type` | `$chained_transformer_trait` |
/// |------------------|-----------------|----------------|---------------------|
/// | **Transformer** | `BoxTransformer<T, U>` | BoxConditionalTransformer | Transformer |
/// | **TransformerOnce** | `BoxTransformerOnce<T, U>` | BoxConditionalTransformerOnce | TransformerOnce |
/// | **StatefulTransformer** | `BoxStatefulTransformer<T, U>` | BoxConditionalStatefulTransformer | StatefulTransformer |
/// | **BiTransformer** | `BoxBiTransformer<T, U, V>` | BoxConditionalBiTransformer | Transformer |
/// | **BiTransformerOnce** | `BoxBiTransformerOnce<T, U, V>` | BoxConditionalBiTransformerOnce | TransformerOnce |
/// | **StatefulBiTransformer** | `BoxStatefulBiTransformer<T, U, V>` | BoxConditionalStatefulBiTransformer | StatefulTransformer |
///
/// # Examples
///
/// ```text
/// // Single-parameter transformer
/// // impl_box_transformer_methods!(
/// //     BoxTransformer<T, U>,
/// //     BoxConditionalTransformer,
/// //     Transformer
/// // );
/// //
/// // Two-parameter transformer
/// // impl_box_transformer_methods!(
/// //     BoxBiTransformer<T, U, V>,
/// //     BoxConditionalBiTransformer,
/// //     Transformer
/// // );
/// ```
macro_rules! impl_box_transformer_methods {
    (@doc_when BoxTransformer) => { concat!(
        "```rust\nuse qubit_function::{BoxTransformer, Transformer};\n",
        "let source = BoxTransformer::new(|value: i32| value + 1);\n",
        "let mut conditional = source.when(|value: &i32| *value > 0).or_else(|_: i32| 0);\n",
        "assert_eq!(conditional.apply(2), 3);\nassert_eq!(conditional.apply(-2), 0);\n```"
    ) };
    (@doc_then BoxTransformer) => { concat!(
        "```rust\nuse qubit_function::{BoxTransformer, Transformer};\n",
        "let source = BoxTransformer::new(|value: i32| value + 1);\n",
        "let combined = source.and_then(|value: i32| value * 2);\nassert_eq!(combined.apply(2), 6);\n```"
    ) };
    (@doc_when BoxTransformerOnce) => { concat!(
        "```rust\nuse qubit_function::{BoxTransformerOnce, TransformerOnce};\n",
        "let suffix = String::from(\"!\");\nlet source = BoxTransformerOnce::new(move |value: i32| (value + 1, suffix));\n",
        "let selected = source.when(|value: &i32| *value > 0).or_else(|value: i32| (value, String::from(\"fallback\")));\n",
        "assert_eq!(selected.apply(2), (3, String::from(\"!\")));\n",
        "let suffix = String::from(\"!\");\nlet source = BoxTransformerOnce::new(move |value: i32| (value + 1, suffix));\n",
        "let rejected = source.when(|value: &i32| *value > 0).or_else(|value: i32| (value, String::from(\"fallback\")));\n",
        "assert_eq!(rejected.apply(-2), (-2, String::from(\"fallback\")));\n```"
    ) };
    (@doc_then BoxTransformerOnce) => { concat!(
        "```rust\nuse qubit_function::{BoxTransformerOnce, TransformerOnce};\n",
        "let suffix = String::from(\"!\");\nlet source = BoxTransformerOnce::new(move |value: i32| (value + 1, suffix));\n",
        "let extra = String::from(\"?\");\nlet combined = source.and_then(move |(value, suffix): (i32, String)| (value * 2, suffix, extra));\n",
        "assert_eq!(combined.apply(2), (6, String::from(\"!\"), String::from(\"?\")));\n```"
    ) };
    (@doc_when BoxStatefulTransformer) => { concat!(
        "```rust\nuse qubit_function::{BoxStatefulTransformer, StatefulTransformer};\n",
        "let mut calls = 0;\nlet source = BoxStatefulTransformer::new(move |value: i32| { calls += 1; value + calls });\n",
        "let mut conditional = source.when(|value: &i32| *value > 0).or_else(|_: i32| 0);\n",
        "assert_eq!(conditional.apply(2), 3);\nassert_eq!(conditional.apply(-2), 0);\nassert_eq!(conditional.apply(2), 4);\n```"
    ) };
    (@doc_then BoxStatefulTransformer) => { concat!(
        "```rust\nuse qubit_function::{BoxStatefulTransformer, StatefulTransformer};\n",
        "let mut calls = 0;\nlet source = BoxStatefulTransformer::new(move |value: i32| { calls += 1; value + calls });\n",
        "let mut combined = source.and_then(|value: i32| value * 2);\n",
        "assert_eq!(combined.apply(2), 6);\nassert_eq!(combined.apply(2), 8);\n```"
    ) };
    (@doc_when BoxBiTransformer) => { concat!(
        "```rust\nuse qubit_function::{BiTransformer, BoxBiTransformer};\n",
        "let source = BoxBiTransformer::new(|first: i32, second: i32| first + second);\n",
        "let mut conditional = source.when(|first: &i32, _second: &i32| *first > 0).or_else(|_: i32, _: i32| 0);\n",
        "assert_eq!(conditional.apply(2, 3), 5);\nassert_eq!(conditional.apply(-2, 3), 0);\n```"
    ) };
    (@doc_then BoxBiTransformer) => { concat!(
        "```rust\nuse qubit_function::{BiTransformer, BoxBiTransformer, Transformer};\n",
        "let source = BoxBiTransformer::new(|first: i32, second: i32| first + second);\n",
        "let combined = source.and_then(|value: i32| value * 2);\nassert_eq!(combined.apply(2, 3), 10);\n```"
    ) };
    (@doc_when BoxBiTransformerOnce) => { concat!(
        "```rust\nuse qubit_function::{BiTransformerOnce, BoxBiTransformerOnce};\n",
        "let suffix = String::from(\"!\");\nlet source = BoxBiTransformerOnce::new(move |value: i32, extra: i32| (value + extra, suffix));\n",
        "let selected = source.when(|value: &i32, _extra: &i32| *value > 0).or_else(|value: i32, extra: i32| (value + extra, String::from(\"fallback\")));\n",
        "assert_eq!(selected.apply(2, 3), (5, String::from(\"!\")));\n",
        "let suffix = String::from(\"!\");\nlet source = BoxBiTransformerOnce::new(move |value: i32, extra: i32| (value + extra, suffix));\n",
        "let rejected = source.when(|value: &i32, _extra: &i32| *value > 0).or_else(|value: i32, extra: i32| (value + extra, String::from(\"fallback\")));\n",
        "assert_eq!(rejected.apply(-2, 3), (1, String::from(\"fallback\")));\n```"
    ) };
    (@doc_then BoxBiTransformerOnce) => { concat!(
        "```rust\nuse qubit_function::{BiTransformerOnce, BoxBiTransformerOnce, TransformerOnce};\n",
        "let suffix = String::from(\"!\");\nlet source = BoxBiTransformerOnce::new(move |first: i32, second: i32| (first + second, suffix));\n",
        "let extra = String::from(\"?\");\nlet combined = source.and_then(move |(value, suffix): (i32, String)| (value * 2, suffix, extra));\n",
        "assert_eq!(combined.apply(2, 3), (10, String::from(\"!\"), String::from(\"?\")));\n```"
    ) };
    (@doc_when BoxStatefulBiTransformer) => { concat!(
        "```rust\nuse qubit_function::{BoxStatefulBiTransformer, StatefulBiTransformer};\n",
        "let mut calls = 0;\nlet source = BoxStatefulBiTransformer::new(move |first: i32, second: i32| { calls += 1; first + second + calls });\n",
        "let mut conditional = source.when(|first: &i32, _second: &i32| *first > 0).or_else(|_: i32, _: i32| 0);\n",
        "assert_eq!(conditional.apply(2, 3), 6);\nassert_eq!(conditional.apply(-2, 3), 0);\nassert_eq!(conditional.apply(2, 3), 7);\n```"
    ) };
    (@doc_then BoxStatefulBiTransformer) => { concat!(
        "```rust\nuse qubit_function::{BoxStatefulBiTransformer, StatefulBiTransformer, StatefulTransformer};\n",
        "let mut calls = 0;\nlet source = BoxStatefulBiTransformer::new(move |first: i32, second: i32| { calls += 1; first + second + calls });\n",
        "let mut combined = source.and_then(|value: i32| value * 2);\n",
        "assert_eq!(combined.apply(2, 3), 12);\nassert_eq!(combined.apply(2, 3), 14);\n```"
    ) };

    (@let_before BoxStatefulTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before BoxStatefulBiTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before $struct_name:ident, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_after StatefulTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_after $transformer_trait:ident, $name:ident, $value:expr) => {
        let $name = $value;
    };

    // Two generic parameter - Transformer
    (
        $struct_name:ident < $t:ident, $r:ident >,
        $conditional_type:ident,
        $chained_transformer_trait:ident
    ) => {
        /// Creates a conditional transformer that executes based on predicate
        /// result.
        ///
        /// # Lifetimes
        ///
        /// This composition method requires `T` and `R` to be `'static`.
        /// Direct application can accept borrowed input types when the
        /// transformer implementation permits them.
        ///
        /// # Type Parameters
        ///
        /// * `P` - The predicate evaluated for each input value.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to determine whether to execute
        ///   the transformation operation
        ///
        /// # Returns
        ///
        /// Returns a conditional transformer that only executes when the
        /// predicate returns `true`.
        ///
        /// # Examples
        ///
        #[doc = impl_box_transformer_methods!(@doc_when $struct_name)]
        #[inline]
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t, $r>
        where
            $t: 'static,
            $r: 'static,
            P: Predicate<$t> + 'static,
        {
            $conditional_type {
                transformer: self,
                predicate: $crate::BoxPredicate::new(predicate),
            }
        }

        /// Passes the output of this transformer to a single-parameter
        /// transformer.
        ///
        /// # Lifetimes
        ///
        /// This composition method requires `T`, `R`, and `S` to be `'static`.
        /// Direct application can accept borrowed input types when the
        /// transformer implementation permits them.
        ///
        /// # Type Parameters
        ///
        /// * `S` - The output type produced by the subsequent transformer.
        /// * `F` - The subsequent single-parameter transformer implementation.
        ///
        /// For `TransformerOnce` variants, `F` implements `TransformerOnce`.
        ///
        /// # Parameters
        ///
        /// * `after` - The single-parameter transformer to apply to the output
        ///   of the current transformer
        ///
        /// # Returns
        ///
        /// Returns a new transformer that preserves the original input shape
        /// and applies `after` to the current transformer's output.
        ///
        /// # Examples
        ///
        #[doc = impl_box_transformer_methods!(@doc_then $struct_name)]
        #[inline]
        pub fn and_then<S, F>(self, after: F) -> $struct_name<$t, S>
        where
            $t: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_transformer_trait<$r, S> + 'static,
        {
            impl_box_transformer_methods!(@let_before $struct_name, before, self.function);
            impl_box_transformer_methods!(@let_after $chained_transformer_trait, after, after);
            $struct_name::new(move |t| {
                let r = before(t);
                after.apply(r)
            })
        }
    };

    // Three generic parameters - BiTransformer
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        $conditional_type:ident,
        $chained_transformer_trait:ident
    ) => {
        /// Creates a conditional two-parameter transformer that executes based
        /// on bi-predicate result.
        ///
        /// # Lifetimes
        ///
        /// This composition method requires `T`, `U`, and `R` to be `'static`.
        /// Direct application can accept borrowed input types when the
        /// transformer implementation permits them.
        ///
        /// # Type Parameters
        ///
        /// * `P` - The predicate evaluated for each pair of input values.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The bi-predicate to determine whether to execute
        ///   the transformation operation
        ///
        /// # Returns
        ///
        /// Returns a conditional two-parameter transformer that only executes
        /// when the predicate returns `true`.
        ///
        /// # Examples
        ///
        #[doc = impl_box_transformer_methods!(@doc_when $struct_name)]
        #[inline]
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            P: BiPredicate<$t, $u> + 'static,
        {
            $conditional_type {
                transformer: self,
                predicate: $crate::BoxBiPredicate::new(predicate),
            }
        }

        /// Passes the output of this two-parameter transformer to a
        /// single-parameter transformer.
        ///
        /// # Lifetimes
        ///
        /// This composition method requires `T`, `U`, `R`, and `S` to be
        /// `'static`. Direct application can accept borrowed input types when
        /// the transformer implementation permits them.
        ///
        /// # Type Parameters
        ///
        /// * `S` - The output type produced by the subsequent transformer.
        /// * `F` - The subsequent single-parameter transformer implementation.
        ///
        /// For `BiTransformerOnce` variants, `F` implements `TransformerOnce`.
        ///
        /// # Parameters
        ///
        /// * `after` - The single-parameter transformer to apply to the output
        ///   of the current transformer
        ///
        /// # Returns
        ///
        /// Returns a new two-parameter transformer that preserves both
        /// original inputs and applies `after` to the current transformer's
        /// output.
        ///
        /// # Examples
        ///
        #[doc = impl_box_transformer_methods!(@doc_then $struct_name)]
        #[inline]
        pub fn and_then<S, F>(self, after: F) -> $struct_name<$t, $u, S>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_transformer_trait<$r, S> + 'static,
        {
            impl_box_transformer_methods!(@let_before $struct_name, before, self.function);
            impl_box_transformer_methods!(@let_after $chained_transformer_trait, after, after);
            $struct_name::new(move |t, u| {
                let r = before(t, u);
                after.apply(r)
            })
        }
    };
}

pub(crate) use impl_box_transformer_methods;
