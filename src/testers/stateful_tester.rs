// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `StatefulTester` public trait.

#[path = "stateful_tester/stateful_tester.rs"]
mod stateful_tester_trait;
pub use stateful_tester_trait::StatefulTester;

mod arc_stateful_tester;
pub use arc_stateful_tester::ArcStatefulTester;
mod box_stateful_tester;
pub use box_stateful_tester::BoxStatefulTester;
#[cfg(feature = "rc")]
mod rc_stateful_tester;
#[cfg(feature = "rc")]
pub use rc_stateful_tester::RcStatefulTester;
