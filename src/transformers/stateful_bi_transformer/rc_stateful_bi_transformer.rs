// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcStatefulBiTransformer` public type.

use std::cell::RefCell;
use std::rc::Rc;

use crate::BiPredicate;
use crate::RcConditionalStatefulBiTransformer;
use crate::StatefulBiTransformer;
use crate::StatefulTransformer;
use crate::transformers::macros::impl_shared_transformer_methods;
use crate::transformers::macros::impl_transformer_clone;
use crate::transformers::macros::impl_transformer_common_methods;
use crate::transformers::macros::impl_transformer_constant_method;
use crate::transformers::macros::impl_transformer_debug_display;

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
/// # Panics
///
/// A call panics if the callback synchronously re-enters the same wrapper
/// while its mutable `RefCell` borrow is active. Mutations completed before a
/// panic are not rolled back.
///
/// # Type Parameters
///
/// * `T` - Type of the first input consumed by the callback.
/// * `U` - Type of the second input consumed by the callback.
/// * `R` - Type of the value returned by the callback.
///
/// # Examples
///
/// Clones share the callback's mutable state:
///
/// ```
/// use qubit_function::RcStatefulBiTransformer;
/// use qubit_function::StatefulBiTransformer;
///
/// let mut total = 0;
/// let mut add_and_accumulate = RcStatefulBiTransformer::new(
///     move |first: i32, second: i32| {
///         total += first + second;
///         total
///     },
/// );
/// let mut shared_clone = add_and_accumulate.clone();
///
/// assert_eq!(add_and_accumulate.apply(2, 3), 5);
/// assert_eq!(shared_clone.apply(1, 4), 10);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcStatefulBiTransformer<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: Rc<RefCell<dyn FnMut(T, U) -> R>>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, U, R> RcStatefulBiTransformer<T, U, R> {
    impl_transformer_common_methods!(
        RcStatefulBiTransformer<T, U, R>,
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
impl<T, U, R> StatefulBiTransformer<T, U, R> for RcStatefulBiTransformer<T, U, R> {
    #[inline]
    fn apply(&mut self, first: T, second: U) -> R {
        let mut self_fn = self.function.borrow_mut();
        self_fn(first, second)
    }
}
