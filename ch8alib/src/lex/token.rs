/*
 * token.rs
 * Defines a struct that represents a program token
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

//usage statements
use super::TokenType;
use super::super::util::Variant;
use std::cmp;

/// A program token
#[derive(Debug)]
pub struct Token {
    /// The type of the `Token`
    ttype: TokenType,

    /// The value of the `Token`
    value: Variant 
}

//implementation
impl Token {
    /// Constructs a new `Token` instance
    ///
    /// # Arguments
    ///
    /// * `new_type` - The type of the token
    /// * `new_value` - The value of the token
    /// 
    /// # Returns
    ///
    /// A new `Token` instance with the given properties
    pub fn new(new_type: TokenType, new_value: Variant) -> Self {
        return Token {
            ttype: new_type,
            value: new_value 
        };
    }

    /// Gets the type of the `Token`
    ///
    /// # Returns
    ///
    /// The type of the `Token`
    pub fn get_type(&self) -> TokenType {
        return self.ttype.clone();
    }

    /// Gets the value of the `Token`
    ///
    /// # Returns
    ///
    /// The value of the `Token`
    pub fn get_value(&self) -> Variant {
        return self.value.clone();
    }
}

//PartialEq implementation
impl cmp::PartialEq for Token {
    fn eq(&self, rhs: &Self) -> bool {
        return (self.ttype == rhs.ttype) && (self.value == rhs.value);
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the Token struct
    use super::*;

    //this test checks equality comparisons
    #[test]
    fn test_equality() {
        let t1 = Token::new(TokenType::DecLit, Variant::Word(0xDEAD));
        let t2 = Token::new(TokenType::DecLit, Variant::Word(0xDEAD));
        assert_eq!(t1, t2);
        assert_eq!(t1, t1);
    }

    //this test checks inequality comparisons
    #[test]
    fn test_inequality() {
        let t1 = Token::new(TokenType::Label, Variant::Word(0xFC00));
        let t2 = Token::new(TokenType::Register, Variant::Byte(0x5));
        assert_ne!(t1, t2);
    }
}

//end of file
