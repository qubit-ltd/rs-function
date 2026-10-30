// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Common Name Methods Macro
//!
//! Generates common name management methods for function-like structs.

/// Implements common name management methods for function-like structs.
///
/// This macro generates name-management methods for structs that have callback
/// metadata. These methods provide a standardized way to get, set, clear, and
/// chain names for debugging and logging purposes.
///
/// # Parameters
///
/// * `$type_desc:literal` - Description of the type (e.g., "consumer")
///
/// # Generated Methods
///
/// * `name(&self) -> Option<&str>` - Gets the current name if set
/// * `set_name(&mut self, name: &str)` - Sets a new name for the instance
/// * `clear_name(&mut self)` - Clears the current name
/// * `with_name(self, name: &str) -> Self` - Sets a name and returns the
///   instance
macro_rules! impl_common_name_methods {
    ($type_desc:literal) => {
        #[doc = concat!("Returns the diagnostic name assigned to this ", $type_desc, ".")]
        ///
        /// The returned string is borrowed from this wrapper. It is `None` when
        /// no diagnostic name has been assigned, and reading it does not
        /// invoke or otherwise affect the wrapped callback.
        ///
        /// # Returns
        ///
        /// `Some(name)` containing the borrowed diagnostic name, or `None` if
        /// this wrapper is unnamed.
        #[inline(always)]
        pub fn name(&self) -> Option<&str> {
            self.metadata.name()
        }

        #[doc = concat!("Replaces the diagnostic name assigned to this ", $type_desc, ".")]
        ///
        /// The supplied name is stored with this wrapper's metadata and does
        /// not change the wrapped callback's behavior.
        ///
        /// # Parameters
        #[doc = concat!("* `name` - The diagnostic name to store for this ", $type_desc)]
        #[inline(always)]
        pub fn set_name(&mut self, name: &str) {
            self.metadata.set_name(name);
        }

        #[doc = concat!("Consumes this ", $type_desc, " and assigns it a diagnostic name.")]
        ///
        /// This changes only the wrapper's metadata, not the wrapped
        /// callback's behavior. The returned wrapper can be used in a method
        /// chain.
        ///
        /// # Parameters
        #[doc = concat!("* `name` - The diagnostic name to store for this ", $type_desc)]
        ///
        /// # Returns
        #[doc = concat!("This ", $type_desc, " with the supplied name.")]
        /// Replaces the stored name only when it differs from `name`, then
        /// returns the updated instance.
        #[inline(always)]
        pub fn with_name(mut self, name: &str) -> Self {
            self.metadata.set_name(name);
            self
        }

        #[doc = concat!("Removes the diagnostic name assigned to this ", $type_desc, ".")]
        ///
        /// This changes only the wrapper's metadata, not the wrapped
        /// callback's behavior.
        #[inline(always)]
        pub fn clear_name(&mut self) {
            self.metadata.clear_name();
        }
    };
}

pub(crate) use impl_common_name_methods;
