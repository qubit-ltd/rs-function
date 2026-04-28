/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `RcStatefulTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// RcStatefulTransformer - Rc<RefCell<dyn FnMut(T) -> R>>
// ============================================================================

/// RcStatefulTransformer - single-threaded transformer wrapper
///
/// A single-threaded, clonable transformer wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<RefCell<dyn FnMut(T) -> R>>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (each call consumes
///   its input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
/// - **Statefulness**: Can modify internal state between calls
///
/// # Author
///
/// Haixing Hu
pub struct RcStatefulTransformer<T, R> {
    pub(super) function: Rc<RefCell<dyn FnMut(T) -> R>>,
    pub(super) name: Option<String>,
}

// Implement RcStatefulTransformer
impl<T, R> RcStatefulTransformer<T, R> {
    impl_transformer_common_methods!(
        RcStatefulTransformer<T, R>,
        (FnMut(T) -> R + 'static),
        |f| Rc::new(RefCell::new(f))
    );

    impl_shared_transformer_methods!(
        RcStatefulTransformer<T, R>,
        RcConditionalStatefulTransformer,
        into_rc,
        StatefulTransformer,
        'static
    );
}

// Implement constant method for RcStatefulTransformer
impl_transformer_constant_method!(stateful RcStatefulTransformer<T, R>);

// Implement Debug and Display for RcStatefulTransformer
impl_transformer_debug_display!(RcStatefulTransformer<T, R>);

// Implement Clone for RcStatefulTransformer
impl_transformer_clone!(RcStatefulTransformer<T, R>);

// Implement StatefulTransformer trait for RcStatefulTransformer
impl<T, R> StatefulTransformer<T, R> for RcStatefulTransformer<T, R> {
    fn apply(&mut self, input: T) -> R {
        let mut self_fn = self.function.borrow_mut();
        self_fn(input)
    }

    // Generate all conversion methods using the unified macro
    impl_rc_conversions!(
        RcStatefulTransformer<T, R>,
        BoxStatefulTransformer,
        BoxTransformerOnce,
        FnMut(input: T) -> R
    );
}
