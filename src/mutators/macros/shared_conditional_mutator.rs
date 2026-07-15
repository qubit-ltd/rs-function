// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Shared Conditional Mutator Macro
//!
//! Generates `and_then` and `or_else` for shared Arc/Rc conditional mutators.
//! Generated methods borrow `&self` and return a wrapper from the same
//! ownership and statefulness family.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Conditional wrapper type.
//! * `$mutator_type` - Result wrapper type, such as `ArcMutator`.
//! * `$mutator_trait` - Semantic trait accepted for the additional callback,
//!   such as `Mutator`.
//! * `$callback_bounds` - Storage capabilities required for the additional
//!   callback.
//!
//! # Capability policy
//!
//! | Wrapper family | `callback_bounds` |
//! |----------------|-------------------|
//! | Arc stateless | `Send + Sync + 'static` |
//! | Arc stateful | `Send + 'static` |
//! | Rc stateless/stateful | `'static` |

/// Generates `and_then` and `or_else` for shared Arc/Rc conditional mutators.
///
/// Invoke this macro at module scope. The selected callback bounds reflect
/// how the result wrapper stores its callback: stateless Arc wrappers call
/// through a shared reference and require `Sync`; stateful Arc wrappers call
/// under an outer mutex and require only `Send`.
///
/// # Capability policy
///
/// | Wrapper family | `callback_bounds` |
/// |----------------|-------------------|
/// | Arc stateless | `Send + Sync + 'static` |
/// | Arc stateful | `Send + 'static` |
/// | Rc stateless/stateful | `'static` |
macro_rules! impl_shared_conditional_mutator {
    (@let_mutator Mutator, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_mutator StatefulMutator, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    // Single generic parameter - Mutator
    (
        $struct_name:ident < $t:ident >,
        $mutator_type:ident,
        $mutator_trait:ident,
        callback_bounds = ($($callback_bounds:tt)+)
    ) => {
        impl<$t> $struct_name<$t> {
            /// Chains another mutator in sequence
            ///
            /// Combines the current conditional mutator with another mutator
            /// into a new mutator that implements the following semantics:
            ///
            /// When the returned mutator is called with an argument:
            /// 1. First, it checks the predicate of this conditional mutator
            /// 2. If the predicate is satisfied, it executes the internal
            ///    mutator of this conditional mutator
            /// 3. Then, **regardless of whether the predicate was satisfied**,
            ///    it unconditionally executes the `next` mutator
            ///
            /// In other words, this creates a mutator that conditionally
            /// executes the first action (based on the predicate), and then
            /// always executes the second action.
            ///
            /// # Parameters
            ///
            /// * `next` - The next mutator to execute (always executed)
            ///
            /// # Returns
            ///
            /// Returns a new combined mutator
            ///
            /// # Examples
            ///
            /// ```ignore
            /// use std::sync::atomic::{AtomicI32, Ordering};
            ///
            /// let result = AtomicI32::new(0);
            ///
            /// let mutator1 = ArcMutator::new(|x: &mut i32| {
            ///     *x += 1;
            /// });
            ///
            /// let mutator2 = ArcMutator::new(|x: &mut i32| {
            ///     *x += 2;
            /// });
            ///
            /// let conditional = mutator1.when(|x| *x > 0);
            /// let chained = conditional.and_then(mutator2);
            ///
            /// let mut val = 5;
            /// chained.apply(&mut val);  // val = 5 + 1 + 2 = 8
            /// let mut val2 = -1;
            /// chained.apply(&mut val2); // val2 = -1 + 2 = 1 (not -1 + 1 + 2!)
            /// ```
            pub fn and_then<M>(&self, next: M) -> $mutator_type<$t>
            where
                $t: 'static,
                M: $mutator_trait<$t> + $($callback_bounds)+,
            {
                let first_predicate = self.predicate.clone();
                impl_shared_conditional_mutator!(@let_mutator $mutator_trait, first_mutator, self.mutator.clone());
                impl_shared_conditional_mutator!(@let_mutator $mutator_trait, next, next);
                $mutator_type::new(move |t: &mut $t| {
                    if first_predicate.test(t) {
                        first_mutator.apply(t);
                    }
                    next.apply(t);
                })
            }

            /// Adds an else branch
            ///
            /// Executes the original mutator when the condition is satisfied, otherwise
            /// executes else_mutator.
            ///
            /// # Parameters
            ///
            /// * `else_mutator` - The mutator for the else branch
            ///
            /// # Returns
            ///
            /// Returns a new mutator with if-then-else logic
            pub fn or_else<M>(&self, else_mutator: M) -> $mutator_type<$t>
            where
                $t: 'static,
                M: $mutator_trait<$t> + $($callback_bounds)+,
            {
                let predicate = self.predicate.clone();
                impl_shared_conditional_mutator!(@let_mutator $mutator_trait, then_mutator, self.mutator.clone());
                impl_shared_conditional_mutator!(@let_mutator $mutator_trait, else_mutator, else_mutator);
                $mutator_type::new(move |t: &mut $t| {
                    if predicate.test(t) {
                        then_mutator.apply(t);
                    } else {
                        else_mutator.apply(t);
                    }
                })
            }
        }
    };
}

pub(crate) use impl_shared_conditional_mutator;
