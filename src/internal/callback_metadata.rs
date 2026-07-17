// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared diagnostic metadata for callback wrappers.

use std::sync::Arc;

/// Diagnostic metadata shared by all concrete callback wrappers.
///
/// Cloning a wrapper clones the small [`Arc`] handle instead of copying the
/// callback name's bytes. Mutating a clone's metadata replaces only that
/// clone's handle.
#[derive(Clone, Debug, Default)]
pub(crate) struct CallbackMetadata {
    /// The optional diagnostic name.
    name: Option<Arc<str>>,
}

impl CallbackMetadata {
    /// Creates metadata without a diagnostic name.
    #[inline(always)]
    pub(crate) const fn unnamed() -> Self {
        Self { name: None }
    }

    /// Creates metadata with a diagnostic name.
    #[inline]
    pub(crate) fn named(name: &str) -> Self {
        Self {
            name: Some(Arc::from(name)),
        }
    }

    /// Creates metadata from the public optional-name representation.
    #[inline]
    pub(crate) fn from_optional_name(name: Option<String>) -> Self {
        Self {
            name: name.map(Arc::from),
        }
    }

    /// Returns the diagnostic name, if present.
    #[inline(always)]
    pub(crate) fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Replaces the diagnostic name when it differs from the current value.
    #[inline(always)]
    pub(crate) fn set_name(&mut self, name: &str) {
        if self.name() != Some(name) {
            self.name = Some(Arc::from(name));
        }
    }

    /// Removes the diagnostic name.
    #[inline(always)]
    pub(crate) fn clear_name(&mut self) {
        self.name = None;
    }
}
