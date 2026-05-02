/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! # Common Name Methods Macro
//!
//! Generates common name management methods for function-like structs.
//!

/// Implements common name management methods for function-like structs.
///
/// This macro generates `name`, and `set_name` methods for structs that have
/// an optional name field. These methods provide a standardized way to get
/// and set names for debugging and logging purposes.
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
///
macro_rules! impl_common_name_methods {
    ($type_desc:literal) => {
        #[doc = concat!("Gets the name of this ", $type_desc, ".")]
        ///
        /// # Returns
        ///
        /// Returns `Some(&str)` if a name was set, `None` otherwise.
        #[inline]
        pub fn name(&self) -> Option<&str> {
            self.name.as_deref()
        }

        #[doc = concat!("Sets the name of this ", $type_desc, ".")]
        ///
        /// # Parameters
        ///
        #[doc = concat!("* `name` - The name to set for this ", $type_desc)]
        #[inline]
        pub fn set_name(&mut self, name: &str) {
            if self.name.as_deref() != Some(name) {
                self.name = Some(name.to_owned());
            }
        }

        #[doc = concat!("Clears the name of this ", $type_desc, ".")]
        #[inline]
        pub fn clear_name(&mut self) {
            self.name = None;
        }
    };
}

pub(crate) use impl_common_name_methods;
