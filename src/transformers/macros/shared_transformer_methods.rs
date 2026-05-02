/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Shared Transformer Methods Macro
//!
//! Generates when and and_then method implementations for Arc/Rc-based Transformer
//!
//! Generates conditional execution when method and chaining and_then method
//! for Arc/Rc-based transformers that borrow &self (because Arc/Rc can be cloned).
//!
//! This macro supports both two-parameter and three-parameter transformers through
//! pattern matching on the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Two parameters: `ArcTransformer<T, U>`
//!   - Three parameters: `ArcBiTransformer<T, U, V>`
//! * `$conditional_type` - The conditional transformer type returned by `when`
//! * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
//! * `$chained_transformer_trait` - The name of the transformer trait that is
//!   chained after the execution of this transformer (e.g., Transformer,
//!   BiTransformer)
//! * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
//!
//! # All Macro Invocations
//!
//! | Transformer Type | Struct Signature | `$conditional_type` |
//! |------------------|------------------|----------------|
//! | **ArcTransformer** | `ArcTransformer<T, U>` | ArcConditionalTransformer |
//! | **RcTransformer** | `RcTransformer<T, U>` | RcConditionalTransformer |
//! | **ArcStatefulTransformer** | `ArcStatefulTransformer<T, U>` | ArcConditionalStatefulTransformer |
//! | **RcStatefulTransformer** | `RcStatefulTransformer<T, U>` | RcConditionalStatefulTransformer |
//! | **ArcBiTransformer** | `ArcBiTransformer<T, U, V>` | ArcConditionalBiTransformer |
//! | **RcBiTransformer** | `RcBiTransformer<T, U, V>` | RcConditionalBiTransformer |
//! | **ArcStatefulBiTransformer** | `ArcStatefulBiTransformer<T, U, V>` | ArcConditionalStatefulBiTransformer |
//! | **RcStatefulBiTransformer** | `RcStatefulBiTransformer<T, U, V>` | RcConditionalStatefulBiTransformer |
//!
//! | `$predicate_conversion` | `$chained_transformer_trait` | `$extra_bounds` |
//! |-------------------------|---------------------|----------------|
//! | into_arc | Transformer | Send + Sync + 'static |
//! | into_rc | Transformer | 'static |
//! | into_arc | StatefulTransformer | Send + Sync + 'static |
//! | into_rc | StatefulTransformer | 'static |
//! | into_arc | BiTransformer | Send + Sync + 'static |
//! | into_rc | BiTransformer | 'static |
//! | into_arc | StatefulBiTransformer | Send + Sync + 'static |
//! | into_rc | StatefulBiTransformer | 'static |
//!
//! # Examples
//!
//! ```ignore
//! // Two-parameter with Arc
//! impl_shared_transformer_methods!(
//!     ArcTransformer<T, U>,
//!     ArcConditionalTransformer,
//!     into_arc,
//!     Transformer,
//!     Send + Sync + 'static
//! );
//!
//! // Three-parameter with Rc
//! impl_shared_transformer_methods!(
//!     RcBiTransformer<T, U, V>,
//!     RcConditionalBiTransformer,
//!     into_rc,
//!     BiTransformer,
//!     'static
//! );
//! ```
//!

