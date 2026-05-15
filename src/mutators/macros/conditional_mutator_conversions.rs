/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Conditional Mutator Conversions Macro
//!
//! Generates conversion methods for Conditional Mutator implementations
//!
//! This macro generates the conversion methods (`into_box`, `into_rc`, `into_fn`) for
//! conditional mutator types. It selects immutable or mutable captures from the
//! generated closure trait (`Fn` or `FnMut`).
//!
//! # Parameters
//!
//! * `$box_type<$t:ident>` - The box-based mutator type (e.g., `BoxMutator<T>`)
//! * `$rc_type:ident` - The rc-based mutator type name (e.g., `RcMutator`)
//! * `$fn_trait:ident` - The function trait (e.g., `Fn` or `FnMut`)
//!
//! # Usage Examples
//!
//! For Mutator (immutable):
//! ```text
//! // impl<T> Mutator<T> for BoxConditionalMutator<T>
//! // where
//! //     T: 'static,
//! // {
//! //     fn apply(&self, value: &mut T) {
//! //         if self.predicate.test(value) {
//! //             self.mutator.apply(value);
//! //         }
//! //     }
//! //
//! //     // Inside the trait impl block
//! //     impl_conditional_mutator_conversions!(
//! //         BoxMutator<T>,
//! //         RcMutator,
//! //         Fn
//! //     );
//! // }
//! ```
//!
//! For StatefulMutator (mutable):
//! ```text
//! // impl<T> StatefulMutator<T> for BoxConditionalStatefulMutator<T>
//! // where
//! //     T: 'static,
//! // {
//! //     fn apply(&mut self, value: &mut T) {
//! //         if self.predicate.test(value) {
//! //             self.mutator.apply(value);
//! //         }
//! //     }
//! //
//! //     // Inside the trait impl block
//! //     impl_conditional_mutator_conversions!(
//! //         BoxStatefulMutator<T>,
//! //         RcStatefulMutator,
//! //         FnMut
//! //     );
//! // }
//! ```
//!
//! # Implementation Details
//!
//! - Uses the `$fn_trait` parameter to choose immutable or mutable captures.
//! - The closures inside `into_box` and `into_rc` capture as `Fn` or `FnMut`
//!   according to the generated operation.
//! - The `into_fn` method uses the provided `$fn_trait` parameter to match the
//!   intended trait type
//!

/// Generates conversion methods for Conditional Mutator implementations
///
/// This macro should be used inside an impl block to generate the conversion
/// methods (`into_box`, `into_rc`, `into_fn`) for conditional mutator types.
/// It selects immutable or mutable captures from the generated closure trait
/// (`Fn` or `FnMut`).
///
/// # Parameters
///
/// * `$box_type<$t:ident>` - The box-based mutator type (e.g., `BoxMutator<T>`)
/// * `$rc_type:ident` - The rc-based mutator type name (e.g., `RcMutator`)
/// * `$fn_trait:ident` - The function trait (e.g., `Fn` or `FnMut`)
///
/// # Usage Location
///
/// This macro should be used inside an impl block for the conditional mutator
/// type, typically within a trait implementation.
///
/// # Usage Examples
///
/// For Mutator (immutable):
/// ```text
/// // impl<T> Mutator<T> for BoxConditionalMutator<T>
/// // where
/// //     T: 'static,
/// // {
/// //     fn apply(&self, value: &mut T) {
/// //         if self.predicate.test(value) {
/// //             self.mutator.apply(value);
/// //         }
/// //     }
/// //
/// //     // Inside the trait impl block
/// //     impl_conditional_mutator_conversions!(
/// //         BoxMutator<T>,
/// //         RcMutator,
/// //         Fn
/// //     );
/// // }
/// ```
///
/// For StatefulMutator (mutable):
/// ```text
/// // impl<T> StatefulMutator<T> for BoxConditionalStatefulMutator<T>
/// // where
/// //     T: 'static,
/// // {
/// //     fn apply(&mut self, value: &mut T) {
/// //         if self.predicate.test(value) {
/// //             self.mutator.apply(value);
/// //         }
/// //     }
/// //
/// //     // Inside the trait impl block
/// //     impl_conditional_mutator_conversions!(
/// //         BoxStatefulMutator<T>,
/// //         RcStatefulMutator,
/// //         FnMut
/// //     );
/// // }
/// ```
///
/// # Implementation Details
///
/// - Uses the `$fn_trait` parameter to choose immutable or mutable captures.
/// - The closures inside `into_box` and `into_rc` capture as `Fn` or `FnMut`
///   according to the generated operation.
/// - The `into_fn` method uses the provided `$fn_trait` parameter to match the
///   intended trait type
///
macro_rules! impl_conditional_mutator_conversions {
    (@let_mutator Fn, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_mutator FnMut, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    // Single generic parameter - Mutator
    (
        $box_type:ident < $t:ident >,
        $rc_type:ident,
        $fn_trait:ident
    ) => {
        fn into_box(self) -> $box_type<$t>
        where
            Self: 'static,
        {
            let pred = self.predicate;
            impl_conditional_mutator_conversions!(@let_mutator $fn_trait, mutator, self.mutator);
            $box_type::new(move |t| {
                if pred.test(t) {
                    mutator.apply(t);
                }
            })
        }

        fn into_rc(self) -> $rc_type<$t>
        where
            Self: 'static,
        {
            let pred = self.predicate.into_rc();
            impl_conditional_mutator_conversions!(@let_mutator $fn_trait, mutator, self.mutator.into_rc());
            $rc_type::new(move |t| {
                if pred.test(t) {
                    mutator.apply(t);
                }
            })
        }

        fn into_fn(self) -> impl $fn_trait(&mut $t) {
            let pred = self.predicate;
            impl_conditional_mutator_conversions!(@let_mutator $fn_trait, mutator, self.mutator);
            move |t: &mut $t| {
                if pred.test(t) {
                    mutator.apply(t);
                }
            }
        }
    };
}

pub(crate) use impl_conditional_mutator_conversions;
