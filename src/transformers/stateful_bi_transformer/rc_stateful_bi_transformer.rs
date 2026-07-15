// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcStatefulBiTransformer` public type.

use {
    crate::BiPredicate,
    crate::RcConditionalStatefulBiTransformer,
    crate::StatefulTransformer,
};
use {
    crate::StatefulBiTransformer,
    crate::transformers::macros::impl_shared_transformer_methods,
    crate::transformers::macros::impl_transformer_clone,
    crate::transformers::macros::impl_transformer_common_methods,
    crate::transformers::macros::impl_transformer_constant_method,
    crate::transformers::macros::impl_transformer_debug_display,
    std::cell::RefCell,
    std::rc::Rc,
};

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
/// # Borrowing and reentrancy
///
/// Each call holds a mutable `RefCell` borrow while the user callback runs.
/// Synchronous re-entry through the same shared wrapper panics with a borrow
/// error. Mutations completed before a panic are not rolled back.
pub struct RcStatefulBiTransformer<T, U, R> {
    pub(super) function: Rc<RefCell<dyn FnMut(T, U) -> R>>,
    pub(super) metadata: crate::callback_metadata::CallbackMetadata,
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
        RcBiPredicate,
        StatefulTransformer,
        predicate_bounds = ('static),
        chained_bounds = ('static)
    );
}

// Implement constant method for RcStatefulBiTransformer
impl_transformer_constant_method!(stateful RcStatefulBiTransformer<T, U, R>);

// Implement Debug and Display for RcStatefulBiTransformer
impl_transformer_debug_display!(RcStatefulBiTransformer<T, U, R>);

// Implement Clone for RcStatefulBiTransformer
impl_transformer_clone!(RcStatefulBiTransformer<T, U, R>);

// Implement StatefulBiTransformer trait for RcStatefulBiTransformer
impl<T, U, R> StatefulBiTransformer<T, U, R>
    for RcStatefulBiTransformer<T, U, R>
{
    fn apply(&mut self, first: T, second: U) -> R {
        let mut self_fn = self.function.borrow_mut();
        self_fn(first, second)
    }
}
