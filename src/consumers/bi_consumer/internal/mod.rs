// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private function-signature aliases shared by bi-consumer wrappers.

mod bi_consumer_fn;
pub(super) use bi_consumer_fn::BiConsumerFn;
mod thread_safe_bi_consumer_fn;
pub(super) use thread_safe_bi_consumer_fn::ThreadSafeBiConsumerFn;
