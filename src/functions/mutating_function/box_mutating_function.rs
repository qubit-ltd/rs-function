// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxMutatingFunction` public type.

use crate::BoxConditionalMutatingFunction;
use crate::Function;
use crate::MutatingFunction;
use crate::Predicate;
use crate::functions::macros::impl_box_function_methods;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_debug_display;
use crate::functions::macros::impl_function_identity_method;

// =======================================================================
// 3. BoxMutatingFunction - Single Ownership Implementation
// =======================================================================

/// BoxMutatingFunction struct
///
/// A mutating function implementation based on `Box<dyn Fn(&mut T) -> R>`
/// for single ownership scenarios. This is the simplest and most efficient
/// mutating function type when sharing is not required.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, ownership moves on use
/// - **Runtime cost**: One heap allocation and dynamic dispatch; no reference
///   counting or locking
/// - **Shared-receiver calls**: Uses `Fn`, so invocation does not require `&mut
///   self`; interior mutability and external side effects remain possible
/// - **Builder Pattern**: Method chaining consumes `self` naturally
/// - **Factory Methods**: Convenient constructors for common patterns
///
/// # Use Cases
///
/// Choose `BoxMutatingFunction` when:
/// - The function is used for stateless operations
/// - Building pipelines where ownership naturally flows
/// - No need to share the function across contexts
/// - Performance is critical and no sharing overhead is acceptable
///
/// # Performance
///
/// `BoxMutatingFunction` has the best performance among the three function
/// types:
/// - No reference counting overhead
/// - No lock acquisition or runtime borrow checking
/// - Direct function call through vtable
/// - Minimal memory footprint (single pointer)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{MutatingFunction, BoxMutatingFunction};
///
/// let func = BoxMutatingFunction::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let mut value = 5;
/// assert_eq!(func.apply(&mut value), 10);
/// assert_eq!(value, 10);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxMutatingFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn Fn(&mut T) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, R> BoxMutatingFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        BoxMutatingFunction<T, R>,
        (Fn(&mut T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_box_function_methods!(
        BoxMutatingFunction<T, R>,
        BoxConditionalMutatingFunction,
        Function  // chains a non-mutating function after this mutating function
    );
}

// Generates: Debug and Display implementations for BoxMutatingFunction<T, R>
impl_function_debug_display!(BoxMutatingFunction<T, R>);

// Generates: identity() method for BoxMutatingFunction<T, T>
impl_function_identity_method!(BoxMutatingFunction<T, T>, mutating);

// Implement MutatingFunction trait for BoxMutatingFunction<T, R>
impl<T, R> MutatingFunction<T, R> for BoxMutatingFunction<T, R> {
    #[inline(always)]
    fn apply(&self, t: &mut T) -> R {
        (self.function)(t)
    }
}
