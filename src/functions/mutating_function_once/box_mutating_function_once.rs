// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxMutatingFunctionOnce` public type.

use crate::BoxConditionalMutatingFunctionOnce;
use crate::FunctionOnce;
use crate::MutatingFunctionOnce;
use crate::Predicate;
use crate::functions::macros::impl_box_function_methods;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_debug_display;
use crate::functions::macros::impl_function_identity_method;
use crate::macros::impl_closure_once_trait;

// =======================================================================
// 2. BoxMutatingFunctionOnce - Single Ownership Implementation
// =======================================================================

/// Owns a one-time mutating callback behind a heap-allocated trait object.
///
/// The wrapper provides single ownership for `FnOnce(&mut T) -> R` callbacks,
/// allowing them to move captured values and return a result when invoked.
/// Shared `Arc` and `Rc` variants are not provided because a one-time callback
/// cannot be invoked by multiple owners.
///
/// # Type Parameters
///
/// * `T` - The mutable input type passed to the callback.
/// * `R` - The result type returned by the callback.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable; `apply` consumes `self`
/// - **Move Semantics**: Can capture and move variables
/// - **Method Chaining**: Compose multiple operations via `and_then`
/// - **Returns Results**: Unlike MutatorOnce, returns information
///
/// # Use Cases
///
/// Choose `BoxMutatingFunctionOnce` when:
/// - Need to store FnOnce closures (with moved captured variables)
/// - One-time resource transfer operations with results
/// - Post-initialization callbacks that return status
/// - Complex operations requiring ownership transfer and results
/// # Why No Arc/Rc Variants?
///
/// FnOnce can only be called once, which conflicts with Arc/Rc shared
/// ownership semantics:
/// - Arc/Rc implies multiple owners might need to call
/// - FnOnce is consumed after calling, cannot be called again
/// - These semantics make Arc/Rc variants meaningless
///
/// # Runtime Cost
///
/// The wrapper stores a type-erased callback and diagnostic metadata. Calls use
/// dynamic dispatch. Callback ownership does not add reference counting or a
/// shared-wrapper lock; an optional diagnostic name uses shared `Arc<str>`
/// storage. Memory usage and allocation depend on the capture and name. No
/// fixed pointer count or unconditional performance ranking is guaranteed.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use qubit_function::{MutatingFunctionOnce, BoxMutatingFunctionOnce};
///
/// let data = vec![1, 2, 3];
/// let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
///     let old_len = x.len();
///     x.extend(data); // Move data
///     old_len
/// });
///
/// let mut target = vec![0];
/// let old_len = func.apply(&mut target);
/// assert_eq!(old_len, 1);
/// assert_eq!(target, vec![0, 1, 2, 3]);
/// ```
///
/// ## Method Chaining
///
/// ```rust
/// # {
/// use qubit_function::{MutatingFunctionOnce, BoxMutatingFunctionOnce};
///
/// let data1 = vec![1, 2];
/// let additional_len = 2;
///
/// let chained = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
///     x.extend(data1);
///     x.len()
/// })
/// .and_then(move |len: &usize| len + additional_len);
///
/// let mut target = vec![0];
/// let final_len = chained.apply(&mut target);
/// assert_eq!(final_len, 5);
/// assert_eq!(target, vec![0, 1, 2]);
/// # }
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxMutatingFunctionOnce<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn FnOnce(&mut T) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, R> BoxMutatingFunctionOnce<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        BoxMutatingFunctionOnce<T, R>,
        (FnOnce(&mut T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_box_function_methods!(
        BoxMutatingFunctionOnce<T, R>,
        BoxConditionalMutatingFunctionOnce,
        FunctionOnce    // chains a non-mutating function after this mutating function
    );
}

impl<T, R> MutatingFunctionOnce<T, R> for BoxMutatingFunctionOnce<T, R> {
    #[inline(always)]
    fn apply(self, input: &mut T) -> R {
        (self.function)(input)
    }
}

// Generates: identity() method for BoxMutatingFunctionOnce<T, T>
impl_function_identity_method!(BoxMutatingFunctionOnce<T, T>, mutating, MutatingFunctionOnce);

// Generates: Debug and Display implementations for BoxMutatingFunctionOnce<T,
// R>
impl_function_debug_display!(BoxMutatingFunctionOnce<T, R>);

// =======================================================================
// 3. Implement MutatingFunctionOnce trait for closures
// =======================================================================

// Implement MutatingFunctionOnce for all FnOnce(&mut T) -> R using macro
impl_closure_once_trait!(
    MutatingFunctionOnce<T, R>,
    apply,
    BoxMutatingFunctionOnce,
    FnOnce(input: &mut T) -> R
);
