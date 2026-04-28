/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxMutatorOnce` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 2. BoxMutatorOnce - Single Ownership Implementation
// ============================================================================

/// BoxMutatorOnce struct
///
/// A one-time mutator implementation based on `Box<dyn FnOnce(&mut T)>` for
/// single ownership scenarios. This is the only MutatorOnce implementation type
/// because FnOnce conflicts with shared ownership semantics.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes self on use
/// - **Zero Overhead**: No reference counting or locking
/// - **Move Semantics**: Can capture and move variables
/// - **Method Chaining**: Compose multiple operations via `and_then`
///
/// # Use Cases
///
/// Choose `BoxMutatorOnce` when:
/// - Need to store FnOnce closures (with moved captured variables)
/// - One-time resource transfer operations
/// - Post-initialization callbacks
/// - Complex operations requiring ownership transfer
///
/// # Performance
///
/// `BoxMutatorOnce` performance characteristics:
/// - No reference counting overhead
/// - No lock acquisition or runtime borrow checking
/// - Direct function call through vtable
/// - Minimal memory footprint (single pointer)
///
/// # Why No Arc/Rc Variants?
///
/// FnOnce can only be called once, which conflicts with Arc/Rc shared ownership
/// semantics:
/// - Arc/Rc implies multiple owners might need to call
/// - FnOnce is consumed after calling, cannot be called again
/// - This semantic incompatibility makes Arc/Rc variants meaningless
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use qubit_function::{MutatorOnce, BoxMutatorOnce};
///
/// let data = vec![1, 2, 3];
/// let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
///     x.extend(data); // Move data
/// });
///
/// let mut target = vec![0];
/// mutator.apply(&mut target);
/// assert_eq!(target, vec![0, 1, 2, 3]);
/// ```
///
/// ## Method Chaining
///
/// ```rust
/// use qubit_function::{MutatorOnce, BoxMutatorOnce};
///
/// let data1 = vec![1, 2];
/// let data2 = vec![3, 4];
///
/// let chained = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
///     x.extend(data1);
/// })
/// .and_then(move |x: &mut Vec<i32>| {
///     x.extend(data2);
/// });
///
/// let mut target = vec![0];
/// chained.apply(&mut target);
/// assert_eq!(target, vec![0, 1, 2, 3, 4]);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxMutatorOnce<T> {
    pub(super) function: Box<dyn FnOnce(&mut T)>,
    pub(super) name: Option<String>,
}

impl<T> BoxMutatorOnce<T> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_mutator_common_methods!(BoxMutatorOnce<T>, (FnOnce(&mut T) + 'static), |f| Box::new(
        f
    ));

    // Generate box mutator methods (when, and_then, or_else, etc.)
    impl_box_mutator_methods!(BoxMutatorOnce<T>, BoxConditionalMutatorOnce, MutatorOnce);
}

impl<T> MutatorOnce<T> for BoxMutatorOnce<T> {
    fn apply(self, value: &mut T) {
        (self.function)(value)
    }

    impl_box_once_conversions!(BoxMutatorOnce<T>, MutatorOnce, FnOnce(&mut T));
}

// Generate Debug and Display trait implementations
impl_mutator_debug_display!(BoxMutatorOnce<T>);

// ============================================================================
// 3. Implement MutatorOnce trait for closures
// ============================================================================

// Implement MutatorOnce for all FnOnce(&mut T) using macro
impl_closure_once_trait!(
    MutatorOnce<T>,
    apply,
    BoxMutatorOnce,
    FnOnce(value: &mut T)
);
