/*
 * addr_table.rs
 * Defines a struct that matches labels to addresses
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

//usage statements
use std::collections::HashMap;
use super::super::error::AddrError;

/// Manages label-to-address relationships
pub struct AddrTable {
    /// The number of entries in the table
    size: u32,

    /// The table data
    data: HashMap<String, u16>
}

//implementation
impl AddrTable {
    /// Constructs a new `AddrTable` instance 
    /// 
    /// # Returns
    ///
    /// A new `AddrTable` instance with no data
    pub fn new() -> Self {
        return AddrTable {
            size: 0,
            data: HashMap::new()
        };
    }

    /// Gets the number of entries in the table
    /// 
    /// # Returns
    ///
    /// The number of entries in the table
    pub fn get_size(&self) -> u32 {
        return self.size;
    }

    /// Adds an entry to the table
    /// 
    /// # Arguments
    ///
    /// * `label` - The label to add to the table
    /// * `addr` - The address corresponding to the label
    pub fn add_entry(&mut self, label: &str, addr: u16) {
        self.data.insert(String::from(label), addr);
        self.size += 1;
    }

    /// Determines whether a given label exists in the table
    /// 
    /// # Argument
    ///
    /// * `label` - The label to check
    ///
    /// # Returns
    ///
    /// Whether the given label exists in the table
    pub fn has_entry(&self, label: &str) -> bool {
        return self.data.contains_key(label);
    }

    /// Gets the address for a given label
    /// 
    /// # Argument
    ///
    /// * `label` - The label to get the address for
    /// 
    /// # Returns
    ///
    /// `Ok(address)` if the label has an entry, 
    /// `Err(AddrError)` otherwise
    pub fn get_entry(&self, label: &str) -> Result<u16, AddrError> {
        //get the address for the label
        return match self.data.get(label) {
            Some(l) => Ok(*l),
            None => Err(AddrError::new(label))
        };
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the AddrTable struct
    use super::*;

    //this test checks that the size field updates
    #[test]
    fn test_size_updates() {
        let mut t = AddrTable::new();
        assert_eq!(t.get_size(), 0);
        t.add_entry("_label", 0x0FFF);
        assert_eq!(t.get_size(), 1);
    }

    //this test checks adding and retrieving labels
    #[test]
    fn test_add_retrieve_labels() {
        let mut t = AddrTable::new();
        t.add_entry("_label", 0x0FFF);
        let addr = t.get_entry("_label").unwrap();
        assert_eq!(addr, 0x0FFF);
    }

    //this test checks the has_entry method
    #[test]
    fn test_has_entry() {
        let mut t = AddrTable::new();
        assert_eq!(t.has_entry("_label"), false);
        t.add_entry("_label", 0x0FFF);
        assert_eq!(t.has_entry("_label"), true);
    }
}

//end of file
