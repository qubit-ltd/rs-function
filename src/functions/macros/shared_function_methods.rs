// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Shared Function Methods Macro
//!
//! Generates `when` and `and_then` for shared Arc/Rc functions.
//! The generated methods borrow `&self`, clone the shared wrapper, and keep
//! predicate-storage capabilities separate from chained-callback
//! capabilities.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Function or BiFunction wrapper type.
//! * `$conditional_type` - Conditional wrapper returned by `when`, such as
//!   `ArcConditionalFunction`.
//! * `$predicate_type` - Predicate wrapper used by the conditional result.
//! * `$chained_function_trait` - Semantic trait implemented by the chained
//!   callback, such as `Function`.
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

/// Generates `when` and `and_then` for shared Arc/Rc functions.
///
/// Invoke this macro inside the target wrapper's `impl` block. Predicate
/// bounds describe the immutable predicate object stored by the conditional
/// wrapper. Chained bounds describe the callback stored by the returned
/// function; stateful Arc callbacks are serialized by their outer
/// mutex and therefore require `Send`, but not `Sync`.
///
/// # Capability policy
///
/// | Wrapper family | `predicate_bounds` | `chained_bounds` |
/// |----------------|--------------------|------------------|
/// | Arc stateless | `Send + Sync + 'static` | `Send + Sync + 'static` |
/// | Arc stateful | `Send + Sync + 'static` | `Send + 'static` |
/// | Rc stateless/stateful | `'static` | `'static` |
macro_rules! impl_shared_function_methods {
    (@doc_source_trait RcFunction) => { "Function" };
    (@doc_source_trait ArcFunction) => { "Function" };
    (@doc_source_trait RcBiFunction) => { "BiFunction" };
    (@doc_source_trait ArcBiFunction) => { "BiFunction" };
    (@doc_source_trait RcMutatingFunction) => { "MutatingFunction" };
    (@doc_source_trait ArcMutatingFunction) => { "MutatingFunction" };
    (@doc_source_trait RcBiMutatingFunction) => { "BiMutatingFunction" };
    (@doc_source_trait ArcBiMutatingFunction) => { "BiMutatingFunction" };
    (@doc_source_trait RcStatefulFunction) => { "StatefulFunction" };
    (@doc_source_trait ArcStatefulFunction) => { "StatefulFunction" };
    (@doc_source_trait RcStatefulMutatingFunction) => { "StatefulMutatingFunction" };
    (@doc_source_trait ArcStatefulMutatingFunction) => { "StatefulMutatingFunction" };

    (@doc_when RcFunction) => { impl_shared_function_methods!(@doc_when_function RcFunction) };
    (@doc_when ArcFunction) => { impl_shared_function_methods!(@doc_when_function ArcFunction) };
    (@doc_when RcStatefulFunction) => { impl_shared_function_methods!(@doc_when_stateful RcStatefulFunction) };
    (@doc_when ArcStatefulFunction) => { impl_shared_function_methods!(@doc_when_stateful ArcStatefulFunction) };
    (@doc_when RcMutatingFunction) => { impl_shared_function_methods!(@doc_when_mutating RcMutatingFunction) };
    (@doc_when ArcMutatingFunction) => { impl_shared_function_methods!(@doc_when_mutating ArcMutatingFunction) };
    (@doc_when RcStatefulMutatingFunction) => { impl_shared_function_methods!(@doc_when_stateful_mutating RcStatefulMutatingFunction) };
    (@doc_when ArcStatefulMutatingFunction) => { impl_shared_function_methods!(@doc_when_stateful_mutating ArcStatefulMutatingFunction) };
    (@doc_when RcBiFunction) => { impl_shared_function_methods!(@doc_when_bi RcBiFunction) };
    (@doc_when ArcBiFunction) => { impl_shared_function_methods!(@doc_when_bi ArcBiFunction) };
    (@doc_when RcBiMutatingFunction) => { impl_shared_function_methods!(@doc_when_bi_mutating RcBiMutatingFunction) };
    (@doc_when ArcBiMutatingFunction) => { impl_shared_function_methods!(@doc_when_bi_mutating ArcBiMutatingFunction) };

    (@doc_when_function $wrapper:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", Function};\n",
        "let source = ", stringify!($wrapper), "::new(|value: &i32| *value + 1);\nlet positive = source.when(|value: &i32| *value > 0);\n",
        "assert_eq!(positive.or_else(|_: &i32| 0).apply(&5), 6);\n",
        "let source = ", stringify!($wrapper), "::new(|value: &i32| *value + 1);\nlet negative = source.when(|value: &i32| *value > 0);\n",
        "assert_eq!(negative.or_else(|_: &i32| 0).apply(&-5), 0);\n```"
    ) };
    (@doc_when_stateful $wrapper:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", StatefulFunction};\n",
        "let mut positive_calls = 0;\nlet source = ", stringify!($wrapper), "::new(move |value: &i32| { positive_calls += 1; *value + positive_calls });\nlet positive = source.when(|value: &i32| *value > 0);\n",
        "let mut positive = positive.or_else(|_: &i32| 0);\nassert_eq!(positive.apply(&5), 6);\n",
        "let mut negative_calls = 0;\nlet source = ", stringify!($wrapper), "::new(move |value: &i32| { negative_calls += 1; *value + negative_calls });\nlet negative = source.when(|value: &i32| *value > 0);\n",
        "let mut negative = negative.or_else(|_: &i32| 0);\nassert_eq!(negative.apply(&-5), 0);\n```"
    ) };
    (@doc_when_mutating $wrapper:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", MutatingFunction};\n",
        "let source = ", stringify!($wrapper), "::new(|value: &mut i32| { *value += 1; *value });\nlet positive = source.when(|value: &i32| *value > 0);\nlet mut input = 5;\n",
        "let result = positive.or_else(|_: &mut i32| 0).apply(&mut input);\nassert_eq!((input, result), (6, 6));\n",
        "let source = ", stringify!($wrapper), "::new(|value: &mut i32| { *value += 1; *value });\nlet negative = source.when(|value: &i32| *value > 0);\nlet mut input = -5;\n",
        "let result = negative.or_else(|_: &mut i32| 0).apply(&mut input);\nassert_eq!((input, result), (-5, 0));\n```"
    ) };
    (@doc_when_stateful_mutating $wrapper:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", StatefulMutatingFunction};\n",
        "let mut calls = 0;\nlet source = ", stringify!($wrapper), "::new(move |value: &mut i32| { calls += 1; *value += calls; *value });\nlet positive = source.when(|value: &i32| *value > 0);\nlet mut input = 5;\n",
        "let mut positive = positive.or_else(|_: &mut i32| 0);\nlet result = positive.apply(&mut input);\nassert_eq!((input, result), (6, 6));\n",
        "let mut calls = 0;\nlet source = ", stringify!($wrapper), "::new(move |value: &mut i32| { calls += 1; *value += calls; *value });\nlet negative = source.when(|value: &i32| *value > 0);\nlet mut input = -5;\n",
        "let mut negative = negative.or_else(|_: &mut i32| 0);\nlet result = negative.apply(&mut input);\nassert_eq!((input, result), (-5, 0));\n```"
    ) };
    (@doc_when_bi $wrapper:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", BiFunction};\n",
        "let source = ", stringify!($wrapper), "::new(|first: &i32, second: &i32| first + second);\nlet positive = source.when(|first: &i32, second: &i32| *first > 0 && *second > 0);\n",
        "assert_eq!(positive.or_else(|_: &i32, _: &i32| 0).apply(&2, &3), 5);\n",
        "let source = ", stringify!($wrapper), "::new(|first: &i32, second: &i32| first + second);\nlet negative = source.when(|first: &i32, second: &i32| *first > 0 && *second > 0);\n",
        "assert_eq!(negative.or_else(|_: &i32, _: &i32| 0).apply(&-2, &3), 0);\n```"
    ) };
    (@doc_when_bi_mutating $wrapper:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", BiMutatingFunction};\n",
        "let source = ", stringify!($wrapper), "::new(|first: &mut i32, second: &mut i32| { *first += 1; *second += 2; *first + *second });\nlet positive = source.when(|first: &i32, second: &i32| *first > 0 && *second > 0);\n",
        "let (mut first, mut second) = (2, 3);\nlet result = positive.or_else(|_: &mut i32, _: &mut i32| 0).apply(&mut first, &mut second);\nassert_eq!((first, second, result), (3, 5, 8));\n",
        "let source = ", stringify!($wrapper), "::new(|first: &mut i32, second: &mut i32| { *first += 1; *second += 2; *first + *second });\nlet negative = source.when(|first: &i32, second: &i32| *first > 0 && *second > 0);\n",
        "let (mut first, mut second) = (-2, 3);\nlet result = negative.or_else(|_: &mut i32, _: &mut i32| 0).apply(&mut first, &mut second);\nassert_eq!((first, second, result), (-2, 3, 0));\n```"
    ) };

    (@doc_then RcFunction, $chained_trait:ident) => { impl_shared_function_methods!(@doc_then_function RcFunction, Function, $chained_trait) };
    (@doc_then ArcFunction, $chained_trait:ident) => { impl_shared_function_methods!(@doc_then_function ArcFunction, Function, $chained_trait) };
    (@doc_then RcStatefulFunction, $chained_trait:ident) => { impl_shared_function_methods!(@doc_then_stateful RcStatefulFunction, StatefulFunction, $chained_trait) };
    (@doc_then ArcStatefulFunction, $chained_trait:ident) => { impl_shared_function_methods!(@doc_then_stateful ArcStatefulFunction, StatefulFunction, $chained_trait) };
    (@doc_then RcMutatingFunction, $chained_trait:ident) => { impl_shared_function_methods!(@doc_then_mutating RcMutatingFunction, MutatingFunction, $chained_trait) };
    (@doc_then ArcMutatingFunction, $chained_trait:ident) => { impl_shared_function_methods!(@doc_then_mutating ArcMutatingFunction, MutatingFunction, $chained_trait) };
    (@doc_then RcStatefulMutatingFunction, $chained_trait:ident) => { impl_shared_function_methods!(@doc_then_stateful_mutating RcStatefulMutatingFunction, StatefulMutatingFunction, $chained_trait) };
    (@doc_then ArcStatefulMutatingFunction, $chained_trait:ident) => { impl_shared_function_methods!(@doc_then_stateful_mutating ArcStatefulMutatingFunction, StatefulMutatingFunction, $chained_trait) };
    (@doc_then RcBiFunction, $chained_trait:ident) => { impl_shared_function_methods!(@doc_then_bi RcBiFunction, BiFunction, $chained_trait) };
    (@doc_then ArcBiFunction, $chained_trait:ident) => { impl_shared_function_methods!(@doc_then_bi ArcBiFunction, BiFunction, $chained_trait) };
    (@doc_then RcBiMutatingFunction, $chained_trait:ident) => { impl_shared_function_methods!(@doc_then_bi_mutating RcBiMutatingFunction, BiMutatingFunction, $chained_trait) };
    (@doc_then ArcBiMutatingFunction, $chained_trait:ident) => { impl_shared_function_methods!(@doc_then_bi_mutating ArcBiMutatingFunction, BiMutatingFunction, $chained_trait) };

    (@doc_then_function $wrapper:ident, $source_trait:ident, $chained_trait:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", stringify!($source_trait), "};\n",
        "// Source trait: ", stringify!($source_trait), "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let source = ", stringify!($wrapper), "::new(|value: &i32| *value + 1);\nlet chained = source.and_then(|value: &i32| *value * 2);\nassert_eq!(chained.apply(&5), 12);\n```"
    ) };
    (@doc_then_stateful $wrapper:ident, $source_trait:ident, $chained_trait:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", stringify!($source_trait), "};\n",
        "// Source trait: ", stringify!($source_trait), "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let mut calls = 0;\nlet source = ", stringify!($wrapper), "::new(move |value: &i32| { calls += 1; *value + calls });\nlet mut chained = source.and_then(|value: &i32| *value * 2);\n",
        "assert_eq!(chained.apply(&5), 12);\nassert_eq!(chained.apply(&5), 14);\n```"
    ) };
    (@doc_then_mutating $wrapper:ident, $source_trait:ident, $chained_trait:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", stringify!($source_trait), "};\n",
        "// Source trait: ", stringify!($source_trait), "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let source = ", stringify!($wrapper), "::new(|value: &mut i32| { *value += 1; *value });\nlet chained = source.and_then(|value: &i32| *value * 2);\nlet mut input = 5;\n",
        "assert_eq!(chained.apply(&mut input), 12);\nassert_eq!(input, 6);\n```"
    ) };
    (@doc_then_stateful_mutating $wrapper:ident, $source_trait:ident, $chained_trait:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", stringify!($source_trait), "};\n",
        "// Source trait: ", stringify!($source_trait), "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let mut calls = 0;\nlet source = ", stringify!($wrapper), "::new(move |value: &mut i32| { calls += 1; *value += calls; *value });\nlet mut chained = source.and_then(|value: &i32| *value * 2);\nlet mut input = 5;\n",
        "assert_eq!(chained.apply(&mut input), 12);\nassert_eq!(input, 6);\n```"
    ) };
    (@doc_then_bi $wrapper:ident, $source_trait:ident, $chained_trait:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", stringify!($source_trait), "};\n",
        "// Source trait: ", stringify!($source_trait), "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let source = ", stringify!($wrapper), "::new(|first: &i32, second: &i32| first + second);\nlet chained = source.and_then(|value: &i32| *value * 2);\nassert_eq!(chained.apply(&2, &3), 10);\n```"
    ) };
    (@doc_then_bi_mutating $wrapper:ident, $source_trait:ident, $chained_trait:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", stringify!($source_trait), "};\n",
        "// Source trait: ", stringify!($source_trait), "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let source = ", stringify!($wrapper), "::new(|first: &mut i32, second: &mut i32| { *first += 1; *second += 2; *first + *second });\nlet chained = source.and_then(|value: &mut i32| { *value *= 2; *value });\nlet (mut first, mut second) = (2, 3);\n",
        "assert_eq!(chained.apply(&mut first, &mut second), 16);\nassert_eq!((first, second), (3, 5));\n```"
    ) };
    (@let_before ArcStatefulFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before RcStatefulFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before ArcStatefulMutatingFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before RcStatefulMutatingFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before ArcStatefulBiFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before RcStatefulBiFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before $struct_name:ident, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_after StatefulFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_after StatefulMutatingFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_after $function_trait:ident, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@apply_after Function, $after:ident, $value:expr) => {{
        let value = $value;
        $after.apply(&value)
    }};

    (@apply_after StatefulFunction, $after:ident, $value:expr) => {{
        let value = $value;
        $after.apply(&value)
    }};

    (@apply_after MutatingFunction, $after:ident, $value:expr) => {{
        let mut value = $value;
        $after.apply(&mut value)
    }};

    (@apply_after StatefulMutatingFunction, $after:ident, $value:expr) => {{
        let mut value = $value;
        $after.apply(&mut value)
    }};

    // Two generic parameters - Function types
    (
        $struct_name:ident < $t:ident, $r:ident >,
        $conditional_type:ident,
        $predicate_type:ident,
        $chained_function_trait:ident,
        predicate_bounds = ($($predicate_bounds:tt)+),
        chained_bounds = ($($chained_bounds:tt)+)
    ) => {
        /// Creates a conditional function that executes based on predicate
        /// result.
        ///
        /// # Type Parameters
        ///
        /// * `P` - The predicate type, which implements `Predicate<T>`.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to determine whether to execute
        ///   the function operation
        ///
        /// # Returns
        ///
        /// Returns a conditional function that only executes when the
        /// predicate returns `true`.
        ///
        /// # Examples
        ///
        #[doc = impl_shared_function_methods!(@doc_when $struct_name)]
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $conditional_type<$t, $r>
        where
            $t: 'static,
            $r: 'static,
            P: Predicate<$t> + $($predicate_bounds)+,
        {
            $conditional_type {
                function: self.clone(),
                predicate: $crate::$predicate_type::new(predicate),
            }
        }

        /// Chains execution with another function, executing the current
        /// function first, then the subsequent function.
        ///
        /// # Type Parameters
        ///
        /// * `S` - The result type produced by the subsequent function.
        /// * `F` - The subsequent function type.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent function to execute after the current
        ///   function completes
        ///
        /// # Returns
        ///
        /// Returns a new function that executes the current function and
        /// the subsequent function in sequence.
        ///
        /// # Examples
        ///
        #[doc = impl_shared_function_methods!(@doc_then $struct_name, $chained_function_trait)]
        #[inline]
        pub fn and_then<S, F>(&self, after: F) -> $struct_name<$t, S>
        where
            $t: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_function_trait<$r, S> + $($chained_bounds)+,
        {
            impl_shared_function_methods!(@let_before $struct_name, before, self.clone());
            impl_shared_function_methods!(@let_after $chained_function_trait, after, after);
            $crate::functions::macros::impl_function_new_callback!($chained_function_trait, $struct_name, $t, |t| {
                impl_shared_function_methods!(@apply_after $chained_function_trait, after, before.apply(t))
            })
        }
    };

    // Three generic parameters - BiFunction types
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        $conditional_type:ident,
        $predicate_type:ident,
        $chained_function_trait:ident,
        predicate_bounds = ($($predicate_bounds:tt)+),
        chained_bounds = ($($chained_bounds:tt)+)
    ) => {
        /// Creates a conditional two-parameter function that executes based
        /// on bi-predicate result.
        ///
        /// # Type Parameters
        ///
        /// * `P` - The predicate type, which implements `BiPredicate<T, U>`.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The bi-predicate to determine whether to execute
        ///   the function operation
        ///
        /// # Returns
        ///
        /// Returns a conditional two-parameter function that only executes
        /// when the predicate returns `true`.
        ///
        /// # Examples
        ///
        #[doc = impl_shared_function_methods!(@doc_when $struct_name)]
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $conditional_type<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            P: BiPredicate<$t, $u> + $($predicate_bounds)+,
        {
            $conditional_type {
                function: self.clone(),
                predicate: $crate::$predicate_type::new(predicate),
            }
        }

        /// Passes the result of this two-parameter function to a
        /// single-parameter function.
        ///
        /// # Type Parameters
        ///
        /// * `S` - The result type produced by the subsequent function.
        /// * `F` - The subsequent function type.
        ///
        /// # Parameters
        ///
        /// * `after` - The single-parameter function to apply to the result
        ///   produced by the current function
        ///
        /// # Returns
        ///
        /// Returns a new two-parameter function that preserves the original
        /// two inputs and applies `after` to the current function's result.
        ///
        /// # Examples
        ///
        #[doc = impl_shared_function_methods!(@doc_then $struct_name, $chained_function_trait)]
        #[inline]
        pub fn and_then<S, F>(&self, after: F) -> $struct_name<$t, $u, S>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_function_trait<$r, S> + $($chained_bounds)+,
        {
            impl_shared_function_methods!(@let_before $struct_name, before, self.clone());
            impl_shared_function_methods!(@let_after $chained_function_trait, after, after);
            $crate::functions::macros::impl_function_new_callback!($chained_function_trait, $struct_name, $t, $u, |t, u| {
                impl_shared_function_methods!(@apply_after $chained_function_trait, after, before.apply(t, u))
            })
        }
    };
}

pub(crate) use impl_shared_function_methods;
