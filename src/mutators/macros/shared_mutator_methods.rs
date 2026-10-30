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
    (@doc_when RcMutator, Mutator) => {
        "```rust\nuse qubit_function::Mutator;\nuse qubit_function::RcMutator;\nlet source = RcMutator::new(|value: &mut i32| *value += 1);\nlet conditional = source.when(|value: &i32| *value > 0);\nlet mut positive = 2;\nconditional.apply(&mut positive);\nassert_eq!(positive, 3);\nlet mut negative = -2;\nconditional.apply(&mut negative);\nassert_eq!(negative, -2);\n```"
    };
    (@doc_when ArcMutator, Mutator) => {
        "```rust\nuse qubit_function::ArcMutator;\nuse qubit_function::Mutator;\nlet source = ArcMutator::new(|value: &mut i32| *value += 1);\nlet conditional = source.when(|value: &i32| *value > 0);\nlet mut positive = 2;\nconditional.apply(&mut positive);\nassert_eq!(positive, 3);\nlet mut negative = -2;\nconditional.apply(&mut negative);\nassert_eq!(negative, -2);\n```"
    };
    (@doc_when RcStatefulMutator, StatefulMutator) => {
        "```rust\nuse qubit_function::RcStatefulMutator;\nuse qubit_function::StatefulMutator;\nlet mut calls = 0;\nlet source = RcStatefulMutator::new(move |value: &mut i32| {\n    calls += 1;\n    *value += calls;\n});\nlet mut conditional = source.when(|value: &i32| *value > 0);\nlet mut first = 2;\nconditional.apply(&mut first);\nassert_eq!(first, 3);\nlet mut skipped = -2;\nconditional.apply(&mut skipped);\nassert_eq!(skipped, -2);\nlet mut second = 2;\nconditional.apply(&mut second);\nassert_eq!(second, 4);\n```"
    };
    (@doc_when ArcStatefulMutator, StatefulMutator) => {
        "```rust\nuse qubit_function::ArcStatefulMutator;\nuse qubit_function::StatefulMutator;\nlet mut calls = 0;\nlet source = ArcStatefulMutator::new(move |value: &mut i32| {\n    calls += 1;\n    *value += calls;\n});\nlet mut conditional = source.when(|value: &i32| *value > 0);\nlet mut first = 2;\nconditional.apply(&mut first);\nassert_eq!(first, 3);\nlet mut skipped = -2;\nconditional.apply(&mut skipped);\nassert_eq!(skipped, -2);\nlet mut second = 2;\nconditional.apply(&mut second);\nassert_eq!(second, 4);\n```"
    };
    (@doc_then RcMutator, Mutator) => {
        "```rust\nuse qubit_function::Mutator;\nuse qubit_function::RcMutator;\nlet source = RcMutator::new(|value: &mut i32| *value += 1);\nlet combined = source.and_then(|value: &mut i32| *value *= 2);\nlet mut input = 2;\ncombined.apply(&mut input);\nassert_eq!(input, 6);\n```"
    };
    (@doc_then ArcMutator, Mutator) => {
        "```rust\nuse qubit_function::ArcMutator;\nuse qubit_function::Mutator;\nlet source = ArcMutator::new(|value: &mut i32| *value += 1);\nlet combined = source.and_then(|value: &mut i32| *value *= 2);\nlet mut input = 2;\ncombined.apply(&mut input);\nassert_eq!(input, 6);\n```"
    };
    (@doc_then RcStatefulMutator, StatefulMutator) => {
        "```rust\nuse qubit_function::RcStatefulMutator;\nuse qubit_function::StatefulMutator;\nlet mut calls = 0;\nlet source = RcStatefulMutator::new(move |value: &mut i32| {\n    calls += 1;\n    *value += calls;\n});\nlet mut combined = source.and_then(|value: &mut i32| *value *= 2);\nlet mut first = 2;\ncombined.apply(&mut first);\nassert_eq!(first, 6);\nlet mut second = 2;\ncombined.apply(&mut second);\nassert_eq!(second, 8);\n```"
    };
    (@doc_then ArcStatefulMutator, StatefulMutator) => {
        "```rust\nuse qubit_function::ArcStatefulMutator;\nuse qubit_function::StatefulMutator;\nlet mut calls = 0;\nlet source = ArcStatefulMutator::new(move |value: &mut i32| {\n    calls += 1;\n    *value += calls;\n});\nlet mut combined = source.and_then(|value: &mut i32| *value *= 2);\nlet mut first = 2;\ncombined.apply(&mut first);\nassert_eq!(first, 6);\nlet mut second = 2;\ncombined.apply(&mut second);\nassert_eq!(second, 8);\n```"
    };
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
        #[doc = impl_shared_mutator_methods!(@doc_when $struct_name, $mutator_trait)]
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $return_type<$t>
        where
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
        #[doc = impl_shared_mutator_methods!(@doc_then $struct_name, $mutator_trait)]
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
