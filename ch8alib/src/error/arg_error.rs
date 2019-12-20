/*
 * arg_error.rs
 * Defines an error that is generated when an instruction is 
 * given bad arguments
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
use std::fmt;
use ch8_isa::data::Register;

/// Generated when an instruction is given bad arguments
pub struct ArgError {
    /// The bad argument
    bad_arg: Register,

    /// The instruction that had the bad argument
    instr: String,

    /// The line number of the bad argument
    line: u32,

    /// The column number of the bad argument
    col: u32 
}

//implementation
impl ArgError {
    /// Constructs a new `ArgError` instance
    ///
    /// # Arguments
    ///
    /// * `new_arg` - The bad argument
    /// * `new_instr` - The instruction that had the bad argument 
    /// * `new_line` - The line number of the bad argument 
    /// * `new_col` - The column number of the bad argument
    /// 
    /// # Returns
    ///
    /// A new `ArgError` instance with the given properties
    pub fn new(new_arg: &Register, new_instr: &str,
               new_line: u32, new_col: u32) -> Self {
        return ArgError {
            bad_arg: new_arg.clone(),
            instr: String::from(new_instr),
            line: new_line,
            col: new_col 
        };
    }
}

//Debug implementation
impl fmt::Debug for ArgError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
   }
}

//Display implementation
impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}:{}): bad argument {:?} for {} instruction",
                self.line, self.col, self.bad_arg, self.instr)
    }
}

//end of file
