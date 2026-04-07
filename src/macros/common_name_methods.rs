/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Common Name Methods Macro
//!
//! Generates common name management methods for function-like structs.
//!
//! # Author
//!
//! Haixing Hu

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
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_common_name_methods {
    ($type_desc:literal) => {
        #[doc = concat!("Gets the name of this ", $type_desc, ".")]
        ///
        /// # Returns
        ///
        /// Returns `Some(&str)` if a name was set, `None` otherwise.
        pub fn name(&self) -> Option<&str> {
            self.name.as_deref()
        }

        #[doc = concat!("Sets the name of this ", $type_desc, ".")]
        ///
        /// # Parameters
        ///
        #[doc = concat!("* `name` - The name to set for this ", $type_desc)]
        pub fn set_name(&mut self, name: &str) {
            self.name = Some(name.to_string());
        }
    };
}

pub(crate) use impl_common_name_methods;
