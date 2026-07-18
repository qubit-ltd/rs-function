// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxStatefulTester` public type.

use std::{
    fmt,
    ops::Not,
};

use {
    super::StatefulTester,
    crate::{
        internal::CallbackMetadata,
        macros::{
            impl_common_name_methods,
            impl_common_new_methods,
        },
    },
};

/// A single-ownership stateful tester backed by `Box<dyn FnMut() -> bool>`.
///
/// `BoxStatefulTester` is the owned wrapper for zero-argument tests that need
/// native `FnMut() -> bool` semantics. Calling [`StatefulTester::test`] may
/// mutate state captured by the wrapped closure.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxStatefulTester {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn FnMut() -> bool>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: CallbackMetadata,
}

impl BoxStatefulTester {
    impl_common_new_methods!(
        semantic_mut(StatefulTester + 'static),
        |source| move || source.test(),
        |function| Box::new(function),
        "stateful tester"
    );

    impl_common_name_methods!("stateful tester");

    /// Combines this tester with another stateful tester using logical AND.
    ///
    /// The second tester is evaluated only when the first tester returns
    /// `true`.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester evaluated after this tester succeeds.
    ///
    /// # Returns
    ///
    /// A tester that evaluates the logical AND of both testers.
    #[inline]
    pub fn and<T>(mut self, mut next: T) -> BoxStatefulTester
    where
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || self.test() && next.test())
    }

    /// Combines this tester with another stateful tester using logical OR.
    ///
    /// The second tester is evaluated only when the first tester returns
    /// `false`.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester evaluated after this tester fails.
    ///
    /// # Returns
    ///
    /// A tester that evaluates the logical OR of both testers.
    #[inline]
    pub fn or<T>(mut self, mut next: T) -> BoxStatefulTester
    where
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || self.test() || next.test())
    }

    /// Combines this tester with another stateful tester using logical NAND.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester combined with this tester.
    ///
    /// # Returns
    ///
    /// A tester that negates the logical AND of both testers.
    #[inline]
    pub fn nand<T>(mut self, mut next: T) -> BoxStatefulTester
    where
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || !(self.test() && next.test()))
    }

    /// Combines this tester with another stateful tester using logical XOR.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester combined with this tester.
    ///
    /// # Returns
    ///
    /// A tester that evaluates whether exactly one tester succeeds.
    #[inline]
    pub fn xor<T>(mut self, mut next: T) -> BoxStatefulTester
    where
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || self.test() ^ next.test())
    }

    /// Combines this tester with another stateful tester using logical NOR.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester combined with this tester.
    ///
    /// # Returns
    ///
    /// A tester that negates the logical OR of both testers.
    #[inline]
    pub fn nor<T>(mut self, mut next: T) -> BoxStatefulTester
    where
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || !(self.test() || next.test()))
    }
}

impl Not for BoxStatefulTester {
    type Output = BoxStatefulTester;

    #[inline]
    fn not(self) -> Self::Output {
        let metadata = self.metadata;
        let mut function = self.function;
        BoxStatefulTester::new_with_metadata(move || !function(), metadata)
    }
}

impl StatefulTester for BoxStatefulTester {
    #[inline(always)]
    fn test(&mut self) -> bool {
        (self.function)()
    }
}

impl fmt::Debug for BoxStatefulTester {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BoxStatefulTester")
            .field("name", &self.metadata.name())
            .finish()
    }
}

impl fmt::Display for BoxStatefulTester {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.metadata.name() {
            Some(name) => write!(formatter, "BoxStatefulTester({name})"),
            None => formatter.write_str("BoxStatefulTester"),
        }
    }
}
