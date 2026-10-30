// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Closure Once Trait Implementation Macro
//!
//! This module provides the `impl_closure_once_trait!` macro for implementing
//! closure-based once traits.
//!
//! ## Overview
//!
//! The macro generates the semantic trait implementation for closures with
//! once semantics. It automatically infers the implementation from the
//! function signature and trait name.
//!
//! ## Generated Method
//!
//! - Core semantic method: Direct delegation to the underlying closure
//!
//! ## Usage
//!
//! The macro is typically used beside trait definitions to provide consistent
//! closure implementations across different once traits.

/// Implements a semantic once trait for closures.
///
/// This macro generates the core semantic method for once traits implemented
/// by closures. It automatically infers everything from the function signature
/// and trait name.
///
/// # Parameters
///
/// * `$trait_name<$(generics),*>` - Full trait name with generics (e.g.,
///   `ConsumerOnce<T>`, `BiFunctionOnce<T, U, R>`)
/// * `$method_name` - Core method name (e.g., `accept`, `apply`)
/// * `$box_type` - Box wrapper type (e.g., `BoxConsumerOnce`,
///   `BoxBiFunctionOnce`)
/// * `$fn_trait` - Function signature (e.g., `FnOnce(value: &T)`,
///   `FnOnce(first: &T, second: &U) -> R`)
///
/// # Generated implementation
///
/// ```text
/// impl<F> ConsumerOnce<i32> for F
/// where
///     F: FnOnce(&i32),
/// {
///     fn accept(self, value: &i32);
/// }
/// ```
///
/// # Examples
///
/// These are crate-internal expansion forms. Identifiers in the trait's type
/// parameter list are declared as generics; they are not concrete argument
/// types. This `text` example illustrates macro syntax and is not a downstream
/// callable API.
///
/// ```text
/// impl_closure_once_trait!(ConsumerOnce<T>, accept, BoxConsumerOnce, FnOnce(value: &T));
/// impl_closure_once_trait!(FunctionOnce<T, R>, apply, BoxFunctionOnce, FnOnce(input: &T) -> R);
/// impl_closure_once_trait!(BiFunctionOnce<T, U, R>, apply, BoxBiFunctionOnce, FnOnce(first: &T, second: &U) -> R);
/// ```
macro_rules! impl_closure_once_trait {
    // ==================== Internal Implementation ====================

    // Core implementation: Generate complete trait implementation
    (
        @impl
        $trait_name:ident < $($generics:ident),* >,
        $method_name:ident,
        $box_type:ident,
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        impl<F, $($generics),*> $trait_name<$($generics),*> for F
        where
            F: FnOnce($($arg_ty),*) $(-> $ret)?,
        {
            // Core method: Direct closure call
            #[inline]
            fn $method_name(self, $($arg : $arg_ty),*) $(-> $ret)? {
                self($($arg),*)
            }
        }
    };

    // ==================== Public Interface ====================

    // No return value version
    (
        $trait_name:ident < $($generics:ident),* >,
        $method_name:ident,
        $box_type:ident,
        FnOnce($($arg:ident : $arg_ty:ty),*)
    ) => {
        impl_closure_once_trait!(
            @impl
            $trait_name<$($generics),*>,
            $method_name,
            $box_type,
            ($($arg : $arg_ty),*)
        );
    };

    // With return value version
    (
        $trait_name:ident < $($generics:ident),* >,
        $method_name:ident,
        $box_type:ident,
        FnOnce($($arg:ident : $arg_ty:ty),*) -> $ret:ty
    ) => {
        impl_closure_once_trait!(
            @impl
            $trait_name<$($generics),*>,
            $method_name,
            $box_type,
            ($($arg : $arg_ty),*) -> $ret
        );
    };
}

pub(crate) use impl_closure_once_trait;
