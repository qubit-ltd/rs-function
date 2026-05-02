/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! # Function Identity Macro
//!
//! Generates identity method implementation for function types.
//!
//! This macro generates an `impl` block that implements the `identity()` method
//! for function types that have identical input and output types (T -> T).
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name (e.g., `BoxFunction`, `RcFunction`, `ArcFunction`)
//! * `$t:ident` - The generic type parameter name (usually `T`)
//!
//! # Usage
//!
//! ```text
//! impl_function_identity!(BoxFunction<T>);
//! impl_function_identity!(RcFunction<T>);
//! impl_function_identity!(ArcFunction<T>);
//! impl_function_identity!(BoxBiFunction<T, U>);
//! impl_function_identity!(ArcBiFunction<T, U>);
//! ```
//!
//! # Generated Implementation
//!
//! For single-parameter functions, the macro generates:
//!
//! ```text
//! impl<T> BoxFunction<T, T> where T: Clone {
//!     pub fn identity() -> BoxFunction<T, T>;
//! }
//! ```
//!
//! For two-parameter functions, the macro generates:
//!
//! ```text
//! impl<T, U> ArcBiFunction<T, U, T> where T: Clone {
//!     pub fn identity() -> ArcBiFunction<T, U, T>;
//! }
//! ```
//!

/// Generates identity method implementation for function types.
///
/// This macro generates an `impl` block that implements the `identity()` method
/// for function types that have identical input and output types (T -> T).
///
/// # Parameters
///
/// * `$struct_name<$input_type, $output_type>` - The struct name with two generic type parameters
///   - Both generic parameters must be the same type identifier (e.g., `BoxFunction<T, T>`)
///   - Note: The macro caller must ensure $input_type and $output_type are identical
///
/// # Usage
///
/// ```text
/// impl_function_identity_method!(BoxFunction<T, T>);
/// impl_function_identity_method!(RcFunction<T, T>);
/// impl_function_identity_method!(ArcFunction<T, T>);
/// impl_function_identity_method!(BoxFunctionOnce<T, T>);
/// impl_function_identity_method!(BoxMutatingFunction<T, T>);
/// impl_function_identity_method!(BoxStatefulFunction<T, T>);
/// ```
///
macro_rules! impl_function_identity_method {
    ($struct_name:ident < $t:ident , $r:ident >) => {
        // Note: The caller must ensure $t and $r are the same identifier
        impl<$t> $struct_name<$t, $t> {
            /// Creates an identity function
            ///
            /// # Examples
            #[doc = concat!("/// ```rust\n/// use qubit_function::", stringify!($struct_name), ";\n///\n/// let identity = ", stringify!($struct_name), "::<i32, i32>::identity();\n/// assert_eq!(identity.apply(&42), 42);\n/// ```")]
            #[inline]
            pub fn identity() -> $struct_name<$t, $t>
            where
                $t: Clone,
            {
                $struct_name::new(|x: &$t| x.clone())
            }
        }
    };

    // Special case for mutating functions that take &mut T
    ($struct_name:ident < $t:ident , $r:ident >, mutating) => {
        // Note: The caller must ensure $t and $r are the same identifier
        impl<$t> $struct_name<$t, $t> {
            /// Creates an identity function
            ///
            /// # Examples
            #[doc = concat!("/// ```rust\n/// use qubit_function::", stringify!($struct_name), ";\n///\n/// let mut identity = ", stringify!($struct_name), "::<i32, i32>::identity();\n/// let mut value = 42;\n/// assert_eq!(identity.apply(&mut value), 42);\n/// ```")]
            #[inline]
            pub fn identity() -> $struct_name<$t, $t>
            where
                $t: Clone,
            {
                $struct_name::new(|x: &mut $t| x.clone())
            }
        }
    };
}

pub(crate) use impl_function_identity_method;
