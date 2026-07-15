// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Shared Mutator Methods Macro
//!
//! Generates `when` and `and_then` for shared Arc/Rc mutators.
//! The generated methods borrow `&self`, clone the shared wrapper, and keep
//! predicate-storage capabilities separate from chained-callback
//! capabilities.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Mutator wrapper type.
//! * `$return_type` - Conditional wrapper returned by `when`, such as
//!   `ArcConditionalMutator`.
//! * `$predicate_type` - Predicate wrapper used by the conditional result.
//! * `$mutator_trait` - Semantic trait implemented by the chained callback,
//!   such as `Mutator`.
//! * `$predicate_bounds` - Bounds required to store the predicate wrapper.
//! * `$chained_bounds` - Bounds required to store the callback produced by
//!   `and_then`.
//!
//! # Capability policy
//!
//! | Wrapper family | `predicate_bounds` | `chained_bounds` |
//! |----------------|--------------------|------------------|
//! | Arc stateless | `Send + Sync + 'static` | `Send + Sync + 'static` |
//! | Arc stateful | `Send + Sync + 'static` | `Send + 'static` |
//! | Rc stateless/stateful | `'static` | `'static` |

/// Generates `when` and `and_then` for shared Arc/Rc mutators.
///
/// Invoke this macro inside the target wrapper's `impl` block. Predicate
/// bounds describe the immutable predicate object stored by the conditional
/// wrapper. Chained bounds describe the callback stored by the returned
/// mutator; stateful Arc callbacks are serialized by their outer
/// mutex and therefore require `Send`, but not `Sync`.
///
/// # Capability policy
///
/// | Wrapper family | `predicate_bounds` | `chained_bounds` |
/// |----------------|--------------------|------------------|
/// | Arc stateless | `Send + Sync + 'static` | `Send + Sync + 'static` |
/// | Arc stateful | `Send + Sync + 'static` | `Send + 'static` |
/// | Rc stateless/stateful | `'static` | `'static` |
macro_rules! impl_shared_mutator_methods {
    (@and_then Mutator, $struct_name:ident, $first:expr, $after:expr, $t:ident) => {{
        let first = $first;
        let after = $after;
        $struct_name::new(move |t: &mut $t| {
            first.apply(t);
            after.apply(t);
        })
    }};

    (@and_then StatefulMutator, $struct_name:ident, $first:expr, $after:expr, $t:ident) => {{
        let mut first = $first;
        let mut after = $after;
        $struct_name::new(move |t: &mut $t| {
            first.apply(t);
            after.apply(t);
        })
    }};

    // Single generic parameter
    (
        $struct_name:ident < $t:ident >,
        $return_type:ident,
        $predicate_type:ident,
        $mutator_trait:ident,
        predicate_bounds = ($($predicate_bounds:tt)+),
        chained_bounds = ($($chained_bounds:tt)+)
    ) => {
        /// Creates a conditional mutator that executes based on predicate
        /// result.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to determine whether to execute
        ///   the mutation operation
        ///
        /// # Returns
        ///
        /// Returns a conditional mutator that only executes when the
        /// predicate returns `true`.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use std::sync::Arc;
        /// use std::sync::atomic::{AtomicI32, Ordering};
        /// use qubit_function::mutators::*;
        ///
        /// let counter = Arc::new(AtomicI32::new(0));
        /// let mutator = ArcMutator::new({
        ///     let counter = Arc::clone(&counter);
        ///     move |value: &mut i32| {
        ///         *value += counter.fetch_add(1, Ordering::SeqCst);
        ///     }
        /// });
        ///
        /// let conditional = mutator.when(|value: &i32| *value > 0);
        /// let mut val = 1;
        /// conditional.apply(&mut val);  // val = 2 (1 + 1)
        /// let mut val2 = -1;
        /// conditional.apply(&mut val2); // not executed, val2 remains -1
        /// ```
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $return_type<$t>
        where
            $t: 'static,
            P: Predicate<$t> + $($predicate_bounds)+,
        {
            $return_type {
                mutator: self.clone(),
                predicate: $crate::$predicate_type::new(predicate),
            }
        }

        /// Chains execution with another mutator, executing the current
        /// mutator first, then the subsequent mutator.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent mutator to execute after the current
        ///   mutator completes
        ///
        /// # Returns
        ///
        /// Returns a new mutator that executes the current mutator and
        /// the subsequent mutator in sequence.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use std::sync::Arc;
        /// use std::sync::atomic::{AtomicI32, Ordering};
        /// use qubit_function::mutators::*;
        ///
        /// let counter1 = Arc::new(AtomicI32::new(0));
        /// let counter2 = Arc::new(AtomicI32::new(0));
        ///
        /// let mutator1 = ArcMutator::new({
        ///     let counter = Arc::clone(&counter1);
        ///     move |value: &mut i32| {
        ///         *value += counter.fetch_add(1, Ordering::SeqCst);
        ///     }
        /// });
        ///
        /// let mutator2 = ArcMutator::new({
        ///     let counter = Arc::clone(&counter2);
        ///     move |value: &mut i32| {
        ///         *value += counter.fetch_add(1, Ordering::SeqCst);
        ///     }
        /// });
        ///
        /// let chained = mutator1.and_then(mutator2);
        /// let mut val = 0;
        /// chained.apply(&mut val);
        /// // val = 2 (0 + 1 + 1)
        /// ```
        #[inline]
        pub fn and_then<M>(&self, after: M) -> $struct_name<$t>
        where
            $t: 'static,
            M: $mutator_trait<$t> + $($chained_bounds)+,
        {
            impl_shared_mutator_methods!(@and_then $mutator_trait, $struct_name, self.clone(), after, $t)
        }
    };
}

pub(crate) use impl_shared_mutator_methods;
