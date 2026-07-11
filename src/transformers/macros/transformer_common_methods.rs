// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Transformer Common Methods Macro
//!
//! Generates common Transformer methods using shared implementations
//! (new, new_with_name, new_with_optional_name, name, set_name, identity)
//!
//! This macro uses `impl_common_new_methods` and `impl_common_name_methods`
//! to generate constructor and name management methods, plus a specialized
//! identity constructor for Transformer structs. This macro should be called
//! inside an impl block.
//!
//! The macro automatically detects the number of generic parameters and
//! generates the appropriate implementations for single-parameter or
//! two-parameter transformers.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$fn_trait_with_bounds` - Closure trait with complete bounds (e.g.,
//!   `Fn(&T) -> U + 'static`)
//! * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
//!
//! # Usage
//!
//! ```ignore
//! // Single generic parameter - Transformer
//! impl_transformer_common_methods!(
//!     BoxTransformer<T, U>,
//!     (Fn(&T) -> U + 'static),
//!     |f| Box::new(f)
//! );
//!
//! // Single generic parameter - StatefulTransformer
//! impl_transformer_common_methods!(
//!     ArcStatefulTransformer<T, U>,
//!     (FnMut(&T) -> U + Send + 'static),
//!     |f| Arc::new(Mutex::new(f))
//! );
//!
//! // Two generic parameters - BiTransformer
//! impl_transformer_common_methods!(
//!     BoxBiTransformer<T, U, V>,
//!     (Fn(&T, &U) -> V + 'static),
//!     |f| Box::new(f)
//! );
//! ```
//!
//! # Generated Methods
//!
//! * `new()` - Creates a new transformer
//! * `new_with_name()` - Creates a named transformer
//! * `new_with_optional_name()` - Creates a transformer with optional name
//! * `name()` - Gets the name of the transformer
//! * `set_name()` - Sets the name of the transformer
//! * `identity()` - Creates a transformer that returns the input unchanged

