// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the public tester types.

#[path = "tester/tester.rs"]
mod tester_trait;
pub use tester_trait::Tester;

mod arc_tester;
pub use arc_tester::ArcTester;
mod box_tester;
pub use box_tester::BoxTester;
#[cfg(feature = "rc")]
mod rc_tester;
#[cfg(feature = "rc")]
pub use rc_tester::RcTester;
