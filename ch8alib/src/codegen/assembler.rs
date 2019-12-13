/*
 * assembler.rs
 * Defines a class that assembles Chip-8 assembly code
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

//crate import statement
extern crate ch8_isa;

//usage statements
use super::AddrTable;
use super::super::lex::AsmLexer;
use super::super::lex::Token;
use super::super::lex::TokenType;
use super::Preprocessor;
use super::super::error::AsmError;
use std::process::exit;
use ch8_isa::codegen::Binary;

/// Assembles Chip-8 binaries
pub struct Assembler {
    /// Lexes the source code 
    lexer: AsmLexer,

    /// The address table for labels in the source code
    addrs: AddrTable,

    /// The current `Token` being assembled
    cur_token: Token,

    /// The binary being created
    binary: Binary 
}

//implementation
impl Assembler {
    /// Constructs a new `Assembler` instance
    ///
    /// # Arguments
    ///
    /// * `code` - The source code to be assembled
    /// * `out_name` - The name of the output file
    ///
    /// # Returns
    ///
    /// A new `Assembler` instance with the given properties
    pub fn new(code: &str, out_name: &str) -> Self {
        
    }
}
