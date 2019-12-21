/*
 * skip_error.rs
 * Defines an error that is generated when a SKIP instruction is
 * given a bad skip condition
 * Created on 12/20/2019
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

//crate import
extern crate ch8_isa;

//usage statements
use ch8_isa::data::SkipType;
use std::fmt;


/// Generated when a `SKIP` instruction is given a bad condition
pub struct SkipError {
    /// The `SkipType` that caused the error 
    bad_type: SkipType,

    /// The line number that the error was found on
    line: u32,

    /// The column the error was found on
    col: u32 
}

//Implementation
impl SkipError {
    /// Constructs a new `SkipError` instance
    ///
    /// # Arguments
    ///
    /// * `new_type` - The `SkipType` that triggered the error
    /// * `new_line` - The line number the error was found on
    /// * `new_col` - The column number the error was found on
    /// 
    /// # Returns
    ///
    /// A new `SkipError` instance with the given properties
    pub fn new(new_type: SkipType, new_line: u32, new_col: u32) -> Self {
        return SkipError {
            bad_type: new_type,
            line: new_line,
            col: new_col 
        };
    }
}

//Debug implementation
impl fmt::Debug for SkipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

//Display implementation
impl fmt::Display for SkipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}:{}): Bad skip type {:?}",
                self.line, self.col, self.bad_type)
    }
}

//end of file
