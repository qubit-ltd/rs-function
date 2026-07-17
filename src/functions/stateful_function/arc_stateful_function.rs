// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcStatefulFunction` public type.

use {
    crate::ArcConditionalStatefulFunction,
    crate::Predicate,
};
use {
    crate::StatefulFunction,
    crate::functions::macros::impl_function_clone,
    crate::functions::macros::impl_function_common_methods,
    crate::functions::macros::impl_function_constant_method,
    crate::functions::macros::impl_function_debug_display,
    crate::functions::macros::impl_function_identity_method,
    crate::functions::macros::impl_shared_function_methods,
    parking_lot::Mutex,
    std::sync::Arc,
};

// ============================================================================
// ArcStatefulFunction - Arc<Mutex<dyn FnMut(&T) -> R + Send>>
// ============================================================================

/// ArcStatefulFunction - thread-safe function wrapper
///
/// A thread-safe, clonable function wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads
/// while maintaining internal state.
///
/// # Features
///
/// - **Based on**: `Arc<Mutex<dyn FnMut(&T) -> R + Send>>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (borrows its input each
///   time)
/// - **Thread Safety**: Thread-safe (`Send` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
/// - **Statefulness**: Can modify internal state between calls
///
/// # Locking and reentrancy
///
/// Each call acquires a `parking_lot::Mutex` and holds it while the user
/// callback runs. Synchronous re-entry through the same shared state
/// deadlocks. The mutex is not poisoned after a panic, and mutations completed
/// before a panic are not rolled back.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcStatefulFunction<T, R> {
    pub(super) function: ArcStatefulFn<T, R>,
    pub(super) metadata: crate::callback_metadata::CallbackMetadata,
}

type ArcStatefulFn<T, R> = Arc<Mutex<dyn FnMut(&T) -> R + Send + 'static>>;

impl<T, R> ArcStatefulFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        ArcStatefulFunction<T, R>,
        (FnMut(&T) -> R + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        ArcStatefulFunction<T, R>,
        ArcConditionalStatefulFunction,
        ArcPredicate,
        StatefulFunction,
        predicate_bounds = (Send + Sync + 'static),
        chained_bounds = (Send + 'static)
    );
}

// Generates: constant() method for ArcStatefulFunction<T, R>
impl_function_constant_method!(ArcStatefulFunction<T, R>, Send + 'static);

// Generates: identity() method for ArcStatefulFunction<T, T>
impl_function_identity_method!(ArcStatefulFunction<T, T>);

// Generates: Clone implementation for ArcStatefulFunction<T, R>
impl_function_clone!(ArcStatefulFunction<T, R>);

// Generates: Debug and Display implementations for ArcStatefulFunction<T, R>
impl_function_debug_display!(ArcStatefulFunction<T, R>);

// Implement StatefulFunction trait for ArcStatefulFunction<T, R>
impl<T, R> StatefulFunction<T, R> for ArcStatefulFunction<T, R> {
    fn apply(&mut self, t: &T) -> R {
        let mut function = self.function.lock();
        function(t)
    }
}
