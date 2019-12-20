/*
 * variant_error.rs
 * Defines an error generated when a variant is unwrapped wrongly
 * Created on 12/12/2019
 * Created by Andrew Davis
 *
 * Copyright (C) 2019  Andrew Davis
 *
 * This program is free software: you can redistribute it and/or modify   
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//usage statements
use std::fmt;

/// Generated when a `Variant` is unwrapped wrongly
pub struct VariantError {
    /// The type that was expected by the `as` method
    expected: String,

    /// The actual type of the variant
    actual: String 
}

//Implementation
impl VariantError {
    /// Constructs a new `VariantError` instance
    ///
    /// # Arguments
    ///
    /// * `new_expected` - The expected type of the `Variant`
    /// * `new_actual` - The actual type of the `Variant`
    ///
    /// # Returns
    ///
    /// A new `VariantError` instance with the given properties
    pub fn new(new_expected: &str, new_actual: &str) -> Self {
        return VariantError {
            expected: String::from(new_expected),
            actual: String::from(new_actual)
        };
    }
}

//Debug implementation
impl fmt::Debug for VariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

//Display implementation
impl fmt::Display for VariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Attempted to unwrap a {} variant as a {} variant.",
               self.actual, self.expected)
    }
}

//end of file
