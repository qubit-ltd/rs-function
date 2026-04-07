/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
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
//! # Author
//!
//! Haixing Hu

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
/// # Author
///
/// Haixing Hu
macro_rules! impl_box_conditional_transformer {
    // Two generic parameters - Transformer
    (
        $struct_name:ident<$t:ident, $r:ident>,
        $transformer_type:ident,
        $transformer_trait:ident
    ) => {
        impl<$t, $r> $struct_name<$t, $r>
        where
            $t: 'static,
            $r: 'static,
        {
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
            #[allow(unused_mut)]
            pub fn or_else<F>(self, mut else_transformer: F) -> $transformer_type<$t, $r>
            where
                F: $transformer_trait<$t, $r> + 'static,
            {
                let predicate = self.predicate;
                let mut then_transformer = self.transformer;
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
        impl<$t, $u, $r> $struct_name<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
        {
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
            #[allow(unused_mut)]
            pub fn or_else<F>(self, mut else_transformer: F) -> $transformer_type<$t, $u, $r>
            where
                F: $transformer_trait<$t, $u, $r> + 'static,
            {
                let predicate = self.predicate;
                let mut then_transformer = self.transformer;
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
