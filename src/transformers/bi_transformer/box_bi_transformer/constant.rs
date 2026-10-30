// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Implements the constant constructor for [`super::BoxBiTransformer`].

use super::BoxBiTransformer;
use crate::transformers::macros::impl_transformer_constant_method;

impl_transformer_constant_method!(BoxBiTransformer<T, U, R>, BiTransformer);
