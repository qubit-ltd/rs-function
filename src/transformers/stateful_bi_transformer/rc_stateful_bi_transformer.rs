/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `RcStatefulBiTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// RcStatefulBiTransformer - Rc<dyn FnMut(T, U) -> R>
// ============================================================================

/// RcStatefulBiTransformer - single-threaded bi-transformer wrapper
///
/// A single-threaded, clonable bi-transformer wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<dyn FnMut(T, U) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (each call consumes its
///   inputs)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct RcStatefulBiTransformer<T, U, R> {
    pub(super) function: Rc<RefCell<dyn FnMut(T, U) -> R>>,
    pub(super) name: Option<String>,
}

impl<T, U, R> RcStatefulBiTransformer<T, U, R> {
    impl_transformer_common_methods!(
        RcStatefulBiTransformer<T, U, R>,
        (FnMut(T, U) -> R + 'static),
        |f| Rc::new(RefCell::new(f))
    );

    impl_shared_transformer_methods!(
        RcStatefulBiTransformer<T, U, R>,
        RcConditionalStatefulBiTransformer,
        into_rc,
        StatefulTransformer,
        'static
    );
}

// Implement constant method for RcStatefulBiTransformer
impl_transformer_constant_method!(stateful RcStatefulBiTransformer<T, U, R>);

// Implement Debug and Display for RcStatefulBiTransformer
impl_transformer_debug_display!(RcStatefulBiTransformer<T, U, R>);

// Implement Clone for RcStatefulBiTransformer
impl_transformer_clone!(RcStatefulBiTransformer<T, U, R>);

// Implement StatefulBiTransformer trait for RcStatefulBiTransformer
impl<T, U, R> StatefulBiTransformer<T, U, R> for RcStatefulBiTransformer<T, U, R> {
    fn apply(&mut self, first: T, second: U) -> R {
        let mut self_fn = self.function.borrow_mut();
        self_fn(first, second)
    }

    // Generate all conversion methods using the unified macro
    impl_rc_conversions!(
        RcStatefulBiTransformer<T, U, R>,
        BoxStatefulBiTransformer,
        BoxBiTransformerOnce,
        FnMut(first: T, second: U) -> R
    );
}
