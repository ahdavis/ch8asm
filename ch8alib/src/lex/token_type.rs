/*
 * token_type.rs
 * Enumerates types of program tokens
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

//imports
use std::fmt;

/// The type of a program `Token`
#[derive(Clone)]
pub enum TokenType {
    /// An assembly instruction
    Instruction,

    /// An address label
    Label,

    /// A label definition
    LblDef,

    /// A register reference
    Register,

    /// A decimal integer literal
    DecLit,

    /// A hex integer literal
    HexLit,

    /// A binary byte literal
    BinLit,

    /// A skip condition
    SkipCond,

    /// A comma character
    Comma,

    /// A period character
    Period,

    /// End of source input
    EndOfInput
}

//implementation
impl TokenType {
    /// Gets the name of the type
    /// 
    /// # Returns
    ///
    /// The name of the type
    pub fn get_name(&self) -> &str {
        //determine the name of the enum value
        let name = match *self {
            TokenType::Instruction => "instruction",
            TokenType::Label => "label",
            TokenType::LblDef => "label definition",
            TokenType::Register => "register",
            TokenType::DecLit => "decimal literal",
            TokenType::HexLit => "hex literal",
            TokenType::BinLit => "binary literal",
            TokenType::SkipCond => "skip condition",
            TokenType::Comma => "comma",
            TokenType::Period => "period",
            TokenType::EndOfInput => "EOF"
        };

        //and return the name
        return name;
    }
}

//Debug implementation
impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write the name of the type
        write!(f, "{}", self.get_name())
    }
}

//Display implementation
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

//end of file
