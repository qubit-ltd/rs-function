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
//! Defines the `RcMutator` public type.

use super::{
    BoxMutator,
    BoxMutatorOnce,
    Mutator,
    Predicate,
    Rc,
    RcConditionalMutator,
    RcMutatorFn,
    impl_mutator_clone,
    impl_mutator_common_methods,
    impl_mutator_debug_display,
    impl_rc_conversions,
    impl_shared_mutator_methods,
};

// ============================================================================
// 4. RcMutator - Single-Threaded Shared Ownership Implementation
// ============================================================================

/// RcMutator struct
///
/// A stateless mutator implementation based on `Rc<dyn Fn(&mut T)>` for
/// single-threaded shared ownership scenarios. This type allows multiple
/// references to the same mutator without the overhead of thread safety.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Stateless**: Cannot modify captured environment (uses `Fn` not `FnMut`)
/// - **Chainable**: Method chaining via `&self` (non-consuming)
/// - **Performance**: More efficient than `ArcMutator` (no locking)
///
/// # Use Cases
///
/// Choose `RcMutator` when:
/// - The mutator needs to be shared within a single thread for stateless operations
/// - Thread safety is not required
/// - Performance is important (avoiding lock overhead)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Mutator, RcMutator};
///
/// let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
/// let clone = mutator.clone();
///
/// let mut value = 5;
/// mutator.apply(&mut value);
/// assert_eq!(value, 10);
/// ```
///
pub struct RcMutator<T> {
    pub(super) function: RcMutatorFn<T>,
    pub(super) name: Option<String>,
}

impl<T> RcMutator<T> {
    // Generate common mutator methods (new, new_with_name, name, set_name, noop)
    impl_mutator_common_methods!(RcMutator<T>, (Fn(&mut T) + 'static), |f| Rc::new(f));

    // Generate shared mutator methods (when, and_then, or_else, conversions)
    impl_shared_mutator_methods!(
        RcMutator<T>,
        RcConditionalMutator,
        into_rc,
        Mutator,
        'static
    );
}

impl<T> Mutator<T> for RcMutator<T> {
    fn apply(&self, value: &mut T) {
        (self.function)(value)
    }

    // Generate all conversion methods using the unified macro
    impl_rc_conversions!(
        RcMutator<T>,
        BoxMutator,
        BoxMutatorOnce,
        Fn(t: &mut T)
    );
}

// Generate Clone trait implementation for mutator
impl_mutator_clone!(RcMutator<T>);

// Generate Debug and Display trait implementations
impl_mutator_debug_display!(RcMutator<T>);
