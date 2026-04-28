/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxMutator` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 3. BoxMutator - Single Ownership Implementation
// ============================================================================

/// BoxMutator struct
///
/// A stateless mutator implementation based on `Box<dyn Fn(&mut T)>` for single
/// ownership scenarios. This is the simplest and most efficient mutator
/// type when sharing is not required.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, ownership moves on use
/// - **Zero Overhead**: No reference counting or locking
/// - **Stateless**: Cannot modify captured environment (uses `Fn` not `FnMut`)
/// - **Builder Pattern**: Method chaining consumes `self` naturally
/// - **Factory Methods**: Convenient constructors for common patterns
///
/// # Use Cases
///
/// Choose `BoxMutator` when:
/// - The mutator is used for stateless transformations
/// - Building pipelines where ownership naturally flows
/// - No need to share the mutator across contexts
/// - Performance is critical and no sharing overhead is acceptable
///
/// # Performance
///
/// `BoxMutator` has the best performance among the three mutator types:
/// - No reference counting overhead
/// - No lock acquisition or runtime borrow checking
/// - Direct function call through vtable
/// - Minimal memory footprint (single pointer)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Mutator, BoxMutator};
///
/// let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
/// let mut value = 5;
/// mutator.apply(&mut value);
/// assert_eq!(value, 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxMutator<T> {
    pub(super) function: Box<dyn Fn(&mut T)>,
    pub(super) name: Option<String>,
}

impl<T> BoxMutator<T> {
    // Generate common mutator methods (new, new_with_name, name, set_name, noop)
    impl_mutator_common_methods!(BoxMutator<T>, (Fn(&mut T) + 'static), |f| Box::new(f));

    // Generate box mutator methods (when, and_then, or_else, etc.)
    impl_box_mutator_methods!(BoxMutator<T>, BoxConditionalMutator, Mutator);
}

impl<T> Mutator<T> for BoxMutator<T> {
    fn apply(&self, value: &mut T) {
        (self.function)(value)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(BoxMutator<T>, RcMutator, Fn(&mut T), BoxMutatorOnce);
}

// Generate Debug and Display trait implementations
impl_mutator_debug_display!(BoxMutator<T>);
