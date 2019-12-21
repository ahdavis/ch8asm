/*
 * main.rs
 * Entry point for ch8asm
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

//crate import statements
extern crate ch8alib;

//usage statements
use std::env;
use std::path::Path;
use std::fs;
use ch8alib::codegen::Assembler;
use ch8alib::util::conv_filename;

//Entry point for the program
fn main() {
    //get the args list
    let args: Vec<String> = env::args().collect();

    //determine whether an argument was supplied
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    //get the filename
    let src = args[1].as_str();

    //ensure that it exists
    if !Path::new(src).exists() {
        println!("Couldn't open {}", src);
        return;
    }

    //read the file into a string
    let code = match fs::read_to_string(src) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    //get the name of the assembled binary
    let asm_name = conv_filename(src);

    //create the assembler
    let mut asm = match Assembler::new(code.as_str(), 
                                        asm_name.as_str()) {
        Ok(a) => a,
        Err(e) =>  {
            eprintln!("{}", e);
            return;
        }
    };

    //assemble the code
    let bin = match asm.assemble() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    //write the binary to its destination
    match bin.write_to_file() {
        Ok(_n) => {},
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    //and print out a success message
    println!("Successfully assembled {} into {}", src, asm_name);
}

//end of file
