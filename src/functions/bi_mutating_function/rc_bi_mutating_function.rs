/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `RcBiMutatingFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// RcBiMutatingFunction - Rc<dyn Fn(&mut T, &mut U) -> R>
// ============================================================================

/// RcBiMutatingFunction - single-threaded bi-mutating-function wrapper
///
/// A single-threaded, clonable bi-mutating-function wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<dyn Fn(&mut T, &mut U) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (borrows inputs mutably each time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct RcBiMutatingFunction<T, U, R> {
    pub(super) function: Rc<dyn Fn(&mut T, &mut U) -> R>,
    pub(super) name: Option<String>,
}

impl<T, U, R> RcBiMutatingFunction<T, U, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        RcBiMutatingFunction<T, U, R>,
        (Fn(&mut T, &mut U) -> R + 'static),
        |f| Rc::new(f)
    );

    // Generate into_box(), into_rc(), into_fn(), into_once(), to_box(), to_rc(), to_fn(), to_once()
    impl_shared_function_methods!(
        RcBiMutatingFunction<T, U, R>,
        RcConditionalBiMutatingFunction,
        into_rc,
        MutatingFunction,
        'static
    );
}

// Implement BiMutatingFunction trait for RcBiMutatingFunction
impl<T, U, R> BiMutatingFunction<T, U, R> for RcBiMutatingFunction<T, U, R> {
    fn apply(&self, first: &mut T, second: &mut U) -> R {
        (self.function)(first, second)
    }

    // Generate into_box(), into_rc(), into_fn(), into_once(), to_box(), to_rc(), to_fn(), to_once()
    impl_rc_conversions!(
        RcBiMutatingFunction<T, U, R>,
        BoxBiMutatingFunction,
        BoxBiMutatingFunctionOnce,
        Fn(first: &mut T, second: &mut U) -> R
    );
}

// Implement constant method for RcBiMutatingFunction
impl_function_constant_method!(RcBiMutatingFunction<T, U, R>);

// Implement Debug and Display for RcBiMutatingFunction
impl_function_debug_display!(RcBiMutatingFunction<T, U, R>);

// Implement Clone for RcBiMutatingFunction
impl_function_clone!(RcBiMutatingFunction<T, U, R>);
