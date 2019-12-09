/*
 * constants.rs
 * Defines constants for ch8asm
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

//no usage statements

/// The address of the start of program memory
pub const MEM_START: u16 = 0x0200;

/// The character that denotes the start of a comment
pub const COMMENT_CHAR: char = ';';

/// The character that denotes the start of a decimal literal
pub const DEC_LIT_CHAR: char = '#';

/// The character that denotes the start of a hex literal
pub const HEX_LIT_CHAR: char = '$';

/// The character that denotes the start of a binary literal
pub const BIN_LIT_CHAR: char = '%';

//end of file
