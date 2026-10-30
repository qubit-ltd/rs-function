// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Transformer Common Methods Macro
//!
//! Provides macros that generate common constructor and name-management
//! methods for one-input and two-input transformers.

/// Generates constructor methods for supported transformer types.
///
/// The target type selects the underlying transformer trait and its bounds;
/// the wrapper expression adapts the supplied closure to the target's storage.
/// This helper is invoked inside the target type's `impl` block by
/// [`impl_transformer_common_methods`].
///
/// The closure-trait token group is retained in the invocation syntax, while
/// this macro selects bounds from the target type.
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

/// Generates constructor and name-management methods for a transformer type.
///
/// The macro selects an implementation based on whether the target transforms
/// one input or two. Invoke it inside the target type's `impl` block.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Transformer type and its generic parameters
/// * `$fn_trait_with_bounds` - Closure-trait bounds associated with the type
/// * `$wrapper_expr` - Expression that wraps the closure (using `f`)
///
/// # Usage
///
/// ```ignore
/// impl_transformer_common_methods!(
///     BoxTransformer<T, U>,
///     (Fn(&T) -> U + 'static),
///     |f| Box::new(f)
/// );
/// ```
///
/// The generated methods include `new`, `new_with_name`,
/// `new_with_optional_name`, `name`, and `set_name`.
macro_rules! impl_transformer_common_methods {
    // Single generic parameter - Transformer types
    (
        $struct_name:ident < $t:ident, $u:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        $crate::transformers::macros::impl_transformer_new_methods!($struct_name<$t, $u>, |$f| $wrapper_expr);

        crate::macros::impl_common_name_methods!("transformer");
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
