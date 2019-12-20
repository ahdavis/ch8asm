/*
 * opcode_error.rs
 * Defines an error generated when an instruction does not map to an opcode
 * Created on 12/16/2019
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

/// Created when an instruction string doesn't map to a valid opcode
pub struct OpcodeError {
    /// The instruction string that generated the error
    bad_instr: String,

    /// The line number the instruction was found on
    line: u32,

    /// The column number the instruction was found on
    col: u32 
}

//Implementation
impl OpcodeError {
    /// Constructs a new `OpcodeError` instance
    ///
    /// # Arguments
    ///
    /// * `new_instr` - The instruction that generated the error
    /// * `new_line` - The line number of the bad instruction
    /// * `new_col` - The column number of the bad instruction
    /// 
    /// # Returns
    ///
    /// A new `OpcodeError` instance with the given properties
    pub fn new(new_instr: &str, new_line: u32, new_col: u32) -> Self {
        return OpcodeError {
            bad_instr: String::from(new_instr),
            line: new_line,
            col: new_col 
        };
    }
}

//Debug implementation
impl fmt::Debug for OpcodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

//Display implementation
impl fmt::Display for OpcodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}:{}): unknown instruction {}",
                self.line, self.col, self.bad_instr)
    }
}

//end of file
