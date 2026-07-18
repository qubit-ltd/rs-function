// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcStatefulFunction` public type.

use {
    crate::Predicate,
    crate::RcConditionalStatefulFunction,
};
use {
    crate::StatefulFunction,
    crate::functions::macros::impl_function_clone,
    crate::functions::macros::impl_function_common_methods,
    crate::functions::macros::impl_function_constant_method,
    crate::functions::macros::impl_function_debug_display,
    crate::functions::macros::impl_function_identity_method,
    crate::functions::macros::impl_shared_function_methods,
    std::cell::RefCell,
    std::rc::Rc,
};

// ============================================================================
// RcStatefulFunction - Rc<RefCell<dyn FnMut(&T) -> R>>
// ============================================================================

/// RcStatefulFunction - single-threaded function wrapper
///
/// A single-threaded, clonable function wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<RefCell<dyn FnMut(&T) -> R>>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (borrows its input each
///   time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
/// - **Statefulness**: Can modify internal state between calls
/// # Borrowing and reentrancy
///
/// Each call holds a mutable `RefCell` borrow while the user callback runs.
/// Synchronous re-entry through the same shared wrapper panics with a borrow
/// error. Mutations completed before a panic are not rolled back.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcStatefulFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: RcStatefulFn<T, R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

/// The erased callback representation used by this implementation.
type RcStatefulFn<T, R> = Rc<RefCell<dyn FnMut(&T) -> R>>;

impl<T, R> RcStatefulFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        RcStatefulFunction<T, R>,
        (FnMut(&T) -> R + 'static),
        |f| Rc::new(RefCell::new(f))
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        RcStatefulFunction<T, R>,
        RcConditionalStatefulFunction,
        RcPredicate,
        StatefulFunction,
        predicate_bounds = ('static),
        chained_bounds = ('static)
    );
}

// Generates: constant() method for RcStatefulFunction<T, R>
impl_function_constant_method!(RcStatefulFunction<T, R>, 'static);

// Generates: identity() method for RcStatefulFunction<T, T>
impl_function_identity_method!(RcStatefulFunction<T, T>);

// Generates: Clone implementation for RcStatefulFunction<T, R>
impl_function_clone!(RcStatefulFunction<T, R>);

// Generates: Debug and Display implementations for RcStatefulFunction<T, R>
impl_function_debug_display!(RcStatefulFunction<T, R>);

// Implement StatefulFunction trait for RcStatefulFunction<T, R>
impl<T, R> StatefulFunction<T, R> for RcStatefulFunction<T, R> {
    #[inline]
    fn apply(&mut self, t: &T) -> R {
        let mut function = self.function.borrow_mut();
        function(t)
    }
}
