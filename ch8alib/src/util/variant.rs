/*
 * variant.rs
 * Defines an enum that manages values of different types
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
use std::cmp;
use super::super::error::VariantError;

/// Contains a single variable of various types
#[derive(Clone, Debug)]
pub enum Variant {
    /// A single byte 
    Byte(u8),

    /// A two-byte word
    Word(u16),

    /// A string of text
    Text(String)
}

//implementation
impl Variant {
    /// Extracts the byte value of the `Variant`
    /// 
    /// # Returns
    ///
    /// The byte value of the `Variant`, wrapped in a `Result`
    pub fn as_byte(&self) -> Result<u8, VariantError> {
        return match *self {
            Variant::Byte(b) => Ok(b),
            Variant::Word(_w) => Err(VariantError::new("byte", "word")),
            Variant::Text(ref _t) => Err(VariantError::new(
                                        "byte", "text"))
        };
    }

    /// Extracts the word value of the `Variant`
    ///
    /// # Returns
    ///
    /// The word value of the `Variant`, wrapped in a `Result`
    pub fn as_word(&self) -> Result<u16, VariantError> {
        return match *self {
            Variant::Word(w) => Ok(w),
            Variant::Byte(_b) => Err(VariantError::new("word", "byte")),
            Variant::Text(ref _t) => Err(VariantError::new(
                                            "word", "text"))
        };
    }

    /// Extracts the text value of the `Variant`
    /// 
    /// # Returns
    ///
    /// The text value of the `Variant`, wrapped in a `Result`
    pub fn as_text(&self) -> Result<String, VariantError> {
        return match *self {
            Variant::Text(ref t) => Ok(t.clone()),
            Variant::Byte(_b) => Err(VariantError::new("text", "byte")),
            Variant::Word(_w) => Err(VariantError::new("text", "word"))
        };
    }

}

//PartialEq implementation
impl cmp::PartialEq for Variant {
    fn eq(&self, rhs: &Self) -> bool {
        return match *self {
            Variant::Byte(ref b1) => {
                match *rhs {
                    Variant::Byte(ref b2) => (b1 == b2),
                    _ => false
                }
            },
            Variant::Word(ref w1) => {
                match *rhs {
                    Variant::Word(ref w2) => (w1 == w2),
                    _ => false 
                }
            },
            Variant::Text(ref t1) => {
                match *rhs {
                    Variant::Text(ref t2) => (t1 == t2),
                    _ => false 
                }
            }
        };
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the Variant enum
    use super::*;

    //this test checks equality comparisons
    #[test]
    fn test_equality() {
        let v1 = Variant::Byte(0xFF);
        let v2 = Variant::Byte(0xFF);
        assert_eq!(v1, v2);
        assert_eq!(v1, v1);
    }

    //this test checks inequality comparisons
    #[test]
    fn test_inequality() {
        let v1 = Variant::Word(0xFC00);
        let v2 = Variant::Word(0xDEAD);
        let v3 = Variant::Text(String::from("Hello World!"));
        assert_ne!(v1, v2);
        assert_ne!(v1, v3);
    }

    //this test checks unwrapping Variants
    #[test]
    fn test_unwrap() {
        let v1 = Variant::Byte(0xFF);
        assert_eq!(v1.as_byte().unwrap(), 0xFF);
        let v2 = Variant::Word(0xFC00);
        assert_eq!(v2.as_word().unwrap(), 0xFC00);
        let v3 = Variant::Text(String::from("Hello"));
        assert_eq!(v3.as_text().unwrap(), String::from("Hello"));
    }
}

//end of file
