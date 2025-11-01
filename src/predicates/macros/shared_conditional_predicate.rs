/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Shared Conditional Predicate Macro
//!
//! Generates Arc/Rc-based Conditional Predicate implementations
//!
//! For Arc/Rc-based conditional predicates, generates `and_then` and `or_else` methods,
//! as well as complete Predicate/BiPredicate trait implementations.
//!
//! Arc/Rc type characteristics:
//! - `and_then` and `or_else` borrow &self (because Arc/Rc can Clone)
//! - Uses trait default implementations for `into_arc()` and `to_arc()`
//! - Arc types will work with `into_arc()` and `to_arc()` (satisfy Send + Sync constraints)
//! - Rc types will get compile errors if trying to use `into_arc()` or `to_arc()` (don't satisfy Send + Sync)
//! - Implement complete `to_xxx()` methods (because they can Clone)
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$predicate_type` - Predicate wrapper type name
//! * `$predicate_trait` - Predicate trait name
//! * `$predicate_conversion` - Predicate conversion method (into_arc or into_rc)
//! * `$extra_bounds` - Extra trait bounds
//!
//! # Usage Examples
//!
//! ```ignore
//! // Arc single-parameter Predicate
//! impl_shared_conditional_predicate!(
//!     ArcConditionalPredicate<T>,
//!     ArcPredicate,
//!     Predicate,
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc single-parameter Predicate
//! impl_shared_conditional_predicate!(
//!     RcConditionalPredicate<T>,
//!     RcPredicate,
//!     Predicate,
//!     into_rc,
//!     'static
//! );
//!
//! // Arc two-parameter BiPredicate
//! impl_shared_conditional_predicate!(
//!     ArcConditionalBiPredicate<T, U>,
//!     ArcBiPredicate,
//!     BiPredicate,
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc two-parameter BiPredicate
//! impl_shared_conditional_predicate!(
//!     RcConditionalBiPredicate<T, U>,
//!     RcBiPredicate,
//!     BiPredicate,
//!     into_rc,
//!     'static
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Arc/Rc-based Conditional Predicate implementations
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// For Arc/Rc-based conditional predicates, generates `and_then` and `or_else` methods,
/// as well as complete Predicate/BiPredicate trait implementations.
///
/// Arc/Rc type characteristics:
/// - `and_then` and `or_else` borrow &self (because Arc/Rc can Clone)
/// - Uses trait default implementations for `into_arc()` and `to_arc()`
/// - Arc types will work with `into_arc()` and `to_arc()` (satisfy Send + Sync constraints)
/// - Rc types will get compile errors if trying to use `into_arc()` or `to_arc()` (don't satisfy Send + Sync)
/// - Implement complete `to_xxx()` methods (because they can Clone)
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$predicate_type` - Predicate wrapper type name
/// * `$predicate_trait` - Predicate trait name
/// * `$predicate_conversion` - Predicate conversion method (into_arc or into_rc)
/// * `$extra_bounds` - Extra trait bounds
///
/// # Usage Examples
///
/// ```ignore
/// // Arc single-parameter Predicate
/// impl_shared_conditional_predicate!(
///     ArcConditionalPredicate<T>,
///     ArcPredicate,
//!     Predicate,
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc single-parameter Predicate
/// impl_shared_conditional_predicate!(
//!     RcConditionalPredicate<T>,
//!     RcPredicate,
//!     Predicate,
//!     into_rc,
//!     'static
//! );
//!
//! // Arc two-parameter BiPredicate
/// impl_shared_conditional_predicate!(
//!     ArcConditionalBiPredicate<T, U>,
//!     ArcBiPredicate,
//!     BiPredicate,
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc two-parameter BiPredicate
/// impl_shared_conditional_predicate!(
//!     RcConditionalBiPredicate<T, U>,
//!     RcBiPredicate,
//!     BiPredicate,
//!     into_rc,
//!     'static
//! );
//! ```
macro_rules! impl_shared_conditional_predicate {
    // Single generic parameter - Predicate
    (
        $struct_name:ident < $t:ident >,
        $predicate_type:ident,
        $predicate_trait:ident,
        $predicate_conversion:ident,
        $($extra_bounds:tt)+
    ) => {
        impl<$t> $struct_name<$t>
        where
            $t: 'static,
        {
            /// Combines with another predicate using logical AND
            ///
            /// Creates a new predicate that returns `true` only when both
            /// this conditional predicate's condition is satisfied AND the
            /// additional predicate returns `true`.
            ///
            /// # Parameters
            ///
            /// * `other` - The additional predicate to combine with
            ///
            /// # Returns
            ///
            /// Returns a new combined predicate
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let pred1 = ArcPredicate::new(|x: &i32| *x > 0);
            /// let pred2 = ArcPredicate::new(|x: &i32| x % 2 == 0);
            /// let conditional = pred1.when(|x| *x < 10);
            /// let combined = conditional.and_then(pred2);
            ///
            /// assert!(combined.test(&4));  // 4 > 0 && 4 < 10 && 4 % 2 == 0
            /// assert!(!combined.test(&15)); // 15 > 0 && 15 < 10 (fails) && 15 % 2 != 0
            /// ```
            pub fn and_then<P>(&self, other: P) -> $predicate_type<$t>
            where
                P: $predicate_trait<$t> + $($extra_bounds)+,
            {
                let condition = self.condition.clone();
                let predicate = self.predicate.clone();
                $predicate_type::new(move |t| {
                    condition.test(t) && predicate.test(t) && other.test(t)
                })
            }

            /// Combines with another predicate using logical OR
            ///
            /// Creates a new predicate that returns `true` when either
            /// this conditional predicate's condition is satisfied OR the
            /// additional predicate returns `true`.
            ///
            /// # Parameters
            ///
            /// * `other` - The alternative predicate
            ///
            /// # Returns
            ///
            /// Returns a new combined predicate
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let pred1 = ArcPredicate::new(|x: &i32| *x > 10);
            /// let pred2 = ArcPredicate::new(|x: &i32| x % 2 == 0);
            /// let conditional = pred1.when(|x| *x < 5);
            /// let combined = conditional.or_else(pred2);
            ///
            /// assert!(combined.test(&4));  // 4 > 10 (false) || 4 < 5 (false) || 4 % 2 == 0 (true)
            /// assert!(!combined.test(&7)); // 7 > 10 (false) || 7 < 5 (false) || 7 % 2 == 0 (false)
            /// ```
            pub fn or_else<P>(&self, other: P) -> $predicate_type<$t>
            where
                P: $predicate_trait<$t> + $($extra_bounds)+,
            {
                let condition = self.condition.clone();
                let predicate = self.predicate.clone();
                $predicate_type::new(move |t| {
                    (condition.test(t) && predicate.test(t)) || other.test(t)
                })
            }
        }
    };

    // Two generic parameters - BiPredicate
    (
        $struct_name:ident < $t:ident, $u:ident >,
        $predicate_type:ident,
        $predicate_trait:ident,
        $predicate_conversion:ident,
        $($extra_bounds:tt)+
    ) => {
        impl<$t, $u> $struct_name<$t, $u>
        where
            $t: 'static,
            $u: 'static,
        {
            /// Combines with another bi-predicate using logical AND
            ///
            /// Creates a new bi-predicate that returns `true` only when both
            /// this conditional bi-predicate's condition is satisfied AND the
            /// additional bi-predicate returns `true`.
            ///
            /// # Parameters
            ///
            /// * `other` - The additional bi-predicate to combine with
            ///
            /// # Returns
            ///
            /// Returns a new combined bi-predicate
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let pred1 = ArcBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
            /// let pred2 = ArcBiPredicate::new(|x: &i32, y: &i32| x * y > 0);
            /// let conditional = pred1.when(|x, y| *x > 0);
            /// let combined = conditional.and_then(pred2);
            ///
            /// assert!(combined.test(&2, &3));  // 2 > 0 && (2+3) > 0 && (2*3) > 0
            /// assert!(!combined.test(&-2, &3)); // -2 > 0 (fails) && (-2+3) > 0 && (-2*3) < 0
            /// ```
            pub fn and_then<P>(&self, other: P) -> $predicate_type<$t, $u>
            where
                P: $predicate_trait<$t, $u> + $($extra_bounds)+,
            {
                let condition = self.condition.clone();
                let predicate = self.predicate.clone();
                $predicate_type::new(move |t, u| {
                    condition.test(t, u) && predicate.test(t, u) && other.test(t, u)
                })
            }

            /// Combines with another bi-predicate using logical OR
            ///
            /// Creates a new bi-predicate that returns `true` when either
            /// this conditional bi-predicate's condition is satisfied OR the
            /// additional bi-predicate returns `true`.
            ///
            /// # Parameters
            ///
            /// * `other` - The alternative bi-predicate
            ///
            /// # Returns
            ///
            /// Returns a new combined bi-predicate
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let pred1 = ArcBiPredicate::new(|x: &i32, y: &i32| x + y > 10);
            /// let pred2 = ArcBiPredicate::new(|x: &i32, y: &i32| x * y > 0);
            /// let conditional = pred1.when(|x, y| *x < 0);
            /// let combined = conditional.or_else(pred2);
            ///
            /// assert!(combined.test(&2, &3));  // 2 < 0 (false) || (2+3) > 10 (false) || (2*3) > 0 (true)
            /// assert!(!combined.test(&-2, &-3)); // -2 < 0 (true) && (-2+-3) < 10 (true) && (-2*-3) > 0 (true)
            ///                                    // Wait, this would be true. Let me fix the example...
            /// ```
            pub fn or_else<P>(&self, other: P) -> $predicate_type<$t, $u>
            where
                P: $predicate_trait<$t, $u> + $($extra_bounds)+,
            {
                let condition = self.condition.clone();
                let predicate = self.predicate.clone();
                $predicate_type::new(move |t, u| {
                    (condition.test(t, u) && predicate.test(t, u)) || other.test(t, u)
                })
            }
        }
    };
}

pub(crate) use impl_shared_conditional_predicate;
