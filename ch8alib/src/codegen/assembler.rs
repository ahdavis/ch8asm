/*
 * assembler.rs
 * Defines a class that assembles Chip-8 assembly code
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

//crate import statement
extern crate ch8_isa;

//usage statements
use super::AddrTable;
use super::Preprocessor;
use super::super::lex::AsmLexer;
use super::super::lex::Token;
use super::super::lex::TokenType;
use super::super::error::AsmError;
use super::super::error::ParseError;
use super::super::error::OpcodeError;
use super::super::error::ArgError;
use super::super::error::SkipError;
use ch8_isa::codegen::Binary;
use ch8_isa::codegen::Instruction;
use ch8_isa::data;

/// Assembles Chip-8 binaries
pub struct Assembler {
    /// Lexes the source code 
    lexer: AsmLexer,

    /// The address table for labels in the source code
    addrs: AddrTable,

    /// The current `Token` being assembled
    cur_token: Token,

    /// The binary being created
    binary: Binary 
}

//implementation
impl Assembler {
    /// Constructs a new `Assembler` instance
    ///
    /// # Arguments
    ///
    /// * `code` - The source code to be assembled 
    /// * `name` - The name of the assembled binary 
    ///
    /// # Returns
    ///
    /// A new `Assembler` instance with the given properties,
    /// wrapped in a `Result`
    pub fn new(code: &str, name: &str) -> Result<Self, AsmError> {
        //create the binary
        let bin = match Binary::new(name) {
            Ok(b) => b,
            Err(be) => return Err(AsmError::Binary(be))
        };

        //create the preprocessor
        let mut prep = match Preprocessor::new(code) {
            Ok(p) => p,
            Err(le) => return Err(AsmError::Lexer(le))
        };

        //preprocess the code
        let new_addrs = match prep.process() {
            Ok(at) => at,
            Err(le) => return Err(AsmError::Lexer(le))
        };

        //create the lexer
        let mut lex = AsmLexer::new(code);

        //get the first token
        let tok = match lex.get_next_token() {
            Ok(t) => t,
            Err(le) => return Err(AsmError::Lexer(le))
        };

        //and return the instance
        return Ok(Assembler {
            lexer: lex,
            addrs: new_addrs,
            cur_token: tok,
            binary: bin 
        });
    }

    /// Assembles source code into a binary
    /// 
    /// # Returns
    ///
    /// The assembled binary, wrapped in a `Result`
    pub fn assemble(&mut self) -> Result<&mut Binary, AsmError> {
        //loop and generate code
        loop {
            //check for an EOF token
            if self.cur_token.get_type() == TokenType::EndOfInput {
                break;
            }

            //handle different tokens
            if self.cur_token.get_type() == TokenType::Instruction {
                //process the instruction
                let instr = self.instruction()?;

                //and add it to the binary
                match self.binary.add_instruction(&instr) {
                    Ok(()) => {},
                    Err(be) => return Err(AsmError::Binary(be))
                };
            } else if self.cur_token.get_type() == TokenType::BinLit {
                //get the byte
                let b = self.bin_lit()?;

                //and write it to the binary
                match self.binary.add_byte(b) {
                    Ok(()) => {},
                    Err(be) => return Err(AsmError::Binary(be))
                };
            } else if self.cur_token.get_type() == TokenType::DecLit {
                //get the decimal literal
                let dec = self.dec_lit()?;

                //determine what size the literal is
                if dec > 0xFF {
                    match self.binary.add_word(dec) {
                        Ok(()) => {},
                        Err(be) => return Err(AsmError::Binary(be))
                    };
                } else {
                    match self.binary.add_byte(dec as u8) {
                        Ok(()) => {},
                        Err(be) => return Err(AsmError::Binary(be))
                    };
                }
            } else if self.cur_token.get_type() == TokenType::HexLit {
                //get the decimal literal
                let hex = self.hex_lit()?;

                //determine what size the literal is
                if hex > 0xFF {
                    match self.binary.add_word(hex) {
                        Ok(()) => {},
                        Err(be) => return Err(AsmError::Binary(be))
                    };
                } else {
                    match self.binary.add_byte(hex as u8) {
                        Ok(()) => {},
                        Err(be) => return Err(AsmError::Binary(be))
                    };
                }
            } else if self.cur_token.get_type() == TokenType::LblDef {
                self.eat(&TokenType::LblDef)?;
            }
        }

        //and return the binary
        return Ok(&mut self.binary);
    }

    /// Verifies the current token and gets the next token
    /// 
    /// # Argument
    ///
    /// * `ttype` - The token type to verify against
    ///
    /// # Returns
    ///
    /// `Ok` if no errors occur, `Err(ParseError)` if the token
    /// is not verified properly, or `Err(LexerError)` if an
    /// unknown character is found
    fn eat(&mut self, ttype: &TokenType ) -> Result<(), AsmError> {
        //check the type
        if self.cur_token.get_type() == *ttype {
            let lr = self.lexer.get_next_token();
            return match lr {
                Err(le) => Err(AsmError::Lexer(le)),
                Ok(t) => {
                    self.cur_token = t;
                    Ok(())
                }
            };
        } else {
            return Err(AsmError::Parser(ParseError::new(
                        ttype, &self.cur_token.get_type(),
                        self.lexer.get_line(),
                        self.lexer.get_column())));
        }
    }

    /// Assembles an instruction
    /// 
    /// # Returns
    ///
    /// The assembled instruction, wrapped in a `Result`
    fn instruction(&mut self) -> Result<Instruction, AsmError> {
        //save the current token
        let save_token = self.cur_token.clone();

        //parse the instruction
        self.eat(&TokenType::Instruction)?;

        //get the instruction string
        let instr = save_token.get_value().as_text().unwrap();

        //and generate the instruction object
        return match instr.as_str() {
            "CLS" => Ok(Instruction::CLS),
            "RET" => Ok(Instruction::RET), 
            "JMP" => {
                //get the label address
                let addr = self.label()?;

                //create the data
                let data = data::JmpData::new(addr);

                //and return the instruction
                Ok(Instruction::JMP(data))
            },
            "CALL" => {
                //get the label address
                let addr = self.label()?;

                //create the data
                let data = data::CallData::new(addr);

                //and return the instruction
                Ok(Instruction::CALL(data))
            },
            "SKIP" => {
                //parse the period
                self.eat(&TokenType::Period)?;

                //get the skip type
                let st = self.skiptype()?;

                //get the first argument
                let vx = self.register()?;

                //ensure that VX is not the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                ArgError::new(&vx, "SKIP",
                                        self.lexer.get_line(),
                                        self.lexer.get_column())));
                }

                //parse a possible comma
                if (st == data::SkipType::Equals) ||
                    (st == data::SkipType::NotEquals) {
                    self.eat(&TokenType::Comma)?;
                }

                //save the tokentype
                let ttype = self.cur_token.get_type().clone();

                //and generate the instruction
                if ttype == TokenType::Register {
                    //parse the register
                    let vy = self.register()?;

                    //ensure that VY is not the I register
                    if vy == data::Register::I {
                        return Err(AsmError::Argument(
                                    ArgError::new(&vy, "SKIP",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                    }

                    //ensure that the skip type is not a key variant
                    if (st == data::SkipType::KeyUp) ||
                        (st == data::SkipType::KeyDown) {
                        return Err(AsmError::Skip(
                                    SkipError::new(st,
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                    }

                    //create the data
                    let data = data::SkipData::with_register(vx,
                                                             vy, st);

                    //and return it
                    Ok(Instruction::SKIP(data))
                } else if (ttype == TokenType::HexLit) ||
                            (ttype == TokenType::DecLit) ||
                            (ttype == TokenType::BinLit) {

                    //get the value
                    let nn16 = self.constant()?;
                    let nn = nn16 as u8; 

                    //ensure that the skip type is not a key variant
                    if (st == data::SkipType::KeyUp) ||
                        (st == data::SkipType::KeyDown) {
                        return Err(AsmError::Skip(
                                    SkipError::new(st,
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                    }

                    //create the data
                    let data = data::SkipData::with_constant(vx, nn, st);

                    //and return the instruction
                    Ok(Instruction::SKIP(data))
                } else {
                    //ensure that st is a key type
                    if (st != data::SkipType::KeyUp) &&
                        (st != data::SkipType::KeyDown) {
                        return Err(AsmError::Skip(
                                    SkipError::new(st,
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                    }

                    let data = data::SkipData::with_key(vx, st);

                    Ok(Instruction::SKIP(data))
                }
            },
            "MOV" => {
                //get the destination register
                let vx = self.register()?;

                //parse the comma
                self.eat(&TokenType::Comma)?;

                //save the token type
                let ttype = self.cur_token.get_type().clone();

                //handle source objects
                if ttype == TokenType::Register {
                    //ensure that the VX register is not the I register
                    if vx == data::Register::I {
                        return Err(AsmError::Argument(
                                    ArgError::new(&vx, "MOV",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                    }

                    //get the source register
                    let vy = self.register()?;

                    //ensure that the VY register is not the I register
                    if vy == data::Register::I {
                        return Err(AsmError::Argument(
                                    ArgError::new(&vy, "MOV",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                    }

                    //construct the data
                    let data = data::MovData::with_register(vx, vy);

                    //and return the instruction
                    Ok(Instruction::MOV(data))
                } else if ttype == TokenType::Label {
                    //ensure that vx is the `I` register
                    if vx != data::Register::I {
                        return Err(AsmError::Argument(
                                    ArgError::new(&vx, "MOV",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                    }

                    //parse the label
                    let addr = self.label()?;

                    //assemble the data
                    let data = data::MovData::with_constant(vx, addr);

                    //and return the instruction
                    Ok(Instruction::MOV(data))
                } else {
                    //get the constant
                    let cst = self.constant()?;

                    //construct the data 
                    let data = data::MovData::with_constant(vx, cst);

                    //and return the instruction
                    Ok(Instruction::MOV(data))
                } 
            },
            "ADD" => {
                //get the destination register
                let vx = self.register()?;
                
                //parse the comma
                self.eat(&TokenType::Comma)?;

                //determine the second argument type
                if self.cur_token.get_type() == TokenType::Register {
                    //get the second register
                    let vy = self.register()?;

                    //ensure that VY is not the I register
                    if vy == data::Register::I {
                        return Err(AsmError::Argument(
                                        ArgError::new(&vy, "ADD",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                    }

                    //construct the data
                    let data = data::AddData::with_register(vx, vy);

                    //and return the instruction
                    return Ok(Instruction::ADD(data));
                } else {
                    //get the constant
                    let cst = self.constant()?;

                    //ensure that VX is not the I register
                    if vx == data::Register::I {
                        return Err(AsmError::Argument(
                                    ArgError::new(&vx, "ADD",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                    }

                    //construct the data
                    let data = data::AddData::with_constant(vx, cst);

                    //and return the instruction
                    Ok(Instruction::ADD(data))
                }
            },
            "OR" => {
                //get the first argument register
                let vx = self.register()?;

                //ensure that it is not the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "OR",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //parse the comma
                self.eat(&TokenType::Comma)?;

                //get the second argument register
                let vy = self.register()?;

                //ensure that it is not the I register
                if vy == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vy, "OR",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::OrData::new(vx, vy);

                //and return the instruction
                Ok(Instruction::OR(data))
            },
            "AND" => {
                //get the first argument register
                let vx = self.register()?;

                //ensure that it is not the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "AND",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //parse the comma
                self.eat(&TokenType::Comma)?;

                //get the second argument register
                let vy = self.register()?;

                //ensure that it is not the I register
                if vy == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vy, "AND",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::AndData::new(vx, vy);

                //and return the instruction
                Ok(Instruction::AND(data))
            },
            "XOR" => {
                //get the first argument register
                let vx = self.register()?;

                //ensure that it is not the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "XOR",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //parse the comma
                self.eat(&TokenType::Comma)?;

                //get the second argument register
                let vy = self.register()?;

                //ensure that it is not the I register
                if vy == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vy, "XOR",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::XorData::new(vx, vy);

                //and return the instruction
                Ok(Instruction::XOR(data))
            },
            "SUB" => {
                //get the first argument register
                let vx = self.register()?;

                //ensure that it is not the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "SUB",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //parse the comma
                self.eat(&TokenType::Comma)?;

                //get the second argument register
                let vy = self.register()?;

                //ensure that it is not the I register
                if vy == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vy, "SUB",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::SubData::new(vx, vy);

                //and return the instruction
                Ok(Instruction::SUB(data))
            },
            "SHR" => {
                //get the argument register
                let vx = self.register()?;

                //ensure that it does not refer to the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "SHR",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::ShrData::new(vx);

                //and return the instruction
                Ok(Instruction::SHR(data))
            },
            "SUBN" => {
                //get the first argument register
                let vx = self.register()?;

                //ensure that it is not the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "SUBN",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //parse the comma
                self.eat(&TokenType::Comma)?;

                //get the second argument register
                let vy = self.register()?;

                //ensure that it is not the I register
                if vy == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vy, "SUBN",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::SubnData::new(vx, vy);

                //and return the instruction
                Ok(Instruction::SUBN(data))
            },
           "SHL" => {
                //get the argument register
                let vx = self.register()?;

                //ensure that it does not refer to the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "SHL",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::ShlData::new(vx);

                //and return the instruction
                Ok(Instruction::SHL(data))
            },
            "JPC" => {
                //determine which type of argument to use
                if self.cur_token.get_type() == TokenType::Label {
                    let addr = self.label()?;
                    let data = data::JpcData::new(addr);
                    Ok(Instruction::JPC(data))
                } else {
                    let addr = self.constant()?;
                    let data = data::JpcData::new(addr);
                    Ok(Instruction::JPC(data))
                }
            },
            "RAND" => {
                //get the destination register
                let vx = self.register()?;

                //ensure that it does not refer to the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "RAND",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //parse the comma
                self.eat(&TokenType::Comma)?;

                //get the limiting value
                let nn16 = self.constant()?;
                let nn = (nn16 & 0xFF) as u8;

                //construct the data
                let data = data::RandData::new(vx, nn);

                //and return the instruction
                Ok(Instruction::RAND(data))
            },
            "DRAW" => {
                //get the X register
                let vx = self.register()?;

                //ensure that it's not the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "DRAW",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //parse the comma
                self.eat(&TokenType::Comma)?;

                //get the Y register
                let vy = self.register()?;

                //ensure that it's not the I register
                if vy == data::Register::I {
                    return Err(AsmError::Argument(
                                        ArgError::new(&vy, "DRAW",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //parse the comma
                self.eat(&TokenType::Comma)?;

                //get the height
                let h16 = self.constant()?;
                let h = (h16 & 0xF) as u8;

                //construct the data
                let data = data::DrawData::new(vx, vy, h);

                //and return the instruction
                Ok(Instruction::DRAW(data))
            },
           "GDL" => {
                //get the argument register
                let vx = self.register()?;

                //ensure that it does not refer to the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "GDL",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::GdlData::new(vx);

                //and return the instruction
                Ok(Instruction::GDL(data))
            },
           "KEY" => {
                //get the argument register
                let vx = self.register()?;

                //ensure that it does not refer to the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "KEY",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::KeyData::new(vx);

                //and return the instruction
                Ok(Instruction::KEY(data))
            },
           "SDL" => {
                //get the argument register
                let vx = self.register()?;

                //ensure that it does not refer to the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "SDL",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::SdlData::new(vx);

                //and return the instruction
                Ok(Instruction::SDL(data))
            },
           "SND" => {
                //get the argument register
                let vx = self.register()?;

                //ensure that it does not refer to the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "SND",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::SndData::new(vx);

                //and return the instruction
                Ok(Instruction::SND(data))
            },
           "SCH" => {
                //get the argument register
                let vx = self.register()?;

                //ensure that it does not refer to the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "SCH",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::SchData::new(vx);

                //and return the instruction
                Ok(Instruction::SCH(data))
            },
           "BCD" => {
                //get the argument register
                let vx = self.register()?;

                //ensure that it does not refer to the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "BCD",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::BcdData::new(vx);

                //and return the instruction
                Ok(Instruction::BCD(data))
            },
            "RDP" => {
                //get the argument register
                let vx = self.register()?;

                //ensure that it does not refer to the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "RDP",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::RdpData::new(vx);

                //and return the instruction
                Ok(Instruction::RDP(data))
            },
           "RLD" => {
                //get the argument register
                let vx = self.register()?;

                //ensure that it does not refer to the I register
                if vx == data::Register::I {
                    return Err(AsmError::Argument(
                                    ArgError::new(&vx, "RLD",
                                            self.lexer.get_line(),
                                            self.lexer.get_column())));
                }

                //construct the data
                let data = data::RldData::new(vx);

                //and return the instruction
                Ok(Instruction::RLD(data))
            },
            _ => Err(AsmError::Opcode(OpcodeError::new(instr.as_str(),
                        self.lexer.get_line(),
                        self.lexer.get_column())))
        };
    }

    /// Assembles a constant literal
    ///
    /// # Returns
    ///
    /// The constant literal, wrapped in a `Result`
    fn constant(&mut self) -> Result<u16, AsmError> {
        //handle different types
        if self.cur_token.get_type() == TokenType::DecLit {
            let dec = self.dec_lit()?;
            return Ok(dec);
        } else if self.cur_token.get_type() == TokenType::HexLit {
            let hex = self.hex_lit()?;
            return Ok(hex);
        } else { //binary literal expected
            let bin = self.bin_lit()?;
            return Ok(bin as u16);
        }
    }

    /// Assembles a label reference
    ///
    /// # Returns
    ///
    /// The address of the label, wrapped in a `Result`
    fn label(&mut self) -> Result<u16, AsmError> {
        //save the token
        let save_token = self.cur_token.clone();

        //parse the label
        self.eat(&TokenType::Label)?;

        //get the label string
        let lstr = save_token.get_value().as_text().unwrap();

        //and get the address
        return match self.addrs.get_entry(&lstr) {
            Ok(addr) => Ok(addr),
            Err(ae) => Err(AsmError::Address(ae))
        };
    }
    

    /// Assembles a register reference
    /// 
    /// # Returns
    ///
    /// The name of the assembled register, wrapped in a `Result`
    fn register(&mut self) -> Result<data::Register, AsmError> {
        //save the current token
        let save_token = self.cur_token.clone();

        //parse the register
        self.eat(&TokenType::Register)?;

        //get the text from the token
        let rtext = save_token.get_value().as_text().unwrap();

        //get the first and second chars of that text
        let fchar = rtext.chars().nth(0).unwrap();

        //and generate the register
        if fchar == 'I' {
            return Ok(data::Register::I);
        } else {
            let schar = rtext.chars().nth(1).unwrap();
            return match schar {
                '0' => Ok(data::Register::V0),
                '1' => Ok(data::Register::V1),
                '2' => Ok(data::Register::V2),
                '3' => Ok(data::Register::V3),
                '4' => Ok(data::Register::V4),
                '5' => Ok(data::Register::V5),
                '6' => Ok(data::Register::V6),
                '7' => Ok(data::Register::V7),
                '8' => Ok(data::Register::V8),
                '9' => Ok(data::Register::V9),
                'A' => Ok(data::Register::VA),
                'B' => Ok(data::Register::VB),
                'C' => Ok(data::Register::VC),
                'D' => Ok(data::Register::VD),
                'E' => Ok(data::Register::VE),
                'F' => Ok(data::Register::VF),
                _ => panic!("Bad register index {}", schar)
            };
        }
    }

    /// Assembles a skip condition
    /// 
    /// # Returns
    ///
    /// The assembled `SkipType`, wrapped in a `Result`
    fn skiptype(&mut self) -> Result<data::SkipType, AsmError> {
        //save the current token
        let save_token = self.cur_token.clone();

        //get the next token
        self.eat(&TokenType::SkipCond)?;

        //get the skip string
        let skstr = save_token.get_value().as_text().unwrap();

        //and generate the skip condition
        return match skstr.as_str() {
            "EQ" => Ok(data::SkipType::Equals),
            "NE" => Ok(data::SkipType::NotEquals),
            "KD" => Ok(data::SkipType::KeyDown),
            "KU" => Ok(data::SkipType::KeyUp),
            _ => panic!("Unknown skip type {}", skstr)
        };
    }

    /// Assembles a decimal integer literal
    ///
    /// # Returns
    ///
    /// The assembled literal, wrapped in a `Result`
    fn dec_lit(&mut self) -> Result<u16, AsmError> {
        //save the current token
        let save_token = self.cur_token.clone();

        //parse the literal
        self.eat(&TokenType::DecLit)?;

        //and return the token value
        return Ok(save_token.get_value().as_word().unwrap())
    }

    /// Assembles a hex integer literal
    ///
    /// # Returns
    ///
    /// The assembled literal, wrapped in a `Result`
    fn hex_lit(&mut self) -> Result<u16, AsmError> {
        //save the current token
        let save_token = self.cur_token.clone();

        //parse the literal
        self.eat(&TokenType::HexLit)?;

        //and return the token value
        return Ok(save_token.get_value().as_word().unwrap());
    }

    /// Assembles a binary literal
    ///
    /// # Returns
    ///
    /// The assembled literal, wrapped in a `Result`
    fn bin_lit(&mut self) -> Result<u8, AsmError> {
        //save the current token
        let save_token = self.cur_token.clone();

        //parse the literal
        self.eat(&TokenType::BinLit)?;

        //and return the token value
        return Ok(save_token.get_value().as_byte().unwrap());
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the Assembler struct
    use super::*;

    //define the program
    const CODE: &str = "CLS
                        MOV V0, #0
                        MOV V1, #15
                        KEY V2
                        SCH V2 
                        _loop:
                        DRAW V0, V1, #5
                        ADD V0, #1
                        JMP _loop";

    //this test checks code generation
    #[test]
    fn test_codegen() {
        let mut asm = Assembler::new(CODE, "test.c8").unwrap();
        let mut i = asm.instruction().unwrap();
        match i {
            Instruction::CLS => {},
            _ => panic!("Expected CLS")
        };
        i = asm.instruction().unwrap();
        match i {
            Instruction::MOV(_data) => {},
            _ => panic!("Expected MOV")
        };
        i = asm.instruction().unwrap();
        match i {
            Instruction::MOV(_data) => {},
            _ => panic!("Expected MOV")
        };
        i = asm.instruction().unwrap();
        match i {
           Instruction::KEY(_data) => {},
           _ => panic!("Expected KEY")
        };
        i = asm.instruction().unwrap();
        match i {
            Instruction::SCH(_data) => {},
            _ => panic!("Expected SCH")
        };
        asm.eat(&TokenType::LblDef).unwrap();
        i = asm.instruction().unwrap();
        match i {
            Instruction::DRAW(_data) => {},
            _ => panic!("Expected DRAW")
        };
        i = asm.instruction().unwrap();
        match i {
            Instruction::ADD(_data) => {},
            _ => panic!("Expected ADD")
        };
        i = asm.instruction().unwrap();
        match i {
            Instruction::JMP(_data) => {},
            _ => panic!("Expected JMP")
        };
    }
}

//end of file
