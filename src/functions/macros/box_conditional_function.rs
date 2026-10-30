// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Box Conditional Function Macro
//!
//! Generates Box-based conditional function implementations. The `when`
//! method returns an intermediate object; `or_else` consumes it and returns a
//! callable function wrapper.
//!
//! Box type characteristics:
//! - `or_else` consumes `self` because `Box` cannot be cloned
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$box_function_type` - Function wrapper type name
//! * `$else_function_trait` - The name of the else function trait (e.g.,
//!   Function, BiFunction)
//!
//! # Usage Examples
//!
//! ```text
//! // The `impl_box_conditional_function` macro is crate-private.
//! // Its usage pattern in crate source is:
//! //
//! // impl_box_conditional_function!(
//! //     BoxConditionalFunction<T, R>,
//! //     BoxFunction,
//! //     Function
//! // );
//! //
//! // impl_box_conditional_function!(
//! //     BoxConditionalBiFunction<T, U, R>,
//! //     BoxBiFunction,
//! //     BiFunction
//! // );
//! ```

/// Generates `or_else` methods for Box-based conditional function
/// intermediates. The returned Box function wrapper implements the configured
/// function trait.
///
/// Use this macro at the top level, outside any `impl` block.
///
/// Box type characteristics:
/// - `or_else` consumes the intermediate object and returns a callable wrapper
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$box_function_type` - Function wrapper type name
/// * `$else_function_trait` - The name of the else function trait (e.g.,
///   Function, BiFunction)
///
/// # Usage Examples
///
/// ```text
/// // Two-parameter Function
/// // `impl_box_conditional_function!` is crate-private and expanded internally.
/// // impl_box_conditional_function!(
/// //     BoxConditionalFunction<T, R>,
/// //     BoxFunction,
/// //     Function
/// // );
///
/// // Three-parameter BiFunction
/// // impl_box_conditional_function!(
/// //     BoxConditionalBiFunction<T, U, R>,
/// //     BoxBiFunction,
/// //     BiFunction
/// // );
/// ```
macro_rules! impl_box_conditional_function {
    (@doc_or_else BoxFunction) => {
        $crate::functions::macros::impl_box_function_methods!(@doc_when_function BoxFunction, "Function")
    };
    (@doc_or_else BoxStatefulFunction) => {
        $crate::functions::macros::impl_box_function_methods!(@doc_when_stateful BoxStatefulFunction, "StatefulFunction")
    };
    (@doc_or_else BoxMutatingFunction) => {
        $crate::functions::macros::impl_box_function_methods!(@doc_when_mutating BoxMutatingFunction, "MutatingFunction")
    };
    (@doc_or_else BoxStatefulMutatingFunction) => {
        $crate::functions::macros::impl_box_function_methods!(@doc_when_stateful_mutating BoxStatefulMutatingFunction, "StatefulMutatingFunction")
    };
    (@doc_or_else BoxFunctionOnce) => {
        $crate::functions::macros::impl_box_function_methods!(@doc_when_once BoxFunctionOnce, "FunctionOnce")
    };
    (@doc_or_else BoxMutatingFunctionOnce) => {
        $crate::functions::macros::impl_box_function_methods!(@doc_when_mutating_once BoxMutatingFunctionOnce, "MutatingFunctionOnce")
    };
    (@doc_or_else BoxBiFunction) => {
        $crate::functions::macros::impl_box_function_methods!(@doc_when_bi BoxBiFunction, "BiFunction")
    };
    (@doc_or_else BoxBiFunctionOnce) => {
        $crate::functions::macros::impl_box_function_methods!(@doc_when_bi_once BoxBiFunctionOnce, "BiFunctionOnce")
    };
    (@doc_or_else BoxBiMutatingFunction) => {
        $crate::functions::macros::impl_box_function_methods!(@doc_when_bi_mutating BoxBiMutatingFunction, "BiMutatingFunction")
    };
    (@doc_or_else BoxBiMutatingFunctionOnce) => {
        $crate::functions::macros::impl_box_function_methods!(@doc_when_bi_mutating_once BoxBiMutatingFunctionOnce, "BiMutatingFunctionOnce")
    };

    (@let_function Function, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_function FunctionOnce, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_function StatefulFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_function MutatingFunction, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_function MutatingFunctionOnce, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_function StatefulMutatingFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_function BiFunction, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_function BiFunctionOnce, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_function StatefulBiFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_function BiMutatingFunction, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_function BiMutatingFunctionOnce, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_function StatefulBiMutatingFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    // Two generic parameters - Function
    (
        $struct_name:ident<$t:ident, $r:ident>,
        $box_function_type:ident,
        $else_function_trait:ident
    ) => {
        impl<$t, $r> $struct_name<$t, $r> {
            /// Provides an alternative function for when the predicate is not satisfied
            ///
            /// Combines the current conditional function with an alternative function
            /// into a new function that implements the following semantics:
            ///
            /// When the returned function is called with an argument:
            /// - If the predicate is satisfied, it executes the internal function
            /// - If the predicate is NOT satisfied, it executes the alternative function
            ///
            /// # Type Parameters
            ///
            /// * `F` - The alternative function implementation.
            ///
            /// # Parameters
            ///
            /// * `else_function` - The alternative function to execute when predicate fails
            ///
            /// # Returns
            ///
            /// Returns a new function that handles both conditional branches
            ///
            /// # Examples
            ///
            #[doc = $crate::functions::macros::impl_box_conditional_function!(
                @doc_or_else $box_function_type
            )]
            pub fn or_else<F>(self, else_function: F) -> $box_function_type<$t, $r>
            where
                $t: 'static,
                $r: 'static,
                F: $else_function_trait<$t, $r> + 'static,
            {
                let predicate = self.predicate;
                impl_box_conditional_function!(@let_function $else_function_trait, then_function, self.function);
                impl_box_conditional_function!(@let_function $else_function_trait, else_function, else_function);
                $crate::functions::macros::impl_function_new_callback!($else_function_trait, $box_function_type, $t, |t| {
                    if predicate.test(t) {
                        then_function.apply(t)
                    } else {
                        else_function.apply(t)
                    }
                })
            }
        }
    };

    // Three generic parameters - BiFunction
    (
        $struct_name:ident<$t:ident, $u:ident, $r:ident>,
        $box_function_type:ident,
        $else_function_trait:ident
    ) => {
        impl<$t, $u, $r> $struct_name<$t, $u, $r> {
            /// Provides an alternative function for when the predicate is not satisfied
            ///
            /// Combines the current conditional bi-function with an alternative bi-function
            /// into a new bi-function that implements the following semantics:
            ///
            /// When the returned bi-function is called with two arguments:
            /// - If the predicate is satisfied, it executes the internal bi-function
            /// - If the predicate is NOT satisfied, it executes the alternative bi-function
            ///
            /// # Type Parameters
            ///
            /// * `F` - The alternative bi-function implementation.
            ///
            /// # Parameters
            ///
            /// * `else_function` - The alternative bi-function to execute when predicate fails
            ///
            /// # Returns
            ///
            /// Returns a new bi-function that handles both conditional branches
            ///
            /// # Examples
            ///
            #[doc = $crate::functions::macros::impl_box_conditional_function!(
                @doc_or_else $box_function_type
            )]
            pub fn or_else<F>(self, else_function: F) -> $box_function_type<$t, $u, $r>
            where
                $t: 'static,
                $u: 'static,
                $r: 'static,
                F: $else_function_trait<$t, $u, $r> + 'static,
            {
                let predicate = self.predicate;
                impl_box_conditional_function!(@let_function $else_function_trait, then_function, self.function);
                impl_box_conditional_function!(@let_function $else_function_trait, else_function, else_function);
                $crate::functions::macros::impl_function_new_callback!($else_function_trait, $box_function_type, $t, $u, |t, u| {
                    if predicate.test(t, u) {
                        then_function.apply(t, u)
                    } else {
                        else_function.apply(t, u)
                    }
                })
            }
        }
    };
}

pub(crate) use impl_box_conditional_function;
