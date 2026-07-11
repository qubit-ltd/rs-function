// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Function Common Methods Macro
//!
//! Generates common Function methods (new, new_with_name, name,
//! set_name, identity)
//!
//! Generates constructor methods, name management methods and identity
//! constructor for Function structs. This macro should be called inside
//! an impl block.
//!
//! The macro automatically detects the number of generic parameters and
//! generates the appropriate implementations for single-parameter or
//! two-parameter functions.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$fn_trait_with_bounds` - Closure trait with complete bounds (e.g.,
//!   `Fn(&T) -> R + 'static`)
//! * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
//!
//! # Usage
//!
//! ```text
//! // Single generic parameter - Function
//! // impl_function_common_methods!(
//! //     BoxFunction<T, R>,
//! //     (Fn(&T) -> R + 'static),
//! //     |f| Box::new(f)
//! // );
//!
//! // Stateful function with state
//! // impl_function_common_methods!(
//! //     ArcStatefulFunction<T, R>,
//! //     (FnMut(&T) -> R + Send + 'static),
//! //     |f| Arc::new(Mutex::new(f))
//! // );
//!
//! // Two generic parameters - BiFunction (if applicable)
//! // impl_function_common_methods!(
//! //     BoxBiFunction<T, U, R>,
//! //     (Fn(&T, &U) -> R + 'static),
//! //     |f| Box::new(f)
//! // );
//! ```
//!
//! # Generated Methods
//!
//! * `new()` - Creates a new function
//! * `new_with_name()` - Creates a named function
//! * `name()` - Gets the name of the function
//! * `set_name()` - Sets the name of the function
//! * `identity()` - Creates an identity function

