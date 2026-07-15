// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Shared Conditional Function Macro
//!
//! Generates `or_else` for shared Arc/Rc conditional functions.
//! Generated methods borrow `&self` and return a wrapper from the same
//! ownership and statefulness family.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Conditional wrapper type.
//! * `$shared_function_type` - Result wrapper type, such as `ArcFunction`.
//! * `$else_function_trait` - Semantic trait accepted for the additional
//!   callback, such as `Function`.
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

/// Generates `or_else` for shared Arc/Rc conditional functions.
///
/// Invoke this macro at module scope. The selected callback bounds reflect
/// how the result wrapper stores its callback: stateless Arc wrappers call
/// through a shared reference and require `Sync`; stateful Arc wrappers call
/// under an outer mutex and require only `Send`.
///
/// # Capability policy
///
/// | Wrapper family | `callback_bounds` |
/// |----------------|-------------------|
/// | Arc stateless | `Send + Sync + 'static` |
/// | Arc stateful | `Send + 'static` |
/// | Rc stateless/stateful | `'static` |
macro_rules! impl_shared_conditional_function {
    (@let_function Function, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_function StatefulFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_function MutatingFunction, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_function StatefulMutatingFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_function BiFunction, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_function StatefulBiFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_function BiMutatingFunction, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_function StatefulBiMutatingFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    // Two generic parameters - Function types
    (
        $struct_name:ident < $t:ident, $r:ident >,
        $shared_function_type:ident,
        $else_function_trait:ident,
        callback_bounds = ($($callback_bounds:tt)+)
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
            /// ```text
            /// // Macro internals are crate-private; usage example is documented at
            /// // the macro expansion site.
            /// ```
            pub fn or_else<F>(&self, else_function: F) -> $shared_function_type<$t, $r>
            where
                $t: 'static,
                $r: 'static,
                F: $else_function_trait<$t, $r> + $($callback_bounds)+,
            {
                let predicate = self.predicate.clone();
                impl_shared_conditional_function!(@let_function $else_function_trait, then_function, self.function.clone());
                impl_shared_conditional_function!(@let_function $else_function_trait, else_function, else_function);
                $crate::functions::macros::impl_function_new_callback!($else_function_trait, $shared_function_type, $t, |t| {
                    if predicate.test(t) {
                        then_function.apply(t)
                    } else {
                        else_function.apply(t)
                    }
                })
            }
        }
    };

    // Three generic parameters - BiFunction types
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        $shared_function_type:ident,
        $else_function_trait:ident,
        callback_bounds = ($($callback_bounds:tt)+)
    ) => {
        impl<$t, $u, $r> $struct_name<$t, $u, $r> {
            /// Provides an alternative bi-function for when the predicate is not satisfied
            ///
            /// Combines the current conditional bi-function with an alternative bi-function
            /// into a new bi-function that implements the following semantics:
            ///
            /// When the returned bi-function is called with arguments:
            /// - If the predicate is satisfied, it executes the internal bi-function
            /// - If the predicate is NOT satisfied, it executes the alternative bi-function
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
            /// ```text
            /// // Macro internals are crate-private; usage example is documented at
            /// // the macro expansion site.
            /// ```
            pub fn or_else<F>(&self, else_function: F) -> $shared_function_type<$t, $u, $r>
            where
                $t: 'static,
                $u: 'static,
                $r: 'static,
                F: $else_function_trait<$t, $u, $r> + $($callback_bounds)+,
            {
                let predicate = self.predicate.clone();
                impl_shared_conditional_function!(@let_function $else_function_trait, then_function, self.function.clone());
                impl_shared_conditional_function!(@let_function $else_function_trait, else_function, else_function);
                $crate::functions::macros::impl_function_new_callback!($else_function_trait, $shared_function_type, $t, $u, |t, u| {
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

pub(crate) use impl_shared_conditional_function;
