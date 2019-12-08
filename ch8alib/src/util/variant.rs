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

//import
use std::cmp;

/// Contains a single variable of various types
#[derive(Clone)]
pub enum Variant {
    /// A single byte 
    Byte(u8),

    /// A two-byte word
    Word(u16),

    /// A string of text
    Text(String)
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

//end of file
