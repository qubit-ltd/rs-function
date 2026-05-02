/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Shared Conditional Transformer Macro
//!
//! Generates Arc/Rc-based Conditional Transformer implementations
//!
//! For Arc/Rc-based conditional transformers, generates `and_then` and `or_else`
//! methods, as well as complete Transformer/BiTransformer trait
//! implementations.
//!
//! Arc/Rc type characteristics:
//! - `and_then` and `or_else` borrow &self (because Arc/Rc can Clone)
//! - Uses trait default implementations for `into_arc()` and `to_arc()`
//! - Arc types will work with `into_arc()` and `to_arc()` (satisfy Send + Sync
//!   constraints)
//! - Rc types will get compile errors if trying to use `into_arc()` or
//!   `to_arc()` (don't satisfy Send + Sync)
//! - Implement complete `to_xxx()` methods (because they can Clone)
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$transformer_type` - Transformer wrapper type name
//! * `$else_transformer_trait` - Transformer trait name
//! * `$predicate_conversion` - Predicate conversion method (into_arc or into_rc)
//! * `$extra_bounds` - Extra trait bounds
//!
//! # Usage Examples
//!
//! ```ignore
//! // Arc two-parameter Transformer
//! impl_shared_conditional_transformer!(
//!     ArcConditionalTransformer<T, U>,
//!     ArcTransformer,
//!     Transformer,
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc two-parameter Transformer
//! impl_shared_conditional_transformer!(
//!     RcConditionalTransformer<T, U>,
//!     RcTransformer,
//!     Transformer,
//!     into_rc,
//!     'static
//! );
//!
//! // Arc three-parameter BiTransformer
//! impl_shared_conditional_transformer!(
//!     ArcConditionalBiTransformer<T, U, V>,
//!     ArcBiTransformer,
//!     BiTransformer,
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc three-parameter BiTransformer
//! impl_shared_conditional_transformer!(
//!     RcConditionalBiTransformer<T, U, V>,
//!     RcBiTransformer,
//!     BiTransformer,
//!     into_rc,
//!     'static
//! );
//! ```
//!

/// Generates Arc/Rc-based Conditional Transformer implementations
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// For Arc/Rc-based conditional transformers, generates `and_then` and `or_else` methods,
/// as well as complete Transformer/BiTransformer trait implementations.
///
/// Arc/Rc type characteristics:
/// - `and_then` and `or_else` borrow &self (because Arc/Rc can Clone)
/// - Uses trait default implementations for `into_arc()` and `to_arc()`
/// - Arc types will work with `into_arc()` and `to_arc()` (satisfy Send + Sync constraints)
/// - Rc types will get compile errors if trying to use `into_arc()` or `to_arc()` (don't satisfy Send + Sync)
/// - Implement complete `to_xxx()` methods (because they can Clone)
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$transformer_type` - Transformer wrapper type name
/// * `$else_transformer_trait` - Transformer trait name
/// * `$predicate_conversion` - Predicate conversion method (into_arc or into_rc)
/// * `$extra_bounds` - Extra trait bounds
///
/// # Usage Examples
///
/// ```ignore
/// // Arc two-parameter Transformer
/// impl_shared_conditional_transformer!(
///     ArcConditionalTransformer<T, U>,
///     ArcTransformer,
///     Transformer,
///     into_arc,
///     Send + Sync + 'static
/// );
///
/// // Rc two-parameter Transformer
/// impl_shared_conditional_transformer!(
///     RcConditionalTransformer<T, U>,
///     RcTransformer,
///     Transformer,
///     into_rc,
///     'static
/// );
///
/// // Arc three-parameter BiTransformer
/// impl_shared_conditional_transformer!(
///     ArcConditionalBiTransformer<T, U, V>,
///     ArcBiTransformer,
///     BiTransformer,
///     into_arc,
///     Send + Sync + 'static
/// );
///
/// // Rc three-parameter BiTransformer
/// impl_shared_conditional_transformer!(
///     RcConditionalBiTransformer<T, U, V>,
///     RcBiTransformer,
///     BiTransformer,
///     into_rc,
///     'static
/// );
/// ```
///
macro_rules! impl_shared_conditional_transformer {
    // Two generic parameters - Transformer
    (
        $struct_name:ident < $t:ident, $r:ident >,
        $transformer_type:ident,
        $else_transformer_trait:ident,
        $predicate_conversion:ident,
        $($extra_bounds:tt)+
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
            #[allow(unused_mut)]
            pub fn or_else<F>(&self, mut else_transformer: F) -> $transformer_type<$t, $r>
            where
                $t: 'static,
                $r: 'static,
                F: $else_transformer_trait<$t, $r> + $($extra_bounds)+,
            {
                let predicate = self.predicate.clone();
                let mut then_transformer = self.transformer.clone();
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
        $predicate_conversion:ident,
        $($extra_bounds:tt)+
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
            #[allow(unused_mut)]
            pub fn or_else<F>(&self, mut else_transformer: F) -> $transformer_type<$t, $u, $r>
            where
                $t: 'static,
                $u: 'static,
                $r: 'static,
                F: $else_transformer_trait<$t, $u, $r> + $($extra_bounds)+,
            {
                let predicate = self.predicate.clone();
                let mut then_transformer = self.transformer.clone();
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
