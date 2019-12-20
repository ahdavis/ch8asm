/*
 * addr_error.rs
 * Defines an error created when a label does not have a defined address
 * Created on 12/11/2019
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

//usage statement
use std::fmt;

/// Created when a label does not have a defined address
pub struct AddrError {
    /// The label that an address was not defined for
    label: String 
}

//implementation
impl AddrError {
    /// Constructs a new `AddrError` instance
    ///
    /// # Argument
    ///
    /// * `bad_label` - The label that the error was triggered by 
    /// 
    /// # Returns
    ///
    /// A new `AddrError` instance with the given properties
    pub fn new(bad_label: &str) -> Self {
        return AddrError {
            label: String::from(bad_label)
        };
    }
}

//Debug implementation
impl fmt::Debug for AddrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

//Display implementation
impl fmt::Display for AddrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not get the address of the label {}", 
               self.label)
    }
}

//end of file
