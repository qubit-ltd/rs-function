// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcMutatingFunction` public type.

use std::rc::Rc;

use crate::Function;
use crate::MutatingFunction;
use crate::Predicate;
use crate::RcConditionalMutatingFunction;
use crate::functions::macros::impl_function_clone;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_debug_display;
use crate::functions::macros::impl_function_identity_method;
use crate::functions::macros::impl_shared_function_methods;

// =======================================================================
// 4. RcMutatingFunction - Single-Threaded Shared Ownership
// =======================================================================

/// RcMutatingFunction struct
///
/// A mutating function implementation based on `Rc<dyn Fn(&mut T) -> R>` for
/// single-threaded shared ownership scenarios. This type allows multiple
/// references to the same function without the overhead of thread safety.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Shared-receiver calls**: Uses `Fn`, so invocation does not require `&mut
///   self`; interior mutability and external side effects remain possible
/// - **Chainable**: Method chaining via `&self` (non-consuming)
/// - **Ownership Cost**: `Rc` uses non-atomic reference counting; `Arc` uses
///   atomic reference counting, and neither wrapper locks callback invocation
///
/// # Use Cases
///
/// Choose `RcMutatingFunction` when:
/// - The function needs to be shared within a single thread for stateless
///   operations
/// - Thread safety is not required
/// - Single-threaded shared ownership without atomic reference counting is
///   preferred; benchmark the real workload when performance matters
///
/// # Examples
///
/// ```rust
/// use qubit_function::{MutatingFunction, RcMutatingFunction};
///
/// let func = RcMutatingFunction::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let clone = func.clone();
///
/// let mut value = 5;
/// assert_eq!(func.apply(&mut value), 10);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcMutatingFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: Rc<dyn Fn(&mut T) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, R> RcMutatingFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        RcMutatingFunction<T, R>,
        (Fn(&mut T) -> R + 'static),
        |f| Rc::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        RcMutatingFunction<T, R>,
        RcConditionalMutatingFunction,
        RcPredicate,
        Function,  // chains a non-mutating function after this mutating function
        predicate_bounds = ('static),
        chained_bounds = ('static)
    );
}

// Generates: Clone implementation for RcMutatingFunction<T, R>
impl_function_clone!(RcMutatingFunction<T, R>);

// Generates: Debug and Display implementations for RcMutatingFunction<T, R>
impl_function_debug_display!(RcMutatingFunction<T, R>);

// Generates: identity() method for RcMutatingFunction<T, T>
impl_function_identity_method!(RcMutatingFunction<T, T>, mutating);

impl<T, R> MutatingFunction<T, R> for RcMutatingFunction<T, R> {
    #[inline(always)]
    fn apply(&self, input: &mut T) -> R {
        (self.function)(input)
    }
}
