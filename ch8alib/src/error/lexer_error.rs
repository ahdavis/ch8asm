/*
 * lexer_error.rs
 * Defines an error created when a lexer encounters an error
 * Created on 12/8/2019
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

/// Created when a `Lexer` encounters an error
pub struct LexerError {
    /// The line of source code that
    /// the error was triggered on 
    line: u32,

    /// The column of source code that
    /// the error was triggered on
    col: u32,

    /// The character that triggered the error
    chr: char
}

//implementation
impl LexerError {
    /// Constructs a new `LexerError` instance
    ///
    /// # Arguments
    ///
    /// * `bad_line` - The line that the error was triggered on
    /// * `bad_col` - The column that the error was triggered on
    /// * `bad_char` - The character that triggered th error
    /// 
    /// # Returns
    ///
    /// A new `LexerError` instance with the given properties
    pub fn new(bad_line: u32, bad_col: u32, bad_char: char) -> Self {
        return LexerError {
            line: bad_line,
            col: bad_col,
            chr: bad_char 
        };
    }
}

//Debug implementation
impl fmt::Debug for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

//Display implementation
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}:{}): unknown character {}", 
               self.line, self.col, self.chr)
    }
}

//end of file
