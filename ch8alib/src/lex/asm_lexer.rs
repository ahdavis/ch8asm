/*
 * asm_lexer.rs
 * Defines a class that lexes assembly code
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

//usage statements
use super::super::util::constants;
use super::Token;
use super::TokenType;
use super::super::error::LexerError;
use super::super::util::Variant;

/// Lexes Chip-8 assembly code
pub struct AsmLexer {
    /// The code being lexed
    text: String,

    /// The current position in the text
    pos: usize,

    /// The current character being lexed
    cur_char: char,

    /// The current line being lexed
    line: u32,

    /// The current column being lexed
    col: u32 
}

//implementation
impl AsmLexer {
    /// Constructs a new `AsmLexer` instance
    ///
    /// # Argument
    ///
    /// * `new_text` - The text to lex
    /// 
    /// # Returns
    ///
    /// A new `AsmLexer` instance with the given text
    pub fn new(new_text: &str) -> Self {
        return AsmLexer {
            text: String::from(new_text),
            pos: 0,
            cur_char: new_text.chars().nth(0).unwrap(),
            line: 1,
            col: 1
        };
    }

    /// Gets the current line being lexed
    /// 
    /// # Returns
    ///
    /// The current line number being lexed
    pub fn get_line(&self) -> u32 {
        return self.line;
    }

    /// Gets the current column being lexed
    ///
    /// # Returns
    ///
    /// The current column number being lexed
    pub fn get_column(&self) -> u32 {
        return self.col;
    }

    /// Gets the next `Token` consumed from the input
    ///
    /// # Returns
    ///
    /// The next `Token` consumed from the input,
    /// wrapped in a `Result`
    pub fn get_next_token(&mut self) -> Result<Token, LexerError> {
        //loop and lex the text
        while self.cur_char != '\0' {
            //handle whitespace
            if self.cur_char.is_ascii_whitespace() {
                self.skip_whitespace();
                continue;
            }

            //handle commas
            if self.cur_char == ',' {
                self.advance();
                return Ok(Token::new(TokenType::Comma,
                                    Variant::Text(String::from(","))));
            }

            //handle periods
            if self.cur_char == '.' {
                self.advance();
                return Ok(Token::new(TokenType::Period,
                                     Variant::Text(String::from("."))));
            }

            //handle comments
            if self.cur_char == constants::COMMENT_CHAR {
                self.consume_comment();
                continue;
            }

            //handle register references
            if (self.cur_char.to_ascii_uppercase() == 'V') ||
                (self.cur_char.to_ascii_uppercase() == 'I') {
                return Ok(Token::new(TokenType::Register,
                                     Variant::Text(self.register())));
            }

            //handle labels
            if self.cur_char == '_' {
                //get the label text
                let lbl = self.label();

                //determine whether it's a definition
                let lidx = lbl.len() - 1;
                let last = lbl.chars().nth(lidx).unwrap();
                if last == ':' {
                    return Ok(Token::new(TokenType::LblDef,
                            Variant::Text(String::from(&lbl[0..lidx]))));
                } else {
                    return Ok(Token::new(TokenType::Label,
                                    Variant::Text(lbl)));
                }
            }

            //handle symbols
            if self.cur_char.is_ascii_alphabetic() {
                //get the symbol
                let sym = self.symbol();

                //check to see if it's a skiptype
                if ((sym == "EQ") || (sym == "NE")) ||
                    ((sym == "KEY") || (sym == "NOKEY")) {
                    return Ok(Token::new(TokenType::SkipCond,
                                    Variant::Text(sym)));
                } else {
                    return Ok(Token::new(TokenType::Instruction,
                                    Variant::Text(sym)));
                }
            }

            //handle decimal literals
            if self.cur_char == constants::DEC_LIT_CHAR {
                return Ok(Token::new(TokenType::DecLit,
                                     Variant::Word(self.dec_lit())));
            }

            //handle hex literals
            if self.cur_char == constants::HEX_LIT_CHAR {
                return Ok(Token::new(TokenType::HexLit,
                                     Variant::Word(self.hex_lit())));
            }

            //handle binary literals
            if self.cur_char == constants::BIN_LIT_CHAR {
                return Ok(Token::new(TokenType::BinLit,
                                     Variant::Byte(self.bin_lit())));
            }

            //if control reaches here, then
            //an unknown character was found
            //so we return an error
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
        //update the position
        self.pos += 1;

        //and get the next character
        if self.pos > (self.text.len() - 1) {
            self.cur_char = '\0';
        } else {
            self.col += 1;
            self.cur_char = self.text.chars().nth(self.pos).unwrap();
        }
    }

    /// Skips whitespace in the text
    fn skip_whitespace(&mut self) {
        loop {
            //check for end of input
            if self.cur_char == '\0' {
                break;
            }

            //check for non whitespace characters
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

    /// Consumes a comment in the text
    fn consume_comment(&mut self) {
        self.advance(); //advance past the semicolon

        //and consume to the end of the line
        while (self.cur_char != '\n') && (self.cur_char != '\r')
                && (self.cur_char != '\0') {
            self.advance();
        }
    }

    /// Lexes a text symbol in the input
    /// 
    /// # Returns
    ///
    /// An uppercase string containing the symbol
    fn symbol(&mut self) -> String {
        //declare the return symbol
        let mut ret = String::new();
        
        //loop and collect the symbol
        while self.cur_char.is_ascii_alphabetic() &&
                (self.cur_char != '\0') {
            ret.push(self.cur_char);
            self.advance();
        }

        //and return the symbol
        return ret;
    }

    /// Lexes a label in the input
    ///
    /// # Returns
    ///
    /// The label consumed from the input
    fn label(&mut self) -> String {
        //create the return value
        let mut ret = String::new();

        //append the underscore
        //and advance past it
        ret.push('_');
        self.advance();

        //loop and collect the label
        while self.cur_char.is_ascii_alphabetic() &&
                (self.cur_char != '\0') {
            ret.push(self.cur_char.to_ascii_uppercase());
            self.advance();
        }

        //check for a colon
        if self.cur_char == ':' {
            ret.push(':');
            self.advance();
        }

        //and return the collected label
        return ret;
    }

    /// Lexes a register reference in the input
    /// 
    /// # Returns
    ///
    /// The register consumed from the input
    fn register(&mut self) -> String {
        if self.cur_char == 'I' {
            self.advance();
            return String::from("I");
        } else {
            let mut ret = String::from("V");
            self.advance();
            ret.push(self.cur_char);
            self.advance();
            return ret;
        }
    }

    /// Lexes a decimal literal in the text
    /// 
    /// # Returns
    ///
    /// The integer consumed from the text 
    fn dec_lit(&mut self) -> u16 {
        //advance past the hash sign
        self.advance();

        //loop and generate the integer string
        let mut buf = String::new();
        while self.cur_char.is_ascii_digit()
            && (self.cur_char != '\0') {
            buf.push(self.cur_char);
            self.advance();
        }

        //convert the string to an integer
        let ret = u16::from_str_radix(&buf, 10).unwrap();

        //and return it
        return ret;
    }

    /// Lexes a hexadecimal literal in the text
    /// 
    /// # Returns
    ///
    /// The integer consumed from the text 
    fn hex_lit(&mut self) -> u16 {
        //advance past the hash sign
        self.advance();

        //loop and generate the integer string
        let mut buf = String::new();
        while self.cur_char.is_ascii_hexdigit()
            && (self.cur_char != '\0') {
            buf.push(self.cur_char);
            self.advance();
        }

        //convert the string to an integer
        let ret = u16::from_str_radix(&buf, 16).unwrap();

        //and return it
        return ret;
    }
 
    /// Lexes a binary literal in the text 
    ///
    /// # Returns
    ///
    /// The byte consumed from the text 
    fn bin_lit(&mut self) -> u8 {
        //advance past the percent sign
        self.advance();

        //declare the buffer
        let mut buf = String::new();

        //loop and generate the string
        for _i in 0..8 {
            buf.push(self.cur_char);
            self.advance();
        }

        //convert it to a byte
        let ret = u8::from_str_radix(&buf, 2).unwrap();

        //and return the byte
        return ret;
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the struct
    use super::*;

    //define code strings to be lexed
    const LEX_STR: &str = "MOV V1, V2
                            _hex:
                            $FC00
                            _bin: 
                            %11111111
                            _dec:
                            #212
                            _start:
                            CLS
                            SKIP.NE V0, V1 
                            CALL _start 
                            RET";
    const ERR_STR: &str = "MOV V1, @";

    //this test checks error generation
    #[test]
    #[should_panic]
    fn test_err_gen() {
        let mut lex = AsmLexer::new(ERR_STR);
        let mut _tok = lex.get_next_token().unwrap();
        _tok = lex.get_next_token().unwrap();
        _tok = lex.get_next_token().unwrap();
        _tok = lex.get_next_token().unwrap();
    }
    
    //this test checks token generation
    #[test]
    fn test_token_gen() {
        let mut lex = AsmLexer::new(LEX_STR);
        let mut tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::Instruction);
        assert_eq!(tok.get_value(), Variant::Text(String::from("MOV")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::Register);
        assert_eq!(tok.get_value(), Variant::Text(String::from("V1")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::Comma);
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::Register);
        assert_eq!(tok.get_value(), Variant::Text(String::from("V2")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::LblDef);
        assert_eq!(tok.get_value(), Variant::Text(String::from("_HEX")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::HexLit);
        assert_eq!(tok.get_value(), Variant::Word(0xFC00));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::LblDef);
        assert_eq!(tok.get_value(), Variant::Text(String::from("_BIN")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::BinLit);
        assert_eq!(tok.get_value(), Variant::Byte(0xFF));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::LblDef);
        assert_eq!(tok.get_value(), Variant::Text(String::from("_DEC")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::DecLit);
        assert_eq!(tok.get_value(), Variant::Word(212));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::LblDef);
        assert_eq!(tok.get_value(), Variant::Text(
                                        String::from("_START")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::Instruction);
        assert_eq!(tok.get_value(), Variant::Text(String::from("CLS")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::Instruction);
        assert_eq!(tok.get_value(), Variant::Text(String::from("SKIP")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::Period);
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::SkipCond);
        assert_eq!(tok.get_value(), Variant::Text(String::from("NE")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::Register);
        assert_eq!(tok.get_value(), Variant::Text(String::from("V0")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::Comma);
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::Register);
        assert_eq!(tok.get_value(), Variant::Text(String::from("V1")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::Instruction);
        assert_eq!(tok.get_value(), Variant::Text(String::from("CALL")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::Label);
        assert_eq!(tok.get_value(), Variant::Text(
                                            String::from("_START")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::Instruction);
        assert_eq!(tok.get_value(), Variant::Text(String::from("RET")));
        tok = lex.get_next_token().unwrap();
        assert_eq!(tok.get_type(), TokenType::EndOfInput);
    }
}

//end of file
