/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcMutator` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 5. ArcMutator - Thread-Safe Shared Ownership Implementation
// ============================================================================

/// ArcMutator struct
///
/// A stateless mutator implementation based on `Arc<dyn Fn(&mut T) + Send + Sync>`
/// for thread-safe shared ownership scenarios. This type allows the mutator
/// to be safely shared and used across multiple threads.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Stateless**: Cannot modify captured environment (uses `Fn` not `FnMut`)
/// - **Chainable**: Method chaining via `&self` (non-consuming)
///
/// # Use Cases
///
/// Choose `ArcMutator` when:
/// - The mutator needs to be shared across multiple threads for stateless operations
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
/// mutator.apply(&mut value);
/// assert_eq!(value, 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcMutator<T> {
    pub(super) function: ArcMutatorFn<T>,
    pub(super) name: Option<String>,
}

impl<T> ArcMutator<T> {
    // Generate common mutator methods (new, new_with_name, name, set_name, noop)
    impl_mutator_common_methods!(ArcMutator<T>, (Fn(&mut T) + Send + Sync + 'static), |f| {
        Arc::new(f)
    });

    // Generate shared mutator methods (when, and_then, or_else, conversions)
    impl_shared_mutator_methods!(
        ArcMutator<T>,
        ArcConditionalMutator,
        into_arc,
        Mutator,
        Send + Sync + 'static
    );
}

impl<T> Mutator<T> for ArcMutator<T> {
    fn apply(&self, value: &mut T) {
        (self.function)(value)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcMutator<T>,
        BoxMutator,
        RcMutator,
        BoxMutatorOnce,
        Fn(input: &mut T)
    );
}

// Generate Clone trait implementation for mutator
impl_mutator_clone!(ArcMutator<T>);

// Generate Debug and Display trait implementations
impl_mutator_debug_display!(ArcMutator<T>);

// ============================================================================
// 6. Implement Mutator trait for closures
// ============================================================================

impl_closure_trait!(
    Mutator<T>,
    apply,
    BoxMutatorOnce,
    Fn(value: &mut T)
);
