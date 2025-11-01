/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Shared Predicate Methods Macro
//!
//! Generates when and and_then method implementations for Arc/Rc-based Predicate
//!
//! Generates conditional execution when method and logical combination and_then method
//! for Arc/Rc-based predicates that borrow &self (because Arc/Rc can be cloned).
//!
//! This macro supports both single-parameter and two-parameter predicates through
//! pattern matching on the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `ArcPredicate<T>`
//!   - Two parameters: `ArcBiPredicate<T, U>`
//! * `$return_type` - The return type for when (e.g., ArcConditionalPredicate)
//! * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
//! * `$predicate_trait` - Predicate trait name (e.g., Predicate, BiPredicate)
//! * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
//!
//! # All Macro Invocations
//!
//! | Predicate Type | Struct Signature | `$return_type` | `$predicate_conversion` | `$predicate_trait` | `$extra_bounds` |
//! |----------------|------------------|----------------|------------------------|-------------------|----------------|
//! | **ArcPredicate** | `ArcPredicate<T>` | ArcConditionalPredicate | into_arc | Predicate | Send + Sync + 'static |
//! | **RcPredicate** | `RcPredicate<T>` | RcConditionalPredicate | into_rc | Predicate | 'static |
//! | **ArcBiPredicate** | `ArcBiPredicate<T, U>` | ArcConditionalBiPredicate | into_arc | BiPredicate | Send + Sync + 'static |
//! | **RcBiPredicate** | `RcBiPredicate<T, U>` | RcConditionalBiPredicate | into_rc | BiPredicate | 'static |
//!
//! # Examples
//!
//! ```ignore
//! // Single-parameter with Arc
//! impl_shared_predicate_methods!(
//!     ArcPredicate<T>,
//!     ArcConditionalPredicate,
//!     into_arc,
//!     Predicate,
//!     Send + Sync + 'static
//! );
//!
//! // Two-parameter with Rc
//! impl_shared_predicate_methods!(
//!     RcBiPredicate<T, U>,
//!     RcConditionalBiPredicate,
//!     into_rc,
//!     BiPredicate,
//!     'static
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates when and and_then method implementations for Arc/Rc-based Predicate
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates conditional execution when method and logical
/// combination and_then method for Arc/Rc-based predicates that borrow &self
/// (because Arc/Rc can be cloned).
///
/// This macro supports both single-parameter and two-parameter predicates through
/// pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `ArcPredicate<T>`
///   - Two parameters: `ArcBiPredicate<T, U>`
/// * `$return_type` - The return type for when (e.g., ArcConditionalPredicate)
/// * `$predicate_conversion` - Method to convert predicate (into_arc or into_rc)
/// * `$predicate_trait` - Predicate trait name (e.g., Predicate, BiPredicate)
/// * `$extra_bounds` - Extra trait bounds ('static for Rc, Send + Sync + 'static for Arc)
///
/// # All Macro Invocations
///
/// | Predicate Type | Struct Signature | `$return_type` | `$predicate_conversion` | `$predicate_trait` | `$extra_bounds` |
/// |----------------|------------------|----------------|------------------------|-------------------|----------------|
/// | **ArcPredicate** | `ArcPredicate<T>` | ArcConditionalPredicate | into_arc | Predicate | Send + Sync + 'static |
/// | **RcPredicate** | `RcPredicate<T>` | RcConditionalPredicate | into_rc | Predicate | 'static |
/// | **ArcBiPredicate** | `ArcBiPredicate<T, U>` | ArcConditionalBiPredicate | into_arc | BiPredicate | Send + Sync + 'static |
/// | **RcBiPredicate** | `RcBiPredicate<T, U>` | RcConditionalBiPredicate | into_rc | BiPredicate | 'static |
///
/// # Examples
///
/// ```ignore
/// // Single-parameter with Arc
/// impl_shared_predicate_methods!(
///     ArcPredicate<T>,
///     ArcConditionalPredicate,
//!     into_arc,
//!     Predicate,
//!     Send + Sync + 'static
//! );
//!
//! // Two-parameter with Rc
/// impl_shared_predicate_methods!(
//!     RcBiPredicate<T, U>,
//!     RcConditionalBiPredicate,
//!     into_rc,
//!     BiPredicate,
//!     'static
//! );
//! ```
macro_rules! impl_shared_predicate_methods {
    // Single generic parameter
    ($struct_name:ident < $t:ident >, $return_type:ident, $predicate_conversion:ident, $predicate_trait:ident, $($extra_bounds:tt)+) => {
        /// Creates a conditional predicate that executes based on condition
        /// predicate result.
        ///
        /// # Parameters
        ///
        /// * `condition` - The condition predicate to determine whether to
        ///   execute the main predicate
        ///
        /// # Returns
        ///
        /// Returns a conditional predicate that combines the condition and
        /// main predicate with logical AND.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// let pred = ArcPredicate::new(|x: &i32| *x > 10);
        /// let condition = ArcPredicate::new(|x: &i32| *x < 100);
        /// let conditional = pred.when(condition);
        ///
        /// assert!(conditional.test(&50));  // 50 > 10 && 50 < 100
        /// assert!(!conditional.test(&5));  // 5 > 10 (false) && 5 < 100
        /// ```
        pub fn when<P>(&self, condition: P) -> $return_type<$t>
        where
            P: $predicate_trait<$t> + $($extra_bounds)+,
        {
            $return_type {
                predicate: self.clone(),
                condition: condition.$predicate_conversion(),
            }
        }

        /// Combines with another predicate using logical AND.
        ///
        /// # Parameters
        ///
        /// * `other` - The other predicate to combine with
        ///
        /// # Returns
        ///
        /// Returns a new predicate that returns `true` only when both
        /// predicates return `true`.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// let pred1 = ArcPredicate::new(|x: &i32| *x > 0);
        /// let pred2 = ArcPredicate::new(|x: &i32| x % 2 == 0);
        /// let combined = pred1.and_then(pred2);
        ///
        /// assert!(combined.test(&4));  // 4 > 0 && 4 % 2 == 0
        /// assert!(!combined.test(&3)); // 3 > 0 && 3 % 2 != 0 (false)
        /// ```
        pub fn and_then<P>(&self, other: P) -> $struct_name<$t>
        where
            $t: 'static,
            P: $predicate_trait<$t> + $($extra_bounds)+,
        {
            let first = self.clone();
            $struct_name::new(move |t: &$t| {
                first.test(t) && other.test(t)
            })
        }
    };
    // Two generic parameters
    ($struct_name:ident < $t:ident, $u:ident >, $return_type:ident, $predicate_conversion:ident, $predicate_trait:ident, $($extra_bounds:tt)+) => {
        /// Creates a conditional two-parameter predicate that executes based
        /// on condition bi-predicate result.
        ///
        /// # Parameters
        ///
        /// * `condition` - The condition bi-predicate to determine whether to
        ///   execute the main bi-predicate
        ///
        /// # Returns
        ///
        /// Returns a conditional two-parameter predicate that combines the
        /// condition and main bi-predicate with logical AND.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// let pred = ArcBiPredicate::new(|x: &i32, y: &i32| x + y > 10);
        /// let condition = ArcBiPredicate::new(|x: &i32, y: &i32| *x > 0);
        /// let conditional = pred.when(condition);
        ///
        /// assert!(conditional.test(&5, &8));  // 5 > 0 && (5+8) > 10
        /// assert!(!conditional.test(&-1, &8)); // -1 > 0 (false) && (-1+8) > 10
        /// ```
        pub fn when<P>(&self, condition: P) -> $return_type<$t, $u>
        where
            P: $predicate_trait<$t, $u> + $($extra_bounds)+,
        {
            $return_type {
                predicate: self.clone(),
                condition: condition.$predicate_conversion(),
            }
        }

        /// Combines with another bi-predicate using logical AND.
        ///
        /// # Parameters
        ///
        /// * `other` - The other bi-predicate to combine with
        ///
        /// # Returns
        ///
        /// Returns a new bi-predicate that returns `true` only when both
        /// bi-predicates return `true`.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// let pred1 = ArcBiPredicate::new(|x: &i32, y: &i32| *x > 0);
        /// let pred2 = ArcBiPredicate::new(|x: &i32, y: &i32| *y > 0);
        /// let combined = pred1.and_then(pred2);
        ///
        /// assert!(combined.test(&1, &2));  // 1 > 0 && 2 > 0
        /// assert!(!combined.test(&1, &-1)); // 1 > 0 && -1 > 0 (false)
        /// ```
        pub fn and_then<P>(&self, other: P) -> $struct_name<$t, $u>
        where
            $t: 'static,
            $u: 'static,
            P: $predicate_trait<$t, $u> + $($extra_bounds)+,
        {
            let first = self.clone();
            $struct_name::new(move |t: &$t, u: &$u| {
                first.test(t, u) && other.test(t, u)
            })
        }
    };
}

pub(crate) use impl_shared_predicate_methods;
