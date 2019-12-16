/*
 * parse_error.rs
 * Defines an error generated when a parser encounters an error
 * Created on 12/13/2019
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
use super::super::lex::TokenType;
use std::fmt;

/// Generated when a Chip-8 parser encounters an error 
pub struct ParseError {
    /// The expected token
    expected: TokenType,

    /// The actual token
    actual: TokenType,

    /// The line number of the bad symbol
    line: u32,

    /// The column number of the bad symbol
    col: u32 
}

//implementation
impl ParseError {
    /// Constructs a new `ParseError` instance
    ///
    /// # Arguments
    ///
    /// * `new_expected` - The expected token type
    /// * `new_actual` - The actual token type
    /// * `new_line` - The line of code that triggered the error
    /// * `new_col` - The column of code that triggered the error
    /// 
    /// # Returns
    ///
    /// A new `ParseError` instance with the given properties
    pub fn new(new_expected: &TokenType, new_actual: &TokenType,
               new_line: u32, new_col: u32) -> Self {
        return ParseError {
            expected: new_expected.clone(),
            actual: new_actual.clone(),
            line: new_line,
            col: new_col 
        };
    }
}

//Debug implementation
impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

//Display implementation
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}:{}): Expected {}, found {}", 
               self.line, self.col, self.expected, self.actual)
    }
}

//end of file
