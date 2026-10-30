// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxStatefulMutatingFunction` public type.

use crate::BoxConditionalStatefulMutatingFunction;
use crate::Function;
use crate::Predicate;
use crate::StatefulMutatingFunction;
use crate::functions::macros::impl_box_function_methods;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_debug_display;
use crate::functions::macros::impl_function_identity_method;

// =======================================================================
// 3. BoxStatefulMutatingFunction - Single Ownership Implementation
// =======================================================================

/// Owns a mutable callback that can update its input and retain state between
/// calls.
///
/// A stateful mutating function implementation based on
/// `Box<dyn FnMut(&mut T) -> R>` for single ownership scenarios. This is the
/// straightforward stateful mutating function type when sharing
/// is not required.
///
/// # Runtime Cost
///
/// The wrapper stores a type-erased callback and diagnostic metadata. Calls use
/// dynamic dispatch. Callback ownership does not add reference counting or a
/// shared-wrapper lock; an optional diagnostic name uses shared `Arc<str>`
/// storage. Memory usage and allocation depend on the capture and name. No
/// fixed pointer count or unconditional performance ranking is guaranteed.
///
/// # Type Parameters
///
/// - `T`: The mutable input type accepted by the callback.
/// - `R`: The result type returned by each callback invocation.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable; `apply` borrows `&mut self`
/// - **Stateful**: Can modify captured environment (uses `FnMut`)
/// - **Builder Pattern**: Method chaining consumes `self` naturally
/// - **Factory Methods**: Convenient constructors for common patterns
///
/// # Use Cases
///
/// Choose `BoxStatefulMutatingFunction` when:
/// - The function needs to maintain internal state
/// - Building pipelines where ownership naturally flows
/// - No need to share the function across contexts
/// - The callback does not need shared ownership
///
/// # Examples
///
/// ```rust
/// use qubit_function::BoxStatefulMutatingFunction;
/// use qubit_function::StatefulMutatingFunction;
///
/// let mut counter = {
///     let mut count = 0;
///     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
///         count += 1;
///         *x *= 2;
///         count
///     })
/// };
/// let mut value = 5;
/// assert_eq!(counter.apply(&mut value), 1);
/// assert_eq!(value, 10);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxStatefulMutatingFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn FnMut(&mut T) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, R> BoxStatefulMutatingFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        BoxStatefulMutatingFunction<T, R>,
        (FnMut(&mut T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when() and and_then()
    impl_box_function_methods!(
        BoxStatefulMutatingFunction<T, R>,
        BoxConditionalStatefulMutatingFunction,
        Function        // chains a non-mutating function after this mutating function
    );
}

// Generates: Debug and Display implementations for
// BoxStatefulMutatingFunction<T, R>
impl_function_debug_display!(BoxStatefulMutatingFunction<T, R>);

// Generates: identity() method for BoxStatefulMutatingFunction<T, T>
impl_function_identity_method!(BoxStatefulMutatingFunction<T, T>, mutating, StatefulMutatingFunction);

// Implement StatefulMutatingFunction trait for BoxStatefulMutatingFunction<T,
// R>
impl<T, R> StatefulMutatingFunction<T, R> for BoxStatefulMutatingFunction<T, R> {
    #[inline(always)]
    fn apply(&mut self, t: &mut T) -> R {
        (self.function)(t)
    }
}