/// Generates when and and_then method implementations for Arc/Rc-based Transformer
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates conditional execution when method and chaining
/// and_then method for Arc/Rc-based transformers that borrow &self (because Arc/Rc
/// can be cloned).
///
/// This macro supports both two-parameter and three-parameter transformers through
/// pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Two parameters: `ArcTransformer<T, U>`
///   - Three parameters: `ArcBiTransformer<T, U, V>`
/// * `$conditional_type` - The conditional transformer type returned by `when`
/// * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
/// * `$chained_transformer_trait` - The name of the transformer trait that is
///   chained after the execution of this transformer (e.g., Transformer, BiTransformer)
/// * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
///
/// # All Macro Invocations
///
/// | Transformer Type | Struct Signature | `$conditional_type` | `$predicate_conversion` | `$chained_transformer_trait` | `$extra_bounds` |
/// |------------------|------------------|----------------|-------------------------|---------------------|----------------|
/// | **ArcTransformer** | `ArcTransformer<T, U>` | ArcConditionalTransformer | into_arc | Transformer | Send + Sync + 'static |
/// | **RcTransformer** | `RcTransformer<T, U>` | RcConditionalTransformer | into_rc | Transformer | 'static |
/// | **ArcStatefulTransformer** | `ArcStatefulTransformer<T, U>` | ArcConditionalStatefulTransformer | into_arc | StatefulTransformer | Send + Sync + 'static |
/// | **RcStatefulTransformer** | `RcStatefulTransformer<T, U>` | RcConditionalStatefulTransformer | into_rc | StatefulTransformer | 'static |
/// | **ArcBiTransformer** | `ArcBiTransformer<T, U, V>` | ArcConditionalBiTransformer | into_arc | BiTransformer | Send + Sync + 'static |
/// | **RcBiTransformer** | `RcBiTransformer<T, U, V>` | RcConditionalBiTransformer | into_rc | BiTransformer | 'static |
/// | **ArcStatefulBiTransformer** | `ArcStatefulBiTransformer<T, U, V>` | ArcConditionalStatefulBiTransformer | into_arc | StatefulBiTransformer | Send + Sync + 'static |
/// | **RcStatefulBiTransformer** | `RcStatefulBiTransformer<T, U, V>` | RcConditionalStatefulBiTransformer | into_rc | StatefulBiTransformer | 'static |
///
/// # Examples
///
/// ```ignore
/// // Two-parameter with Arc
/// impl_shared_transformer_methods!(
///     ArcTransformer<T, U>,
///     ArcConditionalTransformer,
///     into_arc,
///     Transformer,
///     Send + Sync + 'static
/// );
///
/// // Three-parameter with Rc
/// impl_shared_transformer_methods!(
///     RcBiTransformer<T, U, V>,
///     RcConditionalBiTransformer,
///     into_rc,
///     BiTransformer,
///     'static
/// );
/// ```
///
macro_rules! impl_shared_transformer_methods {
    // Two generic parameters
    (
        $struct_name:ident < $t:ident, $r:ident >,
        $conditional_type:ident,
        $predicate_conversion:ident,
        $chained_transformer_trait:ident,
        $($extra_bounds:tt)+
    ) => {
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $conditional_type<$t, $r>
        where
            $t: 'static,
            $r: 'static,
            P: Predicate<$t> + $($extra_bounds)+,
        {
            $conditional_type {
                transformer: self.clone(),
                predicate: predicate.$predicate_conversion(),
            }
        }

        #[allow(unused_mut)]
        #[inline]
        pub fn and_then<S, F>(&self, mut after: F) -> $struct_name<$t, S>
        where
            $t: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_transformer_trait<$r, S> + $($extra_bounds)+,
        {
            let mut before = self.clone();
            $struct_name::new(move |t| {
                let r = before.apply(t);
                after.apply(r)
            })
        }
    };

    // Three generic parameters
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        $conditional_type:ident,
        $predicate_conversion:ident,
        $chained_transformer_trait:ident,
        $($extra_bounds:tt)+
    ) => {
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $conditional_type<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            P: BiPredicate<$t, $u> + $($extra_bounds)+,
        {
            $conditional_type {
                transformer: self.clone(),
                predicate: predicate.$predicate_conversion(),
            }
        }

        #[allow(unused_mut)]
        #[inline]
        pub fn and_then<S, F>(&self, mut after: F) -> $struct_name<$t, $u, S>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_transformer_trait<$r, S> + $($extra_bounds)+,
        {
            let mut before = self.clone();
            $struct_name::new(move |t, u| {
                let mut r = before.apply(t, u);
                after.apply(r)
            })
        }
    };
}

pub(crate) use impl_shared_transformer_methods;
