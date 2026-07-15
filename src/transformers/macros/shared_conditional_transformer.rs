// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Shared Conditional Transformer Macro
//!
//! Generates `or_else` for shared Arc/Rc conditional transformers.
//! Generated methods borrow `&self` and return a wrapper from the same
//! ownership and statefulness family.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Conditional wrapper type.
//! * `$transformer_type` - Result wrapper type, such as `ArcTransformer`.
//! * `$else_transformer_trait` - Semantic trait accepted for the additional
//!   callback, such as `Transformer`.
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

/// Generates `or_else` for shared Arc/Rc conditional transformers.
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
macro_rules! impl_shared_conditional_transformer {
    (@let_transformer Transformer, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_transformer StatefulTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_transformer BiTransformer, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_transformer StatefulBiTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    // Two generic parameters - Transformer
    (
        $struct_name:ident < $t:ident, $r:ident >,
        $transformer_type:ident,
        $else_transformer_trait:ident,
        callback_bounds = ($($callback_bounds:tt)+)
    ) => {
        impl<$t, $r> $struct_name<$t, $r> {
            /// Adds an else branch
            ///
            /// Executes the original transformer when the condition is satisfied, otherwise
            /// executes else_transformer.
            ///
            /// # Parameters
            ///
            /// * `else_transformer` - The transformer for the else branch
            ///
            /// # Returns
            ///
            /// Returns a new transformer with if-then-else logic
            pub fn or_else<F>(&self, else_transformer: F) -> $transformer_type<$t, $r>
            where
                $t: 'static,
                $r: 'static,
                F: $else_transformer_trait<$t, $r> + $($callback_bounds)+,
            {
                let predicate = self.predicate.clone();
                impl_shared_conditional_transformer!(@let_transformer $else_transformer_trait, then_transformer, self.transformer.clone());
                impl_shared_conditional_transformer!(@let_transformer $else_transformer_trait, else_transformer, else_transformer);
                $transformer_type::new(move |t| {
                    if predicate.test(&t) {
                        then_transformer.apply(t)
                    } else {
                        else_transformer.apply(t)
                    }
                })
            }
        }
    };

    // Three generic parameters - BiTransformer
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        $transformer_type:ident,
        $else_transformer_trait:ident,
        callback_bounds = ($($callback_bounds:tt)+)
    ) => {
        impl<$t, $u, $r> $struct_name<$t, $u, $r> {
            /// Adds an else branch
            ///
            /// Executes the original bi-transformer when the condition is satisfied, otherwise
            /// executes else_transformer.
            ///
            /// # Parameters
            ///
            /// * `else_transformer` - The bi-transformer for the else branch
            ///
            /// # Returns
            ///
            /// Returns a new bi-transformer with if-then-else logic
            pub fn or_else<F>(&self, else_transformer: F) -> $transformer_type<$t, $u, $r>
            where
                $t: 'static,
                $u: 'static,
                $r: 'static,
                F: $else_transformer_trait<$t, $u, $r> + $($callback_bounds)+,
            {
                let predicate = self.predicate.clone();
                impl_shared_conditional_transformer!(@let_transformer $else_transformer_trait, then_transformer, self.transformer.clone());
                impl_shared_conditional_transformer!(@let_transformer $else_transformer_trait, else_transformer, else_transformer);
                $transformer_type::new(move |t, u| {
                    if predicate.test(&t, &u) {
                        then_transformer.apply(t, u)
                    } else {
                        else_transformer.apply(t, u)
                    }
                })
            }
        }
    };
}

pub(crate) use impl_shared_conditional_transformer;