/// Generates common Transformer methods using shared implementations
/// (new, new_with_name, new_with_optional_name, name, set_name, identity)
///
/// This macro uses `impl_common_new_methods` and `impl_common_name_methods`
/// to generate constructor and name management methods, plus a specialized
/// identity constructor for Transformer structs. This macro should be used
/// inside an existing impl block for the target struct.
///
/// The macro automatically detects the number of generic parameters and
/// generates the appropriate implementations for single-parameter or
/// two-parameter transformers.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$fn_trait_with_bounds` - Closure trait with complete bounds (e.g.,
///   `Fn(&T) -> U + 'static`)
/// * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
///
/// # Usage
///
/// ```ignore
/// // Single generic parameter - Transformer
/// impl_transformer_common_methods!(
///     BoxTransformer<T, U>,
///     (Fn(&T) -> U + 'static),
///     |f| Box::new(f)
/// );
///
/// // Single generic parameter - StatefulTransformer
/// impl_transformer_common_methods!(
///     ArcStatefulTransformer<T, U>,
///     (FnMut(&T) -> U + Send + 'static),
///     |f| Arc::new(Mutex::new(f))
/// );
///
/// // Two generic parameters - BiTransformer
/// impl_transformer_common_methods!(
///     BoxBiTransformer<T, U, V>,
///     (Fn(&T, &U) -> V + 'static),
///     |f| Box::new(f)
/// );
/// ```
///
/// # Generated Methods
///
/// * `new()` - Creates a new transformer
/// * `new_with_name()` - Creates a named transformer
/// * `new_with_optional_name()` - Creates a transformer with optional name
/// * `name()` - Gets the name of the transformer
/// * `set_name()` - Sets the name of the transformer
/// * `identity()` - Creates a transformer that returns the input unchanged
macro_rules! impl_transformer_new_methods {
    (BoxTransformer<$t:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@one Transformer, $t, $r, ('static), |$f| $w); };
    (RcTransformer<$t:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@one Transformer, $t, $r, ('static), |$f| $w); };
    (ArcTransformer<$t:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@one Transformer, $t, $r, (Send + Sync + 'static), |$f| $w); };
    (BoxStatefulTransformer<$t:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@one_mut StatefulTransformer, $t, $r, ('static), |$f| $w); };
    (RcStatefulTransformer<$t:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@one_mut StatefulTransformer, $t, $r, ('static), |$f| $w); };
    (ArcStatefulTransformer<$t:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@one_mut StatefulTransformer, $t, $r, (Send + 'static), |$f| $w); };
    (BoxTransformerOnce<$t:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@one TransformerOnce, $t, $r, ('static), |$f| $w); };
    (BoxBiTransformer<$t:ident, $u:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@two BiTransformer, $t, $u, $r, ('static), |$f| $w); };
    (RcBiTransformer<$t:ident, $u:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@two BiTransformer, $t, $u, $r, ('static), |$f| $w); };
    (ArcBiTransformer<$t:ident, $u:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@two BiTransformer, $t, $u, $r, (Send + Sync + 'static), |$f| $w); };
    (BoxStatefulBiTransformer<$t:ident, $u:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@two_mut StatefulBiTransformer, $t, $u, $r, ('static), |$f| $w); };
    (RcStatefulBiTransformer<$t:ident, $u:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@two_mut StatefulBiTransformer, $t, $u, $r, ('static), |$f| $w); };
    (ArcStatefulBiTransformer<$t:ident, $u:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@two_mut StatefulBiTransformer, $t, $u, $r, (Send + 'static), |$f| $w); };
    (BoxBiTransformerOnce<$t:ident, $u:ident, $r:ident>, |$f:ident| $w:expr) => { $crate::transformers::macros::impl_transformer_new_methods!(@two BiTransformerOnce, $t, $u, $r, ('static), |$f| $w); };
    (@one $tr:ident, $t:ident, $r:ident, ($($b:tt)+), |$f:ident| $w:expr) => { crate::macros::impl_common_new_methods!(semantic ($tr<$t, $r> + $($b)+), |source| move |input: $t| source.apply(input), |$f| $w, "transformer"); };
    (@one_mut $tr:ident, $t:ident, $r:ident, ($($b:tt)+), |$f:ident| $w:expr) => { crate::macros::impl_common_new_methods!(semantic_mut ($tr<$t, $r> + $($b)+), |source| move |input: $t| source.apply(input), |$f| $w, "transformer"); };
    (@two $tr:ident, $t:ident, $u:ident, $r:ident, ($($b:tt)+), |$f:ident| $w:expr) => { crate::macros::impl_common_new_methods!(semantic ($tr<$t, $u, $r> + $($b)+), |source| move |first: $t, second: $u| source.apply(first, second), |$f| $w, "bi-transformer"); };
    (@two_mut $tr:ident, $t:ident, $u:ident, $r:ident, ($($b:tt)+), |$f:ident| $w:expr) => { crate::macros::impl_common_new_methods!(semantic_mut ($tr<$t, $u, $r> + $($b)+), |source| move |first: $t, second: $u| source.apply(first, second), |$f| $w, "bi-transformer"); };
}

macro_rules! impl_transformer_common_methods {
    // Single generic parameter - Transformer types
    (
        $struct_name:ident < $t:ident, $u:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        $crate::transformers::macros::impl_transformer_new_methods!($struct_name<$t, $u>, |$f| $wrapper_expr);

        crate::macros::impl_common_name_methods!("transformer");

        /// Creates an identity transformer.
        ///
        /// Creates a transformer that returns the input value unchanged. Useful for
        /// default values or placeholder implementations.
        ///
        /// # Returns
        ///
        /// Returns a new transformer instance that returns the input unchanged.
        #[inline]
        pub fn identity() -> $struct_name<$t, $t> {
            $struct_name::<$t, $t>::new(|t: $t| t)
        }
    };

    // Two generic parameters - BiTransformer types
    (
        $struct_name:ident < $t:ident, $u:ident, $v:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        $crate::transformers::macros::impl_transformer_new_methods!($struct_name<$t, $u, $v>, |$f| $wrapper_expr);

        crate::macros::impl_common_name_methods!("bi-transformer");
    };
}

pub(crate) use impl_transformer_common_methods;
pub(crate) use impl_transformer_new_methods;
