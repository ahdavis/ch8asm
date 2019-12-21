/*
 * conv_filename.rs
 * Defines a function that converts a filename from .c8a to .c8
 * Created on 12/21/2019
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
use std::ffi::OsString;
use std::path::Path;

/// Converts a filename extension from source (`.c8a`) to binary (`.c8`) 
///
/// # Argument
///
/// * `fname` - The filename to convert
/// 
/// # Returns
///
/// The argument filename with the extension `.c8`
pub fn conv_filename(fname: &str) -> String {
    //create the path
    let path = Path::new(fname);

    //get the filename without the extension
    let tmp = OsString::from(path.file_stem().unwrap());
    let mut ret = tmp.into_string().unwrap();

    //append the new extension
    ret.push_str(".c8");

    //and return the final filename
    return ret;
}

//unit tests
#[cfg(test)]
mod tests {
    //import the conv_filename function
    use super::*;

    //this test checks filename conversion
    #[test]
    fn test_filename_conversion() {
        let fname = "hello.c8a";
        let fnc = conv_filename(fname);
        assert_eq!(fnc.as_str(), "hello.c8");
    }
}

//end of file
