/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `RcStatefulMutator` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 3. RcMutator - Single-Threaded Shared Ownership Implementation
// ============================================================================

/// RcMutator struct
///
/// A mutator implementation based on `Rc<RefCell<dyn FnMut(&mut T)>>` for
/// single-threaded shared ownership scenarios. This type allows multiple
/// references to the same mutator without the overhead of thread safety.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Interior Mutability**: Uses `RefCell` for runtime borrow checking
/// - **Mutable State**: Can modify captured environment via `FnMut`
/// - **Chainable**: Method chaining via `&self` (non-consuming)
/// - **Performance**: More efficient than `ArcMutator` (no locking)
///
/// # Use Cases
///
/// Choose `RcMutator` when:
/// - The mutator needs to be shared within a single thread
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
/// let mut m = mutator;
/// m.apply(&mut value);
/// assert_eq!(value, 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcStatefulMutator<T> {
    pub(super) function: RcMutMutatorFn<T>,
    pub(super) name: Option<String>,
}

impl<T> RcStatefulMutator<T> {
    impl_mutator_common_methods!(
        RcStatefulMutator<T>,
        (FnMut(&mut T) + 'static),
        |f| Rc::new(RefCell::new(f))
    );

    // Generate shared mutator methods (when, and_then, or_else, conversions)
    impl_shared_mutator_methods!(
        RcStatefulMutator<T>,
        RcConditionalStatefulMutator,
        into_rc,
        StatefulMutator,
        'static
    );
}

impl<T> StatefulMutator<T> for RcStatefulMutator<T> {
    fn apply(&mut self, value: &mut T) {
        (self.function.borrow_mut())(value)
    }

    // Generate all conversion methods using the unified macro
    impl_rc_conversions!(
        RcStatefulMutator<T>,
        BoxStatefulMutator,
        BoxMutatorOnce,
        FnMut(t: &mut T)
    );
}

// Use macro to generate Clone implementation
impl_mutator_clone!(RcStatefulMutator<T>);

// Generate Debug and Display trait implementations
impl_mutator_debug_display!(RcStatefulMutator<T>);
