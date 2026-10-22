// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Transformer Clone Macro
//!
//! Generates `Clone` implementations for Transformer types.
//!
//! The macro supports Transformer structs with two or three type parameters.
//! It clones each struct's `function` and `metadata` fields using their
//! respective `Clone` implementations. For smart pointers such as `Arc` and
//! `Rc`, cloning the `function` field is a shallow clone.
//!
//! # Parameters
//!
//! * `$struct_name` - Transformer struct identifier
//! * `$t`, `$r` - Type parameter identifiers for a two-parameter Transformer
//! * `$t`, `$u`, `$r` - Type parameter identifiers for a three-parameter
//!   Transformer
//!
//! # Examples
//!
//! ```ignore
//! // For two type parameters
//! impl_transformer_clone!(ArcTransformer<T, U>);
//!
//! // For two type parameters with Rc
//! impl_transformer_clone!(RcTransformer<T, U>);
//!
//! // For three type parameters
//! impl_transformer_clone!(ArcBiTransformer<T, U, V>);
//!
//! // For three type parameters with Rc
//! impl_transformer_clone!(RcBiTransformer<T, U, V>);
//! ```

/// Generates a `Clone` implementation for a Transformer type.
///
/// This macro should be used at the top level (outside of any impl block) as
/// it expands to a complete `impl Clone for $struct_name` block. The target
/// struct must have `function` and `metadata` fields that implement `Clone`.
/// Cloning smart pointers such as `Arc` or `Rc` in the `function` field is
/// shallow.
///
/// # Parameters
///
/// * `$struct_name` - Transformer struct identifier
/// * `$t`, `$r` - Type parameter identifiers for a two-parameter Transformer
/// * `$t`, `$u`, `$r` - Type parameter identifiers for a three-parameter
///   Transformer
///
/// # Examples
///
/// ```ignore
/// // For two type parameters with Arc
/// impl_transformer_clone!(ArcTransformer<T, U>);
///
/// // For two type parameters with Rc
/// impl_transformer_clone!(RcTransformer<T, U>);
///
/// // For three type parameters with Arc
/// impl_transformer_clone!(ArcBiTransformer<T, U, V>);
///
/// // For three type parameters with Rc
/// impl_transformer_clone!(RcBiTransformer<T, U, V>);
/// ```
macro_rules! impl_transformer_clone {
    // Two generic parameters
    ($struct_name:ident < $t:ident, $r:ident >) => {
        impl<$t, $r> Clone for $struct_name<$t, $r> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    metadata: self.metadata.clone(),
                }
            }
        }
    };
    // Three generic parameters
    ($struct_name:ident < $t:ident, $u:ident, $r:ident >) => {
        impl<$t, $u, $r> Clone for $struct_name<$t, $u, $r> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    metadata: self.metadata.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_transformer_clone;
