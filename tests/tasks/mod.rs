// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

mod callable;
mod callable_once;
#[cfg(feature = "once")]
mod callable_once_tests;
#[cfg(all(feature = "rc", feature = "once", feature = "stateful"))]
mod callable_tests;
mod callable_with;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod callable_with_tests;
mod runnable;
mod runnable_once;
#[cfg(feature = "once")]
mod runnable_once_tests;
#[cfg(all(feature = "rc", feature = "once", feature = "stateful"))]
mod runnable_tests;
mod runnable_with;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod runnable_with_tests;
#[cfg(feature = "once")]
mod task_auto_trait_contract_tests;
