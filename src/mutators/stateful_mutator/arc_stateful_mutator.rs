/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `ArcStatefulMutator` public type.

use super::{
    Arc,
    ArcConditionalStatefulMutator,
    ArcMutMutatorFn,
    BoxMutatorOnce,
    BoxStatefulMutator,
    Mutex,
    Predicate,
    RcStatefulMutator,
    StatefulMutator,
    impl_arc_conversions,
    impl_closure_trait,
    impl_mutator_clone,
    impl_mutator_common_methods,
    impl_mutator_debug_display,
    impl_shared_mutator_methods,
};

// ============================================================================
// 4. ArcMutator - Thread-Safe Shared Ownership Implementation
// ============================================================================

/// ArcMutator struct
///
/// A mutator implementation based on `Arc<Mutex<dyn FnMut(&mut T) + Send>>`
/// for thread-safe shared ownership scenarios. This type allows the mutator
/// to be safely shared and used across multiple threads.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Interior Mutability**: Uses `Mutex` for safe concurrent mutations
/// - **Mutable State**: Can modify captured environment via `FnMut`
/// - **Chainable**: Method chaining via `&self` (non-consuming)
///
/// # Use Cases
///
/// Choose `ArcMutator` when:
/// - The mutator needs to be shared across multiple threads
/// - Concurrent task processing (e.g., thread pools)
/// - Thread safety is required (Send + Sync)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Mutator, ArcMutator};
///
/// let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
/// let clone = mutator.clone();
///
/// let mut value = 5;
/// let mut m = mutator;
/// m.apply(&mut value);
/// assert_eq!(value, 10);
/// ```
///
pub struct ArcStatefulMutator<T> {
    pub(super) function: ArcMutMutatorFn<T>,
    pub(super) name: Option<String>,
}

impl<T> ArcStatefulMutator<T> {
    impl_mutator_common_methods!(
        ArcStatefulMutator<T>,
        (FnMut(&mut T) + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    // Generate shared mutator methods (when, and_then, or_else, conversions)
    impl_shared_mutator_methods!(
        ArcStatefulMutator<T>,
        ArcConditionalStatefulMutator,
        into_arc,
        StatefulMutator,
        Send + Sync + 'static
    );
}

impl<T> StatefulMutator<T> for ArcStatefulMutator<T> {
    fn apply(&mut self, value: &mut T) {
        (self.function.lock())(value)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcStatefulMutator<T>,
        BoxStatefulMutator,
        RcStatefulMutator,
        BoxMutatorOnce,
        FnMut(input: &mut T)
    );
}

// Use macro to generate Clone implementation
impl_mutator_clone!(ArcStatefulMutator<T>);

// Generate Debug and Display trait implementations
impl_mutator_debug_display!(ArcStatefulMutator<T>);

// ============================================================================
// 5. Implement Mutator trait for closures
// ============================================================================

impl_closure_trait!(
    StatefulMutator<T>,
    apply,
    BoxMutatorOnce,
    FnMut(value: &mut T)
);
