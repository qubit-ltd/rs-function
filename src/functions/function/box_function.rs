/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxFunction - Box<dyn Fn(&T) -> R>
// ============================================================================

/// BoxFunction - function wrapper based on `Box<dyn Fn>`
///
/// A function wrapper that provides single ownership with reusable
/// transformation. The function consumes the input and can be called
/// multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(&T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Author
///
/// Haixing Hu
pub struct BoxFunction<T, R> {
    pub(super) function: Box<dyn Fn(&T) -> R>,
    pub(super) name: Option<String>,
}

impl<T, R> BoxFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        BoxFunction<T, R>,
        (Fn(&T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_box_function_methods!(
        BoxFunction<T, R>,
        BoxConditionalFunction,
        Function
    );
}

// Generates: constant() method for BoxFunction<T, R>
impl_function_constant_method!(BoxFunction<T, R>, 'static);

// Generates: identity() method for BoxFunction<T, T>
impl_function_identity_method!(BoxFunction<T, T>);

// Generates: Debug and Display implementations for BoxFunction<T, R>
impl_function_debug_display!(BoxFunction<T, R>);

// Implement Function trait for BoxFunction<T, R>
impl<T, R> Function<T, R> for BoxFunction<T, R> {
    fn apply(&self, t: &T) -> R {
        (self.function)(t)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxFunction<T, R>,
        RcFunction,
        Fn(&T) -> R,
        BoxFunctionOnce
    );
}
