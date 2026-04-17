/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Conditional Function Clone Macro
//!
//! Generates Clone trait implementation for Conditional Function types
//!
//! Generates Clone implementation for Conditional Function structs that have
//! `function` and `predicate` fields. Both fields are cloned using their
//! respective Clone implementations.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (two or three type parameters)
//!
//! # Examples
//!
//! ```rust
//! // Macro is crate-private and used internally by `qubit_function`.
//! //
//! // impl_conditional_function_clone!(ArcConditionalFunction<T, R>);
//! // impl_conditional_function_clone!(RcConditionalFunction<T, R>);
//! // impl_conditional_function_clone!(ArcConditionalBiFunction<T, U, R>);
//! // impl_conditional_function_clone!(RcConditionalBiFunction<T, U, R>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Clone trait implementation for Conditional Function types
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete `impl Clone for $struct_name` block. Generates
/// Clone implementation for Conditional Function structs that have `function`
/// and `predicate` fields. Both fields are cloned using their respective
/// Clone implementations.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$generic` - Generic parameter list (two or three type parameters)
///
/// # Examples
///
/// ```rust
/// // Macro is crate-private and used internally by `qubit_function`.
/// //
/// // impl_conditional_function_clone!(ArcConditionalFunction<T, R>);
/// // impl_conditional_function_clone!(RcConditionalFunction<T, R>);
/// // impl_conditional_function_clone!(ArcConditionalBiFunction<T, U, R>);
/// // impl_conditional_function_clone!(RcConditionalBiFunction<T, U, R>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_conditional_function_clone {
    // Two generic parameters - Function types
    ($struct_name:ident < $t:ident, $r:ident >) => {
        impl<$t, $r> Clone for $struct_name<$t, $r> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    predicate: self.predicate.clone(),
                }
            }
        }
    };
    // Three generic parameters - BiFunction types
    ($struct_name:ident < $t:ident, $u:ident, $r:ident >) => {
        impl<$t, $u, $r> Clone for $struct_name<$t, $u, $r> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    predicate: self.predicate.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_conditional_function_clone;
