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
//! Defines the `BoxStatefulMutator` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 3. BoxMutator - Single Ownership Implementation
// ============================================================================

/// BoxMutator struct
///
/// A mutator implementation based on `Box<dyn FnMut(&mut T)>` for single
/// ownership scenarios. This is the simplest and most efficient mutator
/// type when sharing is not required.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, ownership moves on use
/// - **Zero Overhead**: No reference counting or locking
/// - **Mutable State**: Can modify captured environment via `FnMut`
/// - **Builder Pattern**: Method chaining consumes `self` naturally
/// - **Factory Methods**: Convenient constructors for common patterns
///
/// # Use Cases
///
/// Choose `BoxMutator` when:
/// - The mutator is used only once or in a linear flow
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
/// let mut mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
/// let mut value = 5;
/// mutator.apply(&mut value);
/// assert_eq!(value, 10);
/// ```
///
pub struct BoxStatefulMutator<T> {
    pub(super) function: Box<dyn FnMut(&mut T)>,
    pub(super) name: Option<String>,
}

impl<T> BoxStatefulMutator<T> {
    impl_mutator_common_methods!(BoxStatefulMutator<T>, (FnMut(&mut T) + 'static), |f| {
        Box::new(f)
    });

    // Generate box mutator methods (when, and_then, or_else, etc.)
    impl_box_mutator_methods!(
        BoxStatefulMutator<T>,
        BoxConditionalStatefulMutator,
        StatefulMutator
    );
}

impl<T> StatefulMutator<T> for BoxStatefulMutator<T> {
    fn apply(&mut self, value: &mut T) {
        (self.function)(value)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxStatefulMutator<T>,
        RcStatefulMutator,
        FnMut(&mut T),
        BoxMutatorOnce
    );
}

// Generate Debug and Display trait implementations
impl_mutator_debug_display!(BoxStatefulMutator<T>);
