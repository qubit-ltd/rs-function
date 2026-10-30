// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

#[cfg(feature = "stateful")]
mod arc_stateful_supplier_tests;
#[cfg(feature = "stateful")]
mod box_stateful_supplier_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod rc_stateful_supplier_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_supplier_debug_display_tests;
