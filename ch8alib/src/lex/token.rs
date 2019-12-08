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

/// A program token
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

//end of file