/// Generates common Function methods (new, new_with_name, name,
/// set_name, identity)
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates constructor methods, name management methods
/// and identity constructor for Function structs.
///
/// The macro automatically detects the number of generic parameters and
/// generates the appropriate implementations for single-parameter or
/// two-parameter functions.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$fn_trait_with_bounds` - Closure trait with complete bounds (e.g.,
///   `Fn(&T) -> R + 'static`)
/// * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
///
/// # Usage
///
/// ```text
/// // Single generic parameter - Function
/// // impl_function_common_methods!(
/// //     BoxFunction<T, R>,
/// //     (Fn(&T) -> R + 'static),
/// //     |f| Box::new(f)
/// // );
///
/// // Stateful function with state
/// // impl_function_common_methods!(
/// //     ArcStatefulFunction<T, R>,
/// //     (FnMut(&T) -> R + Send + 'static),
/// //     |f| Arc::new(Mutex::new(f))
/// // );
///
/// // Two generic parameters - BiFunction (if applicable)
/// // impl_function_common_methods!(
/// //     BoxBiFunction<T, U, R>,
/// //     (Fn(&T, &U) -> R + 'static),
/// //     |f| Box::new(f)
/// // );
/// ```
///
/// # Generated Methods
///
/// * `new()` - Creates a new function
/// * `new_with_name()` - Creates a named function
/// * `name()` - Gets the name of the function
/// * `set_name()` - Sets the name of the function
/// * `identity()` - Creates an identity function
macro_rules! impl_function_common_methods {
    ($name:ident<$t:ident, $r:ident>, (Fn(&$arg:ident) -> $out:ident + $($b:tt)+), |$f:ident| $w:expr) => { $crate::functions::macros::impl_function_common_methods!(@one Function, $t, $r, shared, ($($b)+), |$f| $w); };
    ($name:ident<$t:ident, $r:ident>, (Fn(&mut $arg:ident) -> $out:ident + $($b:tt)+), |$f:ident| $w:expr) => { $crate::functions::macros::impl_function_common_methods!(@one MutatingFunction, $t, $r, shared_mut, ($($b)+), |$f| $w); };
    ($name:ident<$t:ident, $r:ident>, (FnMut(&$arg:ident) -> $out:ident + $($b:tt)+), |$f:ident| $w:expr) => { $crate::functions::macros::impl_function_common_methods!(@one StatefulFunction, $t, $r, stateful, ($($b)+), |$f| $w); };
    ($name:ident<$t:ident, $r:ident>, (FnMut(&mut $arg:ident) -> $out:ident + $($b:tt)+), |$f:ident| $w:expr) => { $crate::functions::macros::impl_function_common_methods!(@one StatefulMutatingFunction, $t, $r, stateful_mut, ($($b)+), |$f| $w); };
    ($name:ident<$t:ident, $r:ident>, (FnOnce(&$arg:ident) -> $out:ident + $($b:tt)+), |$f:ident| $w:expr) => { $crate::functions::macros::impl_function_common_methods!(@one FunctionOnce, $t, $r, shared, ($($b)+), |$f| $w); };
    ($name:ident<$t:ident, $r:ident>, (FnOnce(&mut $arg:ident) -> $out:ident + $($b:tt)+), |$f:ident| $w:expr) => { $crate::functions::macros::impl_function_common_methods!(@one MutatingFunctionOnce, $t, $r, shared_mut, ($($b)+), |$f| $w); };
    ($name:ident<$t:ident, $u:ident, $r:ident>, (Fn(&$a:ident, &$barg:ident) -> $out:ident + $($b:tt)+), |$f:ident| $w:expr) => { $crate::functions::macros::impl_function_common_methods!(@two BiFunction, $t, $u, $r, shared, ($($b)+), |$f| $w); };
    ($name:ident<$t:ident, $u:ident, $r:ident>, (Fn(&mut $a:ident, &mut $barg:ident) -> $out:ident + $($b:tt)+), |$f:ident| $w:expr) => { $crate::functions::macros::impl_function_common_methods!(@two BiMutatingFunction, $t, $u, $r, shared_mut, ($($b)+), |$f| $w); };
    ($name:ident<$t:ident, $u:ident, $r:ident>, (FnOnce(&$a:ident, &$barg:ident) -> $out:ident + $($b:tt)+), |$f:ident| $w:expr) => { $crate::functions::macros::impl_function_common_methods!(@two BiFunctionOnce, $t, $u, $r, shared, ($($b)+), |$f| $w); };
    ($name:ident<$t:ident, $u:ident, $r:ident>, (FnOnce(&mut $a:ident, &mut $barg:ident) -> $out:ident + $($b:tt)+), |$f:ident| $w:expr) => { $crate::functions::macros::impl_function_common_methods!(@two BiMutatingFunctionOnce, $t, $u, $r, shared_mut, ($($b)+), |$f| $w); };
    (@one $tr:ident, $t:ident, $r:ident, shared, ($($b:tt)+), |$f:ident| $w:expr) => { crate::macros::impl_common_new_methods!(semantic ($tr<$t, $r> + $($b)+), |source| move |value: &$t| source.apply(value), |$f| $w, "function"); crate::macros::impl_common_name_methods!("function"); };
    (@one $tr:ident, $t:ident, $r:ident, shared_mut, ($($b:tt)+), |$f:ident| $w:expr) => { crate::macros::impl_common_new_methods!(semantic ($tr<$t, $r> + $($b)+), |source| move |value: &mut $t| source.apply(value), |$f| $w, "function"); crate::macros::impl_common_name_methods!("function"); };
    (@one $tr:ident, $t:ident, $r:ident, stateful, ($($b:tt)+), |$f:ident| $w:expr) => { crate::macros::impl_common_new_methods!(semantic_mut ($tr<$t, $r> + $($b)+), |source| move |value: &$t| source.apply(value), |$f| $w, "function"); crate::macros::impl_common_name_methods!("function"); };
    (@one $tr:ident, $t:ident, $r:ident, stateful_mut, ($($b:tt)+), |$f:ident| $w:expr) => { crate::macros::impl_common_new_methods!(semantic_mut ($tr<$t, $r> + $($b)+), |source| move |value: &mut $t| source.apply(value), |$f| $w, "function"); crate::macros::impl_common_name_methods!("function"); };
    (@two $tr:ident, $t:ident, $u:ident, $r:ident, shared, ($($b:tt)+), |$f:ident| $w:expr) => { crate::macros::impl_common_new_methods!(semantic ($tr<$t, $u, $r> + $($b)+), |source| move |first: &$t, second: &$u| source.apply(first, second), |$f| $w, "bi-function"); crate::macros::impl_common_name_methods!("bi-function"); };
    (@two $tr:ident, $t:ident, $u:ident, $r:ident, shared_mut, ($($b:tt)+), |$f:ident| $w:expr) => { crate::macros::impl_common_new_methods!(semantic ($tr<$t, $u, $r> + $($b)+), |source| move |first: &mut $t, second: &mut $u| source.apply(first, second), |$f| $w, "bi-function"); crate::macros::impl_common_name_methods!("bi-function"); };
}

pub(crate) use impl_function_common_methods;

