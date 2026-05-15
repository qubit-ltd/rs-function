/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Box Conditional Transformer Macro
//!
//! Generates Box-based Conditional Transformer implementations
//!
//! For Box-based conditional transformers, generates `and_then` and `or_else`
//! methods, as well as complete Transformer/BiTransformer trait
//! implementations.
//!
//! Box type characteristics:
//! - `and_then` and `or_else` consume self (because Box cannot Clone)
//! - Does not implement `into_arc()` (because Box types are not Send + Sync)
//! - Does not implement `to_xxx()` methods (because Box types cannot Clone)
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$transformer_type` - Transformer wrapper type name
//! * `$else_transformer_trait` - Transformer trait name
//!
//! # Usage Examples
//!
//! ```ignore
//! // Two-parameter Transformer
//! impl_box_conditional_transformer!(
//!     BoxConditionalTransformer<T, R>,
//!     BoxTransformer,
//!     Transformer
//! );
//!
//! // Three-parameter BiTransformer
//! impl_box_conditional_transformer!(
//!     BoxConditionalBiTransformer<T, U, R>,
//!     BoxBiTransformer,
//!     BiTransformer
//! );
//! ```
//!

/// Generates Box-based Conditional Transformer implementations
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// For Box-based conditional transformers, generates `and_then` and `or_else` methods,
/// as well as complete Transformer/BiTransformer trait implementations.
///
/// Box type characteristics:
/// - `and_then` and `or_else` consume self (because Box cannot Clone)
/// - Does not implement `into_arc()` (because Box types are not Send + Sync)
/// - Does not implement `to_xxx()` methods (because Box types cannot Clone)
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$transformer_type` - Transformer wrapper type name
/// * `$transformer_trait` - Transformer trait name
///
/// # Usage Examples
///
/// ```ignore
/// // Two-parameter Transformer
/// impl_box_conditional_transformer!(
///     BoxConditionalTransformer<T, R>,
///     BoxTransformer,
///     Transformer
/// );
///
/// // Three-parameter BiTransformer
/// impl_box_conditional_transformer!(
///     BoxConditionalBiTransformer<T, U, R>,
///     BoxBiTransformer,
///     BiTransformer
/// );
/// ```
///
macro_rules! impl_box_conditional_transformer {
    (@let_transformer Transformer, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_transformer TransformerOnce, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_transformer StatefulTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_transformer BiTransformer, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_transformer BiTransformerOnce, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_transformer StatefulBiTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    // Two generic parameters - Transformer
    (
        $struct_name:ident<$t:ident, $r:ident>,
        $transformer_type:ident,
        $transformer_trait:ident
    ) => {
        impl<$t, $r> $struct_name<$t, $r> {
            /// Adds an else branch
            ///
            /// Executes the original transformer when the condition is satisfied,
            /// otherwise executes `else_transformer`.
            ///
            /// # Parameters
            ///
            /// * `else_transformer` - The transformer for the else branch
            ///
            /// # Returns
            ///
            /// Returns a new transformer with if-then-else logic
            pub fn or_else<F>(self, else_transformer: F) -> $transformer_type<$t, $r>
            where
                $t: 'static,
                $r: 'static,
                F: $transformer_trait<$t, $r> + 'static,
            {
                let predicate = self.predicate;
                impl_box_conditional_transformer!(@let_transformer $transformer_trait, then_transformer, self.transformer);
                impl_box_conditional_transformer!(@let_transformer $transformer_trait, else_transformer, else_transformer);
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
        $struct_name:ident<$t:ident, $u:ident, $r:ident>,
        $transformer_type:ident,
        $transformer_trait:ident
    ) => {
        impl<$t, $u, $r> $struct_name<$t, $u, $r> {
            /// Adds an else branch
            ///
            /// Executes the original transformer when the condition is satisfied,
            /// otherwise executes `else_transformer`.
            ///
            /// # Parameters
            ///
            /// * `else_transformer` - The transformer for the else branch
            ///
            /// # Returns
            ///
            /// Returns a new bi-transformer with if-then-else logic
            pub fn or_else<F>(self, else_transformer: F) -> $transformer_type<$t, $u, $r>
            where
                $t: 'static,
                $u: 'static,
                $r: 'static,
                F: $transformer_trait<$t, $u, $r> + 'static,
            {
                let predicate = self.predicate;
                impl_box_conditional_transformer!(@let_transformer $transformer_trait, then_transformer, self.transformer);
                impl_box_conditional_transformer!(@let_transformer $transformer_trait, else_transformer, else_transformer);
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

pub(crate) use impl_box_conditional_transformer;
