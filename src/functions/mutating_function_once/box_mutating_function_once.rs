/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxMutatingFunctionOnce` public type.

#![allow(unused_imports)]

use super::*;

// =======================================================================
// 2. BoxMutatingFunctionOnce - Single Ownership Implementation
// =======================================================================

/// BoxMutatingFunctionOnce struct
///
/// A one-time mutating function implementation based on
/// `Box<dyn FnOnce(&mut T) -> R>` for single ownership scenarios. This is
/// the only MutatingFunctionOnce implementation type because FnOnce
/// conflicts with shared ownership semantics.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes self on use
/// - **Zero Overhead**: No reference counting or locking
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
///
/// # Performance
///
/// `BoxMutatingFunctionOnce` performance characteristics:
/// - No reference counting overhead
/// - No lock acquisition or runtime borrow checking
/// - Direct function call through vtable
/// - Minimal memory footprint (single pointer)
///
/// # Why No Arc/Rc Variants?
///
/// FnOnce can only be called once, which conflicts with Arc/Rc shared
/// ownership semantics:
/// - Arc/Rc implies multiple owners might need to call
/// - FnOnce is consumed after calling, cannot be called again
/// - This semantic incompatibility makes Arc/Rc variants meaningless
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
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxMutatingFunctionOnce<T, R> {
    pub(super) function: Box<dyn FnOnce(&mut T) -> R>,
    pub(super) name: Option<String>,
}

impl<T, R> BoxMutatingFunctionOnce<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
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
    fn apply(self, input: &mut T) -> R {
        (self.function)(input)
    }

    impl_box_once_conversions!(
        BoxMutatingFunctionOnce<T, R>,
        MutatingFunctionOnce,
        FnOnce(&mut T) -> R
    );
}

// Generates: identity() method for BoxMutatingFunctionOnce<T, T>
impl_function_identity_method!(BoxMutatingFunctionOnce<T, T>, mutating);

// Generates: Debug and Display implementations for BoxMutatingFunctionOnce<T, R>
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
