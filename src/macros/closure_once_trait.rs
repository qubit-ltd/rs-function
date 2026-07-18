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
/// ```text
/// impl_closure_once_trait!(
///     ConsumerOnce<i32>,
///     accept,
///     BoxConsumerOnce,
///     FnOnce(value: &i32)
/// );
///
/// impl_closure_once_trait!(
///     FunctionOnce<i32, i32>,
///     apply,
///     BoxFunctionOnce,
///     FnOnce(input: &i32) -> i32
/// );
///
/// impl_closure_once_trait!(
///     BiFunctionOnce<i32, i32, i32>,
///     apply,
///     BoxBiFunctionOnce,
///     FnOnce(first: &i32, second: &i32) -> i32
/// );
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
          #[inline(always)]
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
