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
            /// Combines this conditional function with an alternative callback.
            ///
            /// The returned function borrows neither callback. It clones the
            /// existing callback from `self` and stores both callbacks in a
            /// wrapper from the same shared ownership and statefulness family.
            /// Both input and output types must be `'static` because the returned
            /// wrapper owns the callbacks and may outlive this borrow.
            ///
            /// When called, the returned function invokes the existing callback
            /// if the predicate succeeds, and the alternative callback otherwise.
            ///
            /// # Parameters
            ///
            /// * `else_function` - Callback used when the predicate fails. It must
            ///   implement the function trait accepted by this wrapper and satisfy
            ///   the storage bounds selected for the wrapper family.
            ///
            /// # Returns
            ///
            /// A new shared function wrapper that applies the predicate before
            /// selecting which callback to invoke.
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
            /// Combines this conditional bi-function with an alternative callback.
            ///
            /// The returned bi-function borrows neither callback. It clones the
            /// existing callback from `self` and stores both callbacks in a
            /// wrapper from the same shared ownership and statefulness family.
            /// Both input types and the output type must be `'static` because the
            /// returned wrapper owns the callbacks and may outlive this borrow.
            ///
            /// When called, the returned bi-function invokes the existing callback
            /// if the predicate succeeds, and the alternative callback otherwise.
            ///
            /// # Parameters
            ///
            /// * `else_function` - Callback used when the predicate fails. It must
            ///   implement the bi-function trait accepted by this wrapper and
            ///   satisfy the storage bounds selected for the wrapper family.
            ///
            /// # Returns
            ///
            /// A new shared bi-function wrapper that applies the predicate before
            /// selecting which callback to invoke.
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
