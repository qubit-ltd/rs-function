/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Conditional Transformer Clone Macro
//!
//! Generates Clone trait implementation for Conditional Transformer types
//!
//! Generates Clone implementation for Conditional Transformer structs that have
//! `transformer` and `predicate` fields. Both fields are cloned using their
//! respective Clone implementations.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (two or three type parameters)
//!
//! # Examples
//!
//! ```ignore
//! // For two type parameters
//! impl_conditional_transformer_clone!(ArcConditionalTransformer<T, U>);
//! impl_conditional_transformer_clone!(RcConditionalTransformer<T, U>);
//!
//! // For three type parameters
//! impl_conditional_transformer_clone!(ArcConditionalBiTransformer<T, U, V>);
//! impl_conditional_transformer_clone!(RcConditionalBiTransformer<T, U, V>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Clone trait implementation for Conditional Transformer types
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete `impl Clone for $struct_name` block. Generates
/// Clone implementation for Conditional Transformer structs that have `transformer`
/// and `predicate` fields. Both fields are cloned using their respective
/// Clone implementations.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$t` - Generic parameter list (two or three type parameters)
/// * `$r` - Generic parameter list (two or three type parameters)
///
/// # Examples
///
/// ```ignore
/// // For two type parameters
/// impl_conditional_transformer_clone!(ArcConditionalTransformer<T, U>);
/// impl_conditional_transformer_clone!(RcConditionalTransformer<T, U>);
///
/// // For three type parameters
/// impl_conditional_transformer_clone!(ArcConditionalBiTransformer<T, U, V>);
/// impl_conditional_transformer_clone!(RcConditionalBiTransformer<T, U, V>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_conditional_transformer_clone {
    // Two generic parameters
    ($struct_name:ident < $t:ident, $r:ident >) => {
        impl<$t, $r> Clone for $struct_name<$t, $r> {
            fn clone(&self) -> Self {
                Self {
                    transformer: self.transformer.clone(),
                    predicate: self.predicate.clone(),
                }
            }
        }
    };
    // Three generic parameters
    ($struct_name:ident < $t:ident, $u:ident, $r:ident >) => {
        impl<$t, $u, $r> Clone for $struct_name<$t, $u, $r> {
            fn clone(&self) -> Self {
                Self {
                    transformer: self.transformer.clone(),
                    predicate: self.predicate.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_conditional_transformer_clone;
