/*
 * prep_lexer.rs
 * Defines a struct that represents a preprocessor lexer
 * Created on 12/9/2019
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
use super::Token;
use super::TokenType;
use super::super::util::constants;
use super::super::util::Variant;
use super::super::error::LexerError;

/// A preprocessor lexer
pub struct PrepLexer {
    /// The text being lexed
    text: String,

    /// The current character being lexed
    cur_char: char,

    /// The current position in the `text`
    pos: usize,

    /// The current line number being lexed
    line: u32,

    /// The current column number being lexed
    col: u32,

    /// The current address being used to create labels
    addr: u16,

    /// The current number of processed nibbles
    nib_count: u32 
}

//implementation
impl PrepLexer {
    /// Constructs a new `PrepLexer` instance
    ///
    /// # Argument
    ///
    /// * `new_text` - The text to preprocess
    /// 
    /// # Returns
    ///
    /// A new `PrepLexer` instance with the given text
    pub fn new(new_text: &str) -> Self {
        return PrepLexer {
            text: String::from(new_text),
            cur_char: new_text.chars().nth(0).unwrap(),
            pos: 0,
            line: 1,
            col: 1,
            addr: constants::MEM_START,
            nib_count: 0
        };
    }

    /// Gets the current address being processed
    ///
    /// # Returns
    ///
    /// The current address being processed
    pub fn get_address(&self) -> u16 {
        return self.addr;
    }

    /// Gets the current line being processed
    ///
    /// # Returns
    ///
    /// The current line number being processed
    pub fn get_line(&self) -> u32 {
        return self.line;
    }

    /// Gets the current column being processed
    ///
    /// # Returns
    ///
    /// The current column number being processed
    pub fn get_column(&self) -> u32 {
        return self.col;
    }

    /// Gets the next `Token` consumed from the input
    ///
    /// # Returns
    ///
    /// The next `Token` consumed from the input, wrapped
    /// in a `Result`.
    pub fn get_next_token(&mut self) -> Result<Token, LexerError> {
        //loop and process the text
        while self.cur_char != '\0' {
            //process whitespace
            if self.cur_char.is_ascii_whitespace() {
                self.skip_whitespace();
                continue;
            }

            //process commas
            if self.cur_char == ',' {
                self.advance();
                continue;
            }

            //process periods
            if self.cur_char == '.' {
               self.advance();
               continue;
            }

            //process comments
            if self.cur_char == constants::COMMENT_CHAR {
                self.consume_comment();
                continue;
            }

            //process register references
            if (self.cur_char == 'V') || (self.cur_char == 'I') {
                self.consume_register();
                self.update_addr();
                continue;
            }

            //process instructions
            if self.cur_char.is_ascii_alphabetic() {
                self.consume_instr();
                self.update_addr();
                continue;
            }

            //process decimal literals
            if self.cur_char == constants::DEC_LIT_CHAR {
                self.consume_dec_lit();
                self.update_addr();
                continue;
            }

            //process hex literals
            if self.cur_char == constants::HEX_LIT_CHAR {
                self.consume_hex_lit();
                self.update_addr();
                continue;
            }

            //process binary literals
            if self.cur_char == constants::BIN_LIT_CHAR {
                self.consume_bin_lit();
                self.update_addr();
                continue;
            }

            //process labels
            if self.cur_char == '_' {
                //get the label text
                let lbl = self.consume_label();

                //determine whether it's a definition
                let lidx = lbl.len() - 1;
                let last = lbl.chars().nth(lidx).unwrap();
                if last == ':' { //is a definition
                    //get the string up to the colon
                    let tlbl = &lbl[0..lidx];

                    //and return a token
                    return Ok(Token::new(TokenType::LblDef,
                                Variant::Text(String::from(tlbl))));
                } else { //is not a definition
                    //advance past the label reference
                    self.nib_count += 2;

                    //update the address
                    self.update_addr();

                    //and return the label token
                    return Ok(Token::new(TokenType::Label,
                                    Variant::Text(lbl)));
                }
            }

            //if control reaches here, then an unknown 
            //character was found, so we return an error
            return Err(LexerError::new(self.line, self.col, 
                                        self.cur_char));
        }

        //if control reaches here, then
        //the end of input was found
        return Ok(Token::new(TokenType::EndOfInput, 
                             Variant::Text(String::from(""))));
    }

    /// Advances the lexer to the next character
    ///
    /// # Panics
    ///
    /// This method will panic if the next character
    /// cannot be retrieved from the text.
    fn advance(&mut self) {
        self.pos += 1; //advance the position

        //and handle end of input
        if self.pos > (self.text.len() - 1) {
            self.cur_char = '\0';
        } else {
            self.cur_char = self.text.chars().nth(self.pos).unwrap();
        }
    }

    /// Skips whitespace in the input
    fn skip_whitespace(&mut self) {
        loop {
            //check for end of input
            if self.cur_char == '\0' {
                break;
            }

            //ensure that the character is a space
            if !self.cur_char.is_ascii_whitespace() {
                break;
            }

            //check for newlines
            if (self.cur_char == '\n') || (self.cur_char == '\r') {
                self.line += 1;
                self.col = 1;
                self.advance();
            }

            //and advance the lexer
            self.advance();
        }
    }

    /// Consumes a comment from the input
    fn consume_comment(&mut self) {
        self.advance(); //advance past the semicolon

        //and consume to the end of the line
        while (self.cur_char != '\n') && (self.cur_char != '\r') &&
              (self.cur_char != '\0') {
            self.advance();
        }
    }

    /// Updates the address field
    fn update_addr(&mut self) {
        if self.nib_count >= 2 {
            let adv = (self.nib_count as f32 / 2.0).floor() as u16;
            self.nib_count = 0;
            self.addr += adv;
        }
    }

    /// Consumes a register refrence
    fn consume_register(&mut self) {
        //check to see what register it is
        if self.cur_char != 'I' {
            self.advance();
        }

        //advance through the register
        self.advance();

        //and update the nibble count
        self.nib_count += 1;
    }

    /// Consumes an instruction
    fn consume_instr(&mut self) {
        let mut op = String::new();
        while (self.cur_char.is_ascii_alphabetic() 
               || self.cur_char == '.') 
            && !self.cur_char.is_whitespace() {
            op.push(self.cur_char.to_ascii_uppercase());
            self.advance();
        }

        //increment the nibble count
        //CLS (0x00E0) and RET (0x00EE) are the only
        //2-byte instructions
        if (op == "CLS") || (op == "RET") {
            self.nib_count += 4;
        } else {
            self.nib_count += 2;
        }
    }

    /// Consumes a decimal integer literal
    fn consume_dec_lit(&mut self) {
        //advance past the sigil
        self.advance();

        //advance through the digits
        let mut sum = String::new();
        while self.cur_char.is_ascii_digit() {
            sum.push(self.cur_char);
            self.advance();
        }

        //calculate the nibbles in the sum
        let bits = sum.parse::<f32>().unwrap().log2().ceil();
        let nibs_advanced = (bits / 4.0).ceil() as u32;
        
        //and advance the nib count
        self.nib_count += nibs_advanced;
    }

    /// Consumes a hex integer literal
    fn consume_hex_lit(&mut self) {
        //advance past the sigil
        self.advance();

        //advance through the digits
        let mut nibs_advanced: u32 = 0;
        while self.cur_char.is_ascii_hexdigit() {
            nibs_advanced += 1;
            self.advance();
        }

        //and update the nibble count
        self.nib_count += nibs_advanced;
    }

    /// Consumes a binary literal
    fn consume_bin_lit(&mut self) {
        //advance past the sigil
        self.advance();

        //loop and consume the binary
        for _i in 0..8 {
            self.advance();
        }

        //and advance the address
        self.nib_count += 2;
    }

    /// Consumes a label
    fn consume_label(&mut self) -> String {
        //create the return value
        let mut ret = String::from("_");

        //advance past the underscore
        self.advance();

        //loop and generate the label
        while self.cur_char.is_ascii_alphabetic() &&
            !self.cur_char.is_ascii_whitespace() {
            ret.push(self.cur_char);
            self.advance();
        }

        //check for a colon after the label 
        //which indicates a label definition
        if self.cur_char == ':' {
            ret.push(':');
            self.advance();
        }

        //and return the generate label
        return ret;
    } 
}

//unit tests
#[cfg(test)]
mod tests {
    //import the PrepLexer struct
    use super::*;

    //define strings to be lexed
    const LEX_STR: &str = "MOV V1, V2 
                    _hex:
                    $FC00
                    _bin:
                    %01011010
                    _dec:
                    #255
                    _start: 
                    CLS 
                    CALL _start 
                    RET"; 
    const ERR_STR: &str = "MOV V1, @";

    //this test checks error generation
    #[test]
    #[should_panic]
    fn test_err_gen() {
        let mut lex = PrepLexer::new(ERR_STR);
        let _tok = lex.get_next_token().unwrap();
    }

    //this test checks token and address generation
    #[test]
    fn test_token_gen() {
        let mut lex = PrepLexer::new(LEX_STR);
        let mut tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::LblDef);
        assert_eq!(lex.get_address(), constants::MEM_START + 2);
        tok = lex.get_next_token().unwrap();
        let lbl = match tok.get_value() {
            Variant::Text(l) => l,
            _ => panic!("Bad token")
        };
        assert_eq!(tok.get_type(), TokenType::LblDef);
        assert_eq!(lex.get_address(), constants::MEM_START + 4);
        assert_eq!(lbl.as_str(), "_bin");
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::LblDef);
        assert_eq!(lex.get_address(), constants::MEM_START + 5);
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::LblDef);
        assert_eq!(lex.get_address(), constants::MEM_START + 6);
        tok = lex.get_next_token().unwrap();
        let l2 = match tok.get_value() {
            Variant::Text(l) => l,
            _ => panic!("Bad token")
        };
        assert_eq!(tok.get_type(), TokenType::Label);
        assert_eq!(lex.get_address(), constants::MEM_START + 10);
        assert_eq!(l2, "_start");
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::EndOfInput);
        assert_eq!(lex.get_address(), constants::MEM_START + 12);
    }
}

//end of file
