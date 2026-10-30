// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxMutator` public type.

use crate::BoxConditionalMutator;
use crate::Mutator;
use crate::Predicate;
use crate::mutators::macros::impl_box_mutator_methods;
use crate::mutators::macros::impl_mutator_common_methods;
use crate::mutators::macros::impl_mutator_debug_display;

// ============================================================================
// 3. BoxMutator - Single Ownership Implementation
// ============================================================================

/// BoxMutator struct
///
/// A stateless mutator implementation based on `Box<dyn Fn(&mut T)>` for single
/// ownership scenarios. This is the straightforward mutator
/// type when sharing is not required.
///
/// # Type Parameters
///
/// - `T`: The value type that the mutator modifies through a mutable reference.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable; `apply` borrows `&self`
/// - **Shared-receiver calls**: Uses `Fn` and `apply(&self)`; interior
///   mutability and external side effects remain possible
/// - **Builder Pattern**: Method chaining consumes `self` naturally
/// - **Factory Methods**: Convenient constructors for common patterns
///
/// # Use Cases
///
/// Choose `BoxMutator` when:
/// - The mutator is used for stateless transformations
/// - Building pipelines where ownership naturally flows
/// - No need to share the mutator across contexts
/// - The callback does not need shared ownership
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
/// ```rust
/// use qubit_function::BoxMutator;
/// use qubit_function::Mutator;
///
/// let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
/// let mut value = 5;
/// mutator.apply(&mut value);
/// assert_eq!(value, 10);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxMutator<T> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn Fn(&mut T)>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T> BoxMutator<T> {
    // Generate common mutator methods (new, new_with_name, name, set_name,
    // noop)
    impl_mutator_common_methods!(BoxMutator<T>, (Fn(&mut T) + 'static), |f| { Box::new(f) });

    // Generate box mutator methods (when, and_then, or_else, etc.)
    impl_box_mutator_methods!(BoxMutator<T>, BoxConditionalMutator, Mutator);
}

impl<T> Mutator<T> for BoxMutator<T> {
    #[inline(always)]
    fn apply(&self, value: &mut T) {
        (self.function)(value)
    }
}

// Generate Debug and Display trait implementations
impl_mutator_debug_display!(BoxMutator<T>);