macro_rules! impl_function_new_callback {
    ($_trait:ident, BoxMutatingFunction, $t:ident, |$v:ident| $body:block) => { BoxMutatingFunction::new(move |$v: &mut $t| $body) };
    ($_trait:ident, RcMutatingFunction, $t:ident, |$v:ident| $body:block) => { RcMutatingFunction::new(move |$v: &mut $t| $body) };
    ($_trait:ident, ArcMutatingFunction, $t:ident, |$v:ident| $body:block) => { ArcMutatingFunction::new(move |$v: &mut $t| $body) };
    ($_trait:ident, BoxStatefulMutatingFunction, $t:ident, |$v:ident| $body:block) => { BoxStatefulMutatingFunction::new(move |$v: &mut $t| $body) };
    ($_trait:ident, RcStatefulMutatingFunction, $t:ident, |$v:ident| $body:block) => { RcStatefulMutatingFunction::new(move |$v: &mut $t| $body) };
    ($_trait:ident, ArcStatefulMutatingFunction, $t:ident, |$v:ident| $body:block) => { ArcStatefulMutatingFunction::new(move |$v: &mut $t| $body) };
    ($_trait:ident, BoxMutatingFunctionOnce, $t:ident, |$v:ident| $body:block) => { BoxMutatingFunctionOnce::new(move |$v: &mut $t| $body) };
    ($_trait:ident, BoxBiFunction, $t:ident, $u:ident, |$a:ident, $b:ident| $body:block) => { BoxBiFunction::new(move |$a: &$t, $b: &$u| $body) };
    ($_trait:ident, RcBiFunction, $t:ident, $u:ident, |$a:ident, $b:ident| $body:block) => { RcBiFunction::new(move |$a: &$t, $b: &$u| $body) };
    ($_trait:ident, ArcBiFunction, $t:ident, $u:ident, |$a:ident, $b:ident| $body:block) => { ArcBiFunction::new(move |$a: &$t, $b: &$u| $body) };
    ($_trait:ident, BoxBiFunctionOnce, $t:ident, $u:ident, |$a:ident, $b:ident| $body:block) => { BoxBiFunctionOnce::new(move |$a: &$t, $b: &$u| $body) };
    ($_trait:ident, BoxBiMutatingFunction, $t:ident, $u:ident, |$a:ident, $b:ident| $body:block) => { BoxBiMutatingFunction::new(move |$a: &mut $t, $b: &mut $u| $body) };
    ($_trait:ident, RcBiMutatingFunction, $t:ident, $u:ident, |$a:ident, $b:ident| $body:block) => { RcBiMutatingFunction::new(move |$a: &mut $t, $b: &mut $u| $body) };
    ($_trait:ident, ArcBiMutatingFunction, $t:ident, $u:ident, |$a:ident, $b:ident| $body:block) => { ArcBiMutatingFunction::new(move |$a: &mut $t, $b: &mut $u| $body) };
    ($_trait:ident, BoxBiMutatingFunctionOnce, $t:ident, $u:ident, |$a:ident, $b:ident| $body:block) => { BoxBiMutatingFunctionOnce::new(move |$a: &mut $t, $b: &mut $u| $body) };
    (Function, $wrapper:ident, $t:ident, |$value:ident| $body:block) => { $wrapper::new(move |$value: &$t| $body) };
    (StatefulFunction, $wrapper:ident, $t:ident, |$value:ident| $body:block) => { $wrapper::new(move |$value: &$t| $body) };
    (FunctionOnce, $wrapper:ident, $t:ident, |$value:ident| $body:block) => { $wrapper::new(move |$value: &$t| $body) };
    (MutatingFunction, $wrapper:ident, $t:ident, |$value:ident| $body:block) => { $wrapper::new(move |$value: &mut $t| $body) };
    (StatefulMutatingFunction, $wrapper:ident, $t:ident, |$value:ident| $body:block) => { $wrapper::new(move |$value: &mut $t| $body) };
    (MutatingFunctionOnce, $wrapper:ident, $t:ident, |$value:ident| $body:block) => { $wrapper::new(move |$value: &mut $t| $body) };
    (BiFunction, $wrapper:ident, $t:ident, $u:ident, |$first:ident, $second:ident| $body:block) => { $wrapper::new(move |$first: &$t, $second: &$u| $body) };
    (BiFunctionOnce, $wrapper:ident, $t:ident, $u:ident, |$first:ident, $second:ident| $body:block) => { $wrapper::new(move |$first: &$t, $second: &$u| $body) };
    (BiMutatingFunction, $wrapper:ident, $t:ident, $u:ident, |$first:ident, $second:ident| $body:block) => { $wrapper::new(move |$first: &mut $t, $second: &mut $u| $body) };
    (BiMutatingFunctionOnce, $wrapper:ident, $t:ident, $u:ident, |$first:ident, $second:ident| $body:block) => { $wrapper::new(move |$first: &mut $t, $second: &mut $u| $body) };
}

pub(crate) use impl_function_new_callback;
